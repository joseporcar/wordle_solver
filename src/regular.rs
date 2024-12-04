use crate::{OPENER_COUNT, THRESHHOLD, WORDS};
use std::collections::HashSet;

pub struct WordSet {
    indexes: [usize; OPENER_COUNT],
    hash: HashSet<u8>,
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

        // Check if the index is going to overflow to change the previous digit. 
        while i > 0 && self.indexes[i] + 1 > WORDS.len() - OPENER_COUNT + i {
            i -= 1;
        }

        self.indexes[i] += 1;

        // Resetting indexes after the.. idk how to explain its like going from 99 to 100, this puts the zeroes
        while i < OPENER_COUNT - 1 {
            i += 1;
            self.indexes[i] = self.indexes[i-1] + 1;
        }
        if self.indexes[0] < WORDS.len() - OPENER_COUNT + i - 1 {
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
            for c in WORDS[i] {
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
        self.indexes.map(|i| WORDS[i]).map(|&x| String::from_utf8(x.into()).unwrap()).join(" ")
    }
}