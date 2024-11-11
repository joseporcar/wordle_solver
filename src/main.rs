
use std::collections::HashSet;

use wordle_solver::{passes_threshold, WordSet};


fn main() {
    let big = WordSet::new();
    let mut hash = HashSet::new();

    for set in big {
        if passes_threshold(set, &mut hash) {
            let a = set.split_at(5);
            let b = a.1.split_at(5);
            let a = [a.0, b.0, b.1].iter().map(|&x| String::from_utf8(x.into()).unwrap()).collect::<Vec<String>>().join(" ");
            println!("{:?}", a)
        }
    }
}


