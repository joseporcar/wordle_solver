use crate::OPENER_COUNT;
const WORDS: [&[u8; 5]; crate::UNIQUE_WORDS.len()] = crate::UNIQUE_WORDS;

pub struct PerfectWordSet {
    indexes: [usize; OPENER_COUNT],
    // This is defined on object level for optimization reasons
    buff: [u8; OPENER_COUNT*5],
    count: usize
}
impl PerfectWordSet  {

    pub fn new() -> Self {
        // The following is to put it in proper perfect format
        let mut indexes = [0; OPENER_COUNT];
        let mut i = 0;
        while i < OPENER_COUNT - 1 {
            i += 1;
            indexes[i] = Self::next_letter(indexes[i-1]);
        }
        PerfectWordSet {
            indexes: indexes,
            buff: [0; OPENER_COUNT*5],
            count: 0
        }
    }
    // returns the index corresponding to the next beginning letter. Example if the input is abing, this would skip past abeys and the rest of as straight to bachs
    // Method will only work for hard_coded words Uniques
    fn next_letter(index: usize) -> usize {
        // The numbers were fetched from the _find_indexes_of_beginning_char()
        match index {
            0..452 => 452,
            452..1121 => 1121,
            1121..1795 => 1795,
            1795..2283 => 2283,
            2283..2434 => 2434,
            2434..2911 => 2911,
            2911..3384 => 3384,
            3384..3721 => 3721,
            3721..3827 => 3827,
            3827..3972 => 3972,
            3972..4224 => 4224,
            4224..4644 => 4644,
            4644..5250 => 5250,
            5250..5509 => 5509,
            5509..5745 => 5745,
            5745..6491 => 6491,
            6491..6576 => 6576,
            6576..7069 => 7069,
            7069..7956 => 7956,
            7956..8491 => 8491,
            8491..8659 => 8659,
            8659..8855 => 8855,
            8855..9159 => 9159,
            9159..9168 => 9168,
            9168..9296 => 9296,
            9296..     => WORDS.len(),
        }
    }

    fn update_index_no_first_letter_repeats(&mut self) -> Result<(), ()> {
        self.count = OPENER_COUNT - 1;
        
        // Check if the index is going to overflow to change the previous digit. 
        while self.count > 0 && self.indexes[self.count] + 1 > WORDS.len() - OPENER_COUNT + self.count {
            self.count -= 1;
        }

        self.indexes[self.count] += 1;

        // Resetting indexes after the.. idk how to explain its like going from 99 to 100, this puts the zeroes
        while self.count < OPENER_COUNT - 1 {
            self.count += 1;
            self.indexes[self.count] = Self::next_letter(self.indexes[self.count-1]);
        }
        if self.indexes[0] < WORDS.len() - OPENER_COUNT + self.count - 1 {
            Ok(())
        }
        else {Err(())}
    }

    pub fn update(&mut self) -> Result<(), ()> {
        self.update_index_no_first_letter_repeats()?;
        Ok(())
    }

    // like the uniques function but does an early return as soon as there is a repeated letter
    pub fn is_perfect(&mut self) -> bool {
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
impl ToString for PerfectWordSet {
    fn to_string(&self) -> String {
        self.indexes.map(|i| WORDS[i]).map(|&x| String::from_utf8(x.into()).unwrap()).join(" ")
    }
}