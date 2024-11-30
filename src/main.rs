
use wordle_solver::{WordSet};


fn main() {
    let mut big = WordSet::new();
    let mut count = 0;
    while let Ok(_) = big.update() {
        count += 1;
        if big.passes_threshold() {
            println!("{}", big.to_string())
        }
    }
}


