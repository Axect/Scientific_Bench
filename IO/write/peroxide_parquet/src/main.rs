use peroxide::fuga::*;

fn main() {
    let x = linspace(0, 1, 10_000_000);
    let y = x.fmap(|x| x.powi(2));

    let mut df = DataFrame::new(vec![]);
    df.push("x", Series::new(x));
    df.push("y", Series::new(y));

    df.write_parquet("data/data.parquet").unwrap();
}
