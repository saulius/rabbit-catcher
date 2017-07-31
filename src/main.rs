#![feature(const_fn)]

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

const WORDLIST_FILE: &'static str = "wordlist";
const TARGET_WORDS: &'static [&'static str] = &["poultry", "outwits", "ants"];

fn read_words(f: &'static str) -> Result<(Vec<Word>), io::Error> {
    let f = File::open(f)?;
    let f = BufReader::new(f);

    let target_word = Word::from_string(TARGET_WORDS.clone().join(""));

    // filter out words that have chars which are not present among TARGET_WORDS
    let mut valid_words: Vec<Word> = f
        .lines()
        .map(|word| word.unwrap())
        .filter(|word| word.chars().all(|c| c.is_alphabetic() && c.is_ascii()))
        .map(Word::from_string)
        // words of a single letter are unlikely to appear
        .filter(|word| word.len() > 1 && target_word.is_superset_of(word))
        .collect();

    // filter out words that have superset-words in the list
    // e.g. filter out 'upstart' as there is 'upstarts' in the list
    valid_words.sort_by(|w1, w2| w2.len().cmp(&w1.len()));
    valid_words.dedup();

    Ok(valid_words)
}

fn main() {
    let words = read_words(WORDLIST_FILE).unwrap();

    let now = Instant::now();

    let mut expected = vec!["e4820b45d2277f3844eac66c903e84be",
                            "23170acc097c24edb98fc5488ab033fe",
                            "665e5bcb0c20062fe8abaaf4628bb154"];

    let target_word = Word::from_string(TARGET_WORDS.clone().join(""));

    let mut hasher = Md5::new();

    let combinations = words.iter()
        .combinations(3)
        .filter(|c| Word::from_word_slice(&c).is_same(&target_word));

    for combination in combinations {
        let mut c = combination.clone();
        let permutator = Heap::new(&mut c);

        for permutation in permutator {
            let final_phrase: String = permutation.iter().map(|p| p.value.clone()).collect::<Vec<String>>().join(" ");

            hasher.input(final_phrase.as_bytes());
            let res = hasher.result_str();

            if expected.contains(&res.as_str()) {
                println!("Found anagram: {:?} - {:?} (in {} seconds)", final_phrase, res, now.elapsed().as_secs());

                // No longer need to check with the hash we just found
                expected.retain(|&hash| hash != res);
            }

            hasher.reset();
        }
    }
}
