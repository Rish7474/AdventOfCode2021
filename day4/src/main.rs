use std::env;
use std::fs;
mod bingo_eval;

// Puzzle Link: https://adventofcode.com/2021/day/4
fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);
    let file_content = fs::read_to_string(&args[1]).unwrap();
    let file_content: Vec<&str> = file_content
        .lines()
        .filter(|line| !line.is_empty())
        .collect();
    println!(
        "Bingo match score of first board: {}",
        bingo_eval::evaluate_bingo_match(&file_content, 1)
    );
    println!(
        "Bingo match score of last board: {}",
        bingo_eval::evaluate_bingo_match(&file_content, -1)
    );
}
