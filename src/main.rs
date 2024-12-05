use wordle_solver::perfect::PerfectWordSet;

fn main() {
    let mut words = PerfectWordSet::new();
    // let mut count: i32 = 0;
    while let Ok(_) = words.update() {
        
        if words.is_perfect() {
            println!("{}", words.to_string())
        }
        // count += 1;
        // if count % 100000000 == 0 {
        //     println!("{}", words.to_string())
        // }
    }
}


