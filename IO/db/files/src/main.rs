use peroxide::fuga::*;
use serde::{Serialize, Deserialize};

const I: usize = 100;
const J: usize = 500;
const ROW: usize = 500;
const COL: usize = 4;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
struct DBID {
    id: (usize, usize),
    m: f64,
}

impl DBID {
    fn new(id: (usize, usize), m: f64) -> Self {
        Self { id, m }
    }
}

fn write() -> Result<(), Box<dyn std::error::Error>> {
    // Remove "data/matrix/" if it exists
    let file = std::fs::File::open("data/matrix/");
    if file.is_ok() {
        std::fs::remove_dir_all("data/matrix/")?;
    }

    let db_path = std::path::Path::new("data/matrix/");
    std::fs::create_dir(db_path)?;

    let u = Uniform(0.0, 1.0);
    let mut m_vec = u.sample(I * J);
    m_vec[9301] = 0.1;

    for i in 0 .. I {
        let run_dir = format!("data/matrix/run_{:03}", i);
        for j in 0 .. J {
            let trial_dir = format!("{}/trial_{:04}", run_dir, j);
            std::fs::create_dir_all(&trial_dir)?;

            let m = m_vec[i * J + j];
            let matrix = rand(ROW, COL);

            let data = matrix.data.clone();
            let row = matrix.row;
            let col = matrix.col;
            let mut df = DataFrame::new(vec![]);
            df.push("m", Series::new(vec![m]));
            df.push("data", Series::new(data));
            df.push("row", Series::new(vec![row]));
            df.push("col", Series::new(vec![col]));

            let file_name = format!("{}/data.nc", trial_dir);
            df.write_nc(&file_name)?;
        }
    }

    Ok(())
}

fn update() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = std::path::Path::new("data/matrix/");

    for run_num in 0 .. I {
        for trial_num in 0 .. J {
            let file_name = format!("{}/run_{:03}/trial_{:04}/data.nc", db_path.to_str().unwrap(), run_num, trial_num);
            let df = DataFrame::read_nc(&file_name)?;
            let m: f64 = df["m"].at_raw(0);
            if m != 0.1 {
                continue;
            }
            let dbid = DBID::new((run_num, trial_num), m);
            let data: Vec<f64> = df["data"].to_vec();
            let row: u64 = df["row"].at_raw(0);
            let col: u64 = df["col"].at_raw(0);
            let row = row as usize;
            let col = col as usize;

            let matrix = matrix(data, row, col, Row);

            println!("id: {:?}, m: {:.4}", dbid.id, dbid.m);
            matrix.row(0).print();

            let matrix = zeros(ROW, COL);

            println!("id: {:?}, m: {:.4}", dbid.id, dbid.m);
            matrix.row(0).print();

            let data = matrix.data.clone();
            let row = matrix.row;
            let col = matrix.col;

            let mut df = DataFrame::new(vec![]);
            df.push("m", Series::new(vec![m]));
            df.push("data", Series::new(data));
            df.push("row", Series::new(vec![row]));
            df.push("col", Series::new(vec![col]));

            df.write_nc(&file_name)?;
        }
    }

    Ok(())
}

fn read() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = std::path::Path::new("data/matrix/");

    for run_num in 0 .. I {
        for trial_num in 0 .. J {
            let file_name = format!("{}/run_{:03}/trial_{:04}/data.nc", db_path.to_str().unwrap(), run_num, trial_num);
            let df = DataFrame::read_nc(&file_name)?;

            let m: f64 = df["m"].at_raw(0);
            if m != 0.1 {
                continue;
            }

            let dbid = DBID::new((run_num, trial_num), m);

            let data: Vec<f64> = df["data"].to_vec();
            let row: u64 = df["row"].at_raw(0);
            let col: u64 = df["col"].at_raw(0);
            let row = row as usize;
            let col = col as usize;

            let matrix = matrix(data, row, col, Row);

            println!("id: {:?}, m: {:.4}", dbid.id, dbid.m);
            matrix.row(0).print();
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let read_or_update_or_write = std::env::args().nth(1).unwrap();

    match read_or_update_or_write.as_str() {
        "write" => write()?,
        "update" => update()?,
        "read" => read()?,
        _ => println!("unknown command"),
    }

    Ok(())
}
