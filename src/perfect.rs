use crate::OPENER_COUNT;
const WORDS: [&[u8; 5]; crate::UNIQUE_WORDS.len()] = crate::UNIQUE_WORDS;

pub struct PerfectWordSet {
    indexes: [usize; OPENER_COUNT],
    buff: [u8; OPENER_COUNT*5],
    count: usize
}
impl PerfectWordSet  {
    pub fn new() -> Self {
        PerfectWordSet {
            indexes: (0..OPENER_COUNT).collect::<Vec<usize>>().try_into().unwrap(),
            buff: [0; OPENER_COUNT*5],
            count: 0
        }
    }

    // 0: abeng
    // 452: bachs
    // 1121: caber
    // 1795: daces
    // 2283: eagly
    // 2434: fable
    // 2911: gable
    // 3384: habit
    // 3721: iambs
    // 3827: jabot
    // 3972: kades
    // 4224: labis
    // 4644: mabes
    // 5250: nabes
    // 5509: oaked
    // 5745: paced
    // 6491: qadis
    // 6576: rabic
    // 7069: sabed
    // 7956: taber
    // 8491: ubacs
    // 8659: vacui
    // 8855: wacke
    // 9159: xenia
    // 9168: yacht
    // 9296: zacks
    // returns the index corresponding to the next beginning letter. Example if the input is abing, this would skip past abeys and the rest of as straight to bachs
    // Method will only work for hard_coded words Uniques
    fn next_letter(index: usize) -> usize {
        // The numbers were fetched from the _find_indexes_of_beginning_char()
        match index {
            0..452 => 452,
            452..1121 => 1121,
            1121..1795 => 1795,
            1795..2283 => 2283,
            2283.. => 0,
        }
    }

    fn update_index_no_first_letter_repeats(&mut self) -> Result<(), ()> {
        let mut i = OPENER_COUNT - 1;
        
        // Check if the index is going to overflow to change the previous digit. 
        while i > 0 && self.indexes[i] + 1 > WORDS.len() - OPENER_COUNT + i {
            i -= 1;
        }

        self.indexes[i] += 1;

        // Resetting indexes after the.. idk how to explain its like going from 99 to 100, this puts the zeroes
        while i < OPENER_COUNT - 1 {
            i += 1;
            self.indexes[i] = Self::next_letter(self.indexes[i-1]);
        }
        if self.indexes[0] < WORDS.len() - OPENER_COUNT + i - 1 {
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