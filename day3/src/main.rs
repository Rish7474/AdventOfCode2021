use std::env;
use std::fs;
mod binary_diagnostic;

// Puzzle Link: https://adventofcode.com/2021/day/3
fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);
    let file_content = fs::read_to_string(&args[1]).unwrap();
    let binary_strs: Vec<&str> = file_content.lines().collect();
    let diagonstic1 = binary_diagnostic::diagonstic1(&binary_strs);
    println!("diagonstic 1 output: {}", diagonstic1);
    let diagonstic2 = binary_diagnostic::diagonstic2(&binary_strs);
    println!("diagonstic 2 output: {}", diagonstic2);
}
