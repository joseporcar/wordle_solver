
// TODO 
// make enum for Unique or not
// make special update methods for unique or not
// the uniques will have special checks for first letter or so before doing the sets
// changing to not only pass around indexes instead of the words

use std::io::{Read, Write};
use std::fs::File;
mod hard_coded_words;
mod hard_coded_words_uniques;
pub mod perfect;
pub mod regular;
use hard_coded_words_uniques::UNIQUE_WORDS;
use hard_coded_words::WORDS;


// ideally, this should be provided from main or at runtime, however, for performance issues this is best
// as many performance features require this value to be compiled
const OPENER_COUNT: usize = 4; // How many openers to spew
const THRESHHOLD: u8 = 20;


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
// This one doesn't work anymore... no need to fix but just in case I will later, fixing is better than making from start
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

// This method is to print all of the indexes of letter changes in owrds (from airplane and airbus to base and bus)
pub fn _find_indexes_of_beginning_char() {
    let mut abc = "abcdefghijklmnopqrstuvwxyza".chars();
    let mut index = 0; 
    let mut letter = abc.next().unwrap();
    for word in WORDS { 
        if char::from_u32(word[0] as u32).unwrap() == letter { 
            letter = abc.next().unwrap();
            println!("{index}: {}", String::from_utf8(WORDS[index].to_vec()).unwrap());
        }
        index += 1;
    }
}