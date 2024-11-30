use std::collections::HashSet;
use std::io::{Read, Write};
use std::fs::File;
mod hard_coded_words;
use hard_coded_words::WORDS;
mod hard_coded_words_uniques;
use hard_coded_words_uniques::UNIQUE_WORDS;

const OPENER_COUNT: usize = 4; // How many openers to spew
const THRESHHOLD: u8 = 20;

// TODO 
// make enum for Unique or not
// make special update methods for unique or not
// the uniques will have special checks for first letter or so before doing the sets
// changing to not only pass around indexes instead of the words

pub struct WordSet {
    indexes: [usize; OPENER_COUNT],
    hash: HashSet<u8>
}

impl WordSet  {
    pub fn new() -> Self {
        WordSet {
            indexes: (0..OPENER_COUNT).collect::<Vec<usize>>().try_into().unwrap(),
            hash: HashSet::new(),
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

    pub fn update(&mut self) -> Result<(), ()> {
        self.update_index()?;
        Ok(())
    }

    // How many unique letters does a set have
    fn uniques(&mut self) -> u8 {
        self.hash.clear();
        for i in self.indexes {
            for c in UNIQUE_WORDS[i] {
                self.hash.insert(*c);
            }
        }
        self.hash.len() as u8
    }

    pub fn passes_threshold(&mut self) -> bool {
        self.uniques() >= THRESHHOLD
    }

}

impl ToString for WordSet {
    fn to_string(&self) -> String {
        self.indexes.map(|i| UNIQUE_WORDS[i]).map(|&x| String::from_utf8(x.into()).unwrap()).join(" ")
    }
}


// The following should never be called.  writes the words to hard_coded file.. requires additional work
// pub fn _hardcodewords() {
//     use std::fs::OpenOptions;
//     let mut buff = "pub const WORDS: [&[u8;5]; 14855] = [".to_owned();
//     let mut file = File::open("data/words.txt").unwrap();
//     let mut target = OpenOptions::new().write(true).open("src/hard_coded_words.rs").unwrap();
//     file.read_to_string(&mut buff).unwrap();
//     for word in buff.split_ascii_whitespace() {
//         target.write(format!("b\"{word}\",").as_bytes()).unwrap();
//     };
// }
// pub fn _hardcodewords_uniques() {
//     use std::fs::OpenOptions;
//     let mut buff = String::new();
//     let mut file = File::open("data/words.txt").unwrap();
//     let mut target = OpenOptions::new().write(true).open("src/hard_coded_words_uniques.rs").unwrap();
//     target.write(b"pub const UNIQUE_WORDS: [&[u8;5]; 14855] = [").unwrap();
//     file.read_to_string(&mut buff).unwrap();
//     let mut hash = HashSet::new();
//     for word in buff.split_ascii_whitespace().filter(|&word| uniques(word.as_bytes().repeat(3)[0..15].try_into().unwrap(), &mut hash) == 5) {
//         target.write(format!("b\"{word}\",").as_bytes()).unwrap();
//     };
//     target.write(b"];").unwrap();
// }