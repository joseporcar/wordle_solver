use wordle_solver::perfect::PerfectWordSet;

fn main() {
    let mut words = PerfectWordSet::new();
    let mut count: i32 = 0;
    while let Ok(_) = words.update() {
        
        if words.is_perfect() {
            println!("{}", words.to_string())
        }
        println!("{}", words.to_string());
        count += 1;
        if count % 10000000 == 0 {
            println!("{}", words.to_string())
        }
        if count == 20 {
            break;
        }
    }
}


