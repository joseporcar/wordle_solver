
use wordle_solver::WordSet;


fn main() {
    let mut big = WordSet::new();
    let mut count: i32 = 0;
    while let Ok(_) = big.update() {
        
        if big.is_perfectly_unique() {
            println!("{}", big.to_string())
        }
        count += 1;
        if count % 10000000 == 0 {
            println!("{}", big.to_string())
        }
    }
}


