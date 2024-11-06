use std::collections::{HashMap, HashSet};
use std::io::{Read, Write};
use std::fs::File;
mod hard_coded_words;
use hard_coded_words::WORDS;

const OPENER_COUNT: usize = 3; // How many openers to spew
fn main() {
    
    
}

// How many unique letters does a set have
fn uniques(word: &[u8; OPENER_COUNT * 5], hash: &mut HashSet<u8>) -> u8 {
    hash.clear();
    for c in word {
        hash.insert(*c);
    }
    hash.len() as u8
}


fn word_combos() {
    todo!()
}

// should never be called.  writes the words to hard_coded file.. requires additional work
fn _hardcodewords() {
    use std::fs::OpenOptions;
    let mut buff = String::new();
    let mut file = File::open("data/words.txt").unwrap();
    let mut target = OpenOptions::new().write(true).open("src/hard_coded_words.rs").unwrap();
    file.read_to_string(&mut buff).unwrap();
    for word in buff.split_ascii_whitespace() {
        target.write(format!("b\"{word}\",").as_bytes()).unwrap();
       //println!("{word}");
    };
}
