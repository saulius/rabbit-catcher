#[macro_use] extern crate lazy_static;
extern crate itertools;
extern crate permutohedron;
extern crate simd;
extern crate crypto;

mod word;

use std::ascii::AsciiExt;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::time::Instant;

use crypto::md5::Md5;
use crypto::digest::Digest;
use itertools::Itertools;
use permutohedron::Heap;

use word::Word;

lazy_static! {
    static ref WORDLIST_FILE: &'static str = "wordlist";

    static ref TARGET_WORDS: [String; 3] = ["poultry".to_owned(),
                                            "outwits".to_owned(),
                                            "ants".to_owned()];

    static ref TARGET_WORD: Word = Word::from_string(TARGET_WORDS.join(""));
}

fn read_words(f: &'static str) -> Result<(Vec<Word>), io::Error> {
    let f = File::open(f)?;
    let f = BufReader::new(f);

    // filter out words that have chars which are not present among TARGET_WORDS
    let mut valid_words: Vec<Word> = f
        .lines()
        .map(|word| word.unwrap())
        .filter(|word| word.chars().all(|c| c.is_alphabetic() && c.is_ascii()))
        .map(Word::from_string)
        // words of a single letter are unlikely to appear
        .filter(|word| word.len() > 1 && TARGET_WORD.is_superset_of(word))
        .collect();

    // filter out words that have superset-words in the list
    // e.g. filter out 'upstart' as there is 'upstarts' in the list
    valid_words.sort_by(|w1, w2| w2.len().cmp(&w1.len()));
    valid_words.dedup();

    Ok(valid_words)
}

fn main() {
    let words = read_words(&WORDLIST_FILE).unwrap();

    let mut expected = vec!["e4820b45d2277f3844eac66c903e84be",
                            "23170acc097c24edb98fc5488ab033fe",
                            "665e5bcb0c20062fe8abaaf4628bb154"];

    let mut hasher = Md5::new();

    let start = Instant::now();

    let combinations = words.iter()
        .combinations(3)
        .filter(|c| {
            c.iter().fold(TARGET_WORD.histo.clone(), |combined_histo, word|
                          combined_histo - word.histo.clone()
            ).value.eq(*word::ZERO_VEC).all()
        });

    for combination in combinations {
        let mut c = combination.clone();
        let permutator = Heap::new(&mut c);

        for permutation in permutator {
            let bytes: Vec<u8> = permutation.iter().map(|p| p.value.clone()).intersperse(" ".to_owned().into_bytes()).collect::<Vec<Vec<u8>>>().concat();

            hasher.input(&bytes);
            let res = hasher.result_str();

            if expected.contains(&res.as_str()) {
                let final_phrase: String = String::from_utf8(bytes).unwrap();
                let elapsed = start.elapsed();
                let elapsed_ms = (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64;

                println!("Found anagram: {:?} - {:?} (in {} ms)", final_phrase, res, elapsed_ms);

                // No longer need to check with the hash we just found
                expected.retain(|&hash| hash != res);
            }

            hasher.reset();
        }
    }
}
