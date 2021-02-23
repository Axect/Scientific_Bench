use std::env;
use packed_simd_2::f64x4;

fn main() {
    let args: Vec<String> = env::args().collect();
    let p: u32 = args[1].parse().unwrap();
    let end = 10i64.pow(p);
    let x: Vec<f64> = (0i64 .. end).map(|t| t as f64).collect();
    let s = fast_sum(&x);
    println!("{}", s);
}

fn fast_sum(x: &[f64]) -> f64 {
    assert_eq!(x.len() % 4, 0);
    let mut sum = f64x4::splat(0.);
    for i in (0 .. x.len()).step_by(4) {
        sum += f64x4::from_slice_unaligned(&x[i..]);
    }
    sum.sum()
}
