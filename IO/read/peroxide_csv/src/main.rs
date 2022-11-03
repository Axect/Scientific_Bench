use peroxide::fuga::*;

fn main() {
    let df = DataFrame::read_csv("../write/data/data.csv", ',').unwrap();

    df.print();
}
