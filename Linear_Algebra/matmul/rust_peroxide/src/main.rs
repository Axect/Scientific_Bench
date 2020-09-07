extern crate peroxide;
use peroxide::fuga::*;

const ROW: usize = 100;
const COL: usize = 100;

fn main() {
    // Create Matrix
    let m = rand(ROW, COL);
    
    // Copy Matrix
    let n = rand(ROW, COL);
    
    // Matmul
    let result = m * n;

    result[(ROW/2, COL/2)].print();
}
