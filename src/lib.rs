use std::collections::HashSet;
use std::io::{Read, Write};
use std::fs::File;
mod hard_coded_words;
use hard_coded_words::WORDS;
mod hard_coded_words_uniques;
use hard_coded_words_uniques::UNIQUE_WORDS;

const OPENER_COUNT: usize = 3; // How many openers to spew
const THRESHHOLD: u8 = 15;

pub struct WordSet {
    indexes: [usize; OPENER_COUNT],
    parts: [&'static [u8; 5]; OPENER_COUNT],
    combined: [u8; OPENER_COUNT * 5]
}

impl WordSet  {
    pub fn new() -> Self {
        WordSet {
            indexes: (0..OPENER_COUNT).collect::<Vec<usize>>().try_into().unwrap(),
            parts: (0..OPENER_COUNT).map(|i| &UNIQUE_WORDS[i]).cloned().collect::<Vec<&[u8; 5]>>().try_into().unwrap(),
            combined: (0..OPENER_COUNT).map(|i| UNIQUE_WORDS[i]).cloned().collect::<Vec<[u8; 5]>>().concat().try_into().unwrap(),
        }
    }
    
    fn join_set(&mut self) {
        for p in 0..OPENER_COUNT {
            for i in 0..5 {
                self.combined[(p*5) + i ] = self.parts[p][i];
            }
        }
    }

    fn update_index(&mut self) -> Result<(),()> {
        let mut i = OPENER_COUNT - 1;

        while i > 0 && self.indexes[i] + 1 > UNIQUE_WORDS.len() - OPENER_COUNT + i {
            i -= 1;
        }
        self.indexes[i] += 1;
        while i < OPENER_COUNT - 1 {
            i += 1;
            self.indexes[i] = self.indexes[i-1] + 1;
        }
        if self.indexes[0] < UNIQUE_WORDS.len() - OPENER_COUNT + i - 1 {
            Ok(())
        }
        else {Err(())}
    }
    fn update_parts(&mut self) {
        for i in 0..OPENER_COUNT {
            if self.indexes[i] == 14855 {
                dbg!(self.indexes);
            }
            self.parts[i] = UNIQUE_WORDS[self.indexes[i]]
        }
    }
    fn update(&mut self) -> Result<(), ()> {
        self.update_index()?;
        self.update_parts();
        self.join_set();
        Ok(())
    }
}
impl <'a> Iterator for WordSet {
    type Item = [u8; OPENER_COUNT * 5];
    fn next(&mut self) -> Option<Self::Item> {
        self.update().ok()?;

        Some(self.combined)
    }
}

pub fn passes_threshold(set: [u8; OPENER_COUNT * 5], hash: &mut HashSet<u8>) -> bool {
    uniques(&set, hash) >= THRESHHOLD
}

fn lazy_uniques(hash: &mut HashSet<u8>) -> u8 {
    todo!()
}
// How many unique letters does a set have
fn uniques(word: &[u8; OPENER_COUNT * 5], hash: &mut HashSet<u8>) -> u8 {
    hash.clear();
    for c in word {
        hash.insert(*c);
    }
    hash.len() as u8
}

// The following should never be called.  writes the words to hard_coded file.. requires additional work
pub fn _hardcodewords() {
    use std::fs::OpenOptions;
    let mut buff = "pub const WORDS: [&[u8;5]; 14855] = [".to_owned();
    let mut file = File::open("data/words.txt").unwrap();
    let mut target = OpenOptions::new().write(true).open("src/hard_coded_words.rs").unwrap();
    file.read_to_string(&mut buff).unwrap();
    for word in buff.split_ascii_whitespace() {
        target.write(format!("b\"{word}\",").as_bytes()).unwrap();
    };
}
pub fn _hardcodewords_uniques() {
    use std::fs::OpenOptions;
    let mut buff = String::new();
    let mut file = File::open("data/words.txt").unwrap();
    let mut target = OpenOptions::new().write(true).open("src/hard_coded_words_uniques.rs").unwrap();
    target.write(b"pub const UNIQUE_WORDS: [&[u8;5]; 14855] = [").unwrap();
    file.read_to_string(&mut buff).unwrap();
    let mut hash = HashSet::new();
    for word in buff.split_ascii_whitespace().filter(|&word| uniques(word.as_bytes().repeat(3)[0..15].try_into().unwrap(), &mut hash) == 5) {
        target.write(format!("b\"{word}\",").as_bytes()).unwrap();
    };
    target.write(b"];").unwrap();
}