extern crate peroxide;
use peroxide::fuga::*;

const ROW: usize = 100;
const COL: usize = 100;

fn main() {
    // Create Matrix
    let m = Matrix::from_index(|i, j| (i * ROW + j) as f64, (ROW, COL));
    
    // Copy Matrix
    let n = m.clone();
    
    // Matmul
    let result = m * n;

    result[(ROW/2, COL/2)].print();
}
