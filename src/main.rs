use std::io::{Read, Write};

fn main() {
    let mut words: [&str; 14855] = [""; 14855];
    file_to_vec(&mut words);
}

fn file_to_vec(words: &mut [&str; 14855]) {
    use std::fs::File;
    use std::fs::OpenOptions;
    let mut buff = String::new();
    let mut file = File::open("data/words.txt").unwrap();
    let mut target = OpenOptions::new().write(true).open("data/hard_coded_words.rs").unwrap();
    file.read_to_string(&mut buff);
    for word in buff.split_ascii_whitespace() {
        target.write(format!("\"{word}\",").as_bytes()).unwrap();
       //println!("{word}");
    };

}
