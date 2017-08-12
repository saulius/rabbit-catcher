#[macro_use] extern crate lazy_static;
extern crate permutohedron;
extern crate simd;
extern crate crypto;
extern crate smallvec;

mod word;

use std::env;
use std::ascii::AsciiExt;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::time::Instant;
use std::iter::FromIterator;

use crypto::md5::Md5;
use crypto::digest::Digest;
use smallvec::SmallVec;
use permutohedron::heap_recursive;

use word::{Word, Histogram};

lazy_static! {
    static ref WORDLIST_FILE: &'static str = "wordlist";

    static ref TARGET_WORDS: [String; 3] = [
        "poultry".to_owned(),
        "outwits".to_owned(),
        "ants".to_owned()
    ];

    static ref TARGET_WORD: Word = Word::from_string(TARGET_WORDS.join(""));

    static ref EXPECTED_HASHES: [String; 3] = [
        "e4820b45d2277f3844eac66c903e84be".to_owned(),
        "23170acc097c24edb98fc5488ab033fe".to_owned(),
        "665e5bcb0c20062fe8abaaf4628bb154".to_owned()
    ];
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

fn scan(dictionary: &[Word],
        indices: SmallVec<[usize; 8]>,
        histo: Histogram,
        start: Instant,
        hasher: &mut Md5
  ) {
    let last_idx = indices.iter().last().unwrap();

    for i in last_idx + 1..dictionary.len() {
        let current_word = &dictionary[i];

        let diff = histo - current_word.histo;

        let is_subset = diff.value.ge(*word::ZERO_VEC).all();

        if !is_subset {
            continue;
        }

        let is_anagram = diff.value.eq(*word::ZERO_VEC).all();

        let mut new_indices = indices.clone();
        new_indices.push(i);

        if is_anagram {
            heap_recursive(&mut new_indices, |permutation| {
                let bytes: Vec<u8> = permutation
                    .iter()
                    .map(|idx| dictionary[*idx].value.as_slice())
                    .collect::<Vec<&[u8]>>()
                    .join(&b' ');
                hasher.input(&bytes);
                let res = hasher.result_str();

                if EXPECTED_HASHES.contains(&res) {
                    let final_phrase: String = String::from_utf8(bytes).unwrap();
                    let elapsed = start.elapsed();
                    let elapsed_ms = (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64;

                    println!("Found anagram: {:?} - {:?} (in {} ms)", final_phrase, res, elapsed_ms);
                }

                hasher.reset();
            });
        } else if is_subset {
            scan(dictionary, new_indices, diff, start, hasher)
        }
    }
}

fn main() {
    let mut parallelism = 1;

    if let Some(par) = env::args().nth(1) {
        parallelism = par.parse::<usize>().unwrap();
    }

    let words = read_words(&WORDLIST_FILE).unwrap();

    let start = Instant::now();

    let hasher = Md5::new();

    let num_tasks_per_thread = words.len() / parallelism;

    let mut threads = vec![];

    // Divide wordlist into chunks depending on desired thread count
    let wordlist = words
        .chunks(num_tasks_per_thread)
        .enumerate()
        .map(|(thread_num, chunk)| {
            chunk
                .iter()
                .enumerate()
                .map(|(i, word)| {
                    let index_offset = thread_num + 1;
                    let indices = SmallVec::<[usize; 8]>::from_iter(i * index_offset..(i+1) * index_offset);
                    let current_histo = TARGET_WORD.histo - word.histo;

                    (indices, current_histo)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for wl in wordlist {
        let ws = words.clone();

        threads.push(std::thread::spawn(move || {
            for (indices, current_histo) in wl {
                scan(&ws, indices, current_histo, start, &mut hasher.clone())
            }
        }))
    }

    for thread in threads {
        thread.join().unwrap()
    }
}
