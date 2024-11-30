use std::collections::HashSet;
use std::io::{Read, Write};
use std::fs::File;
mod hard_coded_words;
mod hard_coded_words_uniques;
use hard_coded_words_uniques::UNIQUE_WORDS;

// the set of words to use, change depending on purpose
const WORDS: [&[u8; 5]; UNIQUE_WORDS.len()] = UNIQUE_WORDS;

const OPENER_COUNT: usize = 4; // How many openers to spew
const THRESHHOLD: u8 = 20;

// TODO 
// make enum for Unique or not
// make special update methods for unique or not
// the uniques will have special checks for first letter or so before doing the sets
// changing to not only pass around indexes instead of the words

pub struct WordSet {
    indexes: [usize; OPENER_COUNT],
    hash: HashSet<u8>,
    buff: [u8; OPENER_COUNT*5],
    count: usize
}

impl WordSet  {
    pub fn new() -> Self {
        WordSet {
            indexes: (0..OPENER_COUNT).collect::<Vec<usize>>().try_into().unwrap(),
            hash: HashSet::new(),
            buff: [0; OPENER_COUNT*5],
            count: 0
        }
    }

    fn update_index(&mut self) -> Result<(),()> {
        let mut i = OPENER_COUNT - 1;

        while i > 0 && self.indexes[i] + 1 > WORDS.len() - OPENER_COUNT + i {
            i -= 1;
        }
        self.indexes[i] += 1;
        while i < OPENER_COUNT - 1 {
            i += 1;
            self.indexes[i] = self.indexes[i-1] + 1;
        }
        if self.indexes[0] < WORDS.len() - OPENER_COUNT + i - 1 {
            Ok(())
        }
        else {Err(())}
    }

    // fn update_index_no_first_letter_repeats(&mut self) -> Result<(), ()> {
    //     let mut i = OPENER_COUNT - 1;
        
    //     // Check if the index is going to overflow to change the previous digit. 
    //     while i > 0 && self.indexes[i] + 1 > WORDS.len() - OPENER_COUNT + i {
    //         i -= 1;
    //     }

    //     self.indexes[i] += 1;

    //     // Resetting indexes after the.. idk how to explain its like going from 99 to 100, this puts the zeroes
    //     while i < OPENER_COUNT - 1 {
    //         i += 1;
    //         self.indexes[i] = self.indexes[i-1] + 1;
    //     }
    //     if self.indexes[0] < WORDS.len() - OPENER_COUNT + i - 1 {
    //         Ok(())
    //     }
    //     else {Err(())}
    // }

    pub fn update(&mut self) -> Result<(), ()> {
        self.update_index()?;
        Ok(())
    }



    // How many unique letters does a set have
    fn uniques(&mut self) -> u8 {
        self.hash.clear();
        for i in self.indexes {
            for c in WORDS[i] {
                self.hash.insert(*c);
            }
        }
        self.hash.len() as u8
    }

    pub fn passes_threshold(&mut self) -> bool {
        self.uniques() >= THRESHHOLD
    }

    // // like the uniques function but does an early return as soon as there is a repeated letter
    // pub fn is_perfectly_unique(&mut self) -> bool {
    //     self.hash.clear();
    //     let mut count= 0; 
    //     for i in self.indexes {
    //         for c in WORDS[i] {
    //             self.hash.insert(*c);
    //             count += 1;
    //             if self.hash.len() != count {
    //                 return false 
    //             }
    //         }
    //     }
    //     true
    // }

    // like the uniques function but does an early return as soon as there is a repeated letter
    pub fn is_perfectly_unique(&mut self) -> bool {
        self.buff.fill(0);
        self.count = 0;
        for i in self.indexes {
            for c in WORDS[i] {
                if self.buff.contains(c) {
                    return false 
                }
                self.buff[self.count] = *c;
                self.count += 1;
            }
        }
        true
    }
}

impl ToString for WordSet {
    fn to_string(&self) -> String {
        self.indexes.map(|i| WORDS[i]).map(|&x| String::from_utf8(x.into()).unwrap()).join(" ")
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