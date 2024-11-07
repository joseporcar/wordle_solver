use std::collections::HashSet;
use std::io::{Read, Write};
use std::fs::File;
mod hard_coded_words;
use hard_coded_words::WORDS;

const OPENER_COUNT: usize = 3; // How many openers to spew
const THRESHHOLD: u8 = 13;

// Makes the sets of OPENER_COUNT words in length
fn word_sets() {
    for i in 0..(WORDS.len() - 2) {
        for j in 1..(WORDS.len() - 1) {
            for k in 2..WORDS.len() {
                
            }
        }
    }
}

fn passes_threshold(set: [u8; OPENER_COUNT * 5]) -> bool {
    todo!()
}

fn join_set(set: [[u8; 5]; OPENER_COUNT]) -> [u8; OPENER_COUNT * 5] {
    set.concat().try_into().unwrap()
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