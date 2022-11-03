use peroxide::fuga::*;

fn main() {
    let df = DataFrame::read_nc("../write/data/data.nc").unwrap();

    df.print();
}
