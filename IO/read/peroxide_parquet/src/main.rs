use peroxide::fuga::*;

fn main() {
    let df = DataFrame::read_parquet("../write/data/data.parquet").unwrap();

    df.print();
}
