extern crate peroxide;
use peroxide::fuga::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let row = args[1].parse().unwrap();
    let col = args[2].parse().unwrap();

    // Create Matrix
    let m = rand(row, col);
    
    // Copy Matrix
    let n = rand(row, col);
    
    // Matmul
    let result = m * n;

    result[(row/2, col/2)].print();
}
