use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let p: u32 = args[1].parse().unwrap();
    let end = 10i64.pow(p);
    let x: Vec<f64> = (0i64 .. end).map(|t| t as f64).collect();
    let s: f64 = x.chunks_exact(4).map(|t| t.iter().sum::<f64>()).sum();
    println!("{}", s);
}
