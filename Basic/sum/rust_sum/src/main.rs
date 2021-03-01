use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let p: usize = args[1].parse().unwrap();
    let end = 10i64.pow(p as u32);
    let x: Vec<f64> = (0 .. end).map(|t| t as f64).collect();
    let s = for_sum(&x);
    println!("{}", s);
}

fn for_sum(x: &[f64]) -> f64 {
    let mut s = 0f64;
    for t in x.iter() {
        s += *t;
    }
    s
}
