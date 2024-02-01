use peroxide::fuga::*;
use serde::{Serialize, Deserialize};
use regex::Regex;

const ROW: usize = 500;
const COL: usize = 4;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
struct DBParam {
    m_pbh: f64,
    m_a: f64,
}

impl DBParam {
    fn new(m_pbh: f64, m_a: f64) -> Self {
        Self { m_pbh, m_a }
    }

    fn folder_path(&self) -> String {
        format!("data/matrix/m_pbh_{}/m_a_{}", self.m_pbh.to_bits(), self.m_a.to_bits())
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

    let m_pbh_vec = (1 .. 100).map(|x| (x as f64) * 1e+15).collect::<Vec<_>>();
    let m_a_vec = seq_with_precision(1e-3, 1e-1, 1e-3, 3);


    for &m_pbh in m_pbh_vec.iter() {
        for &m_a in m_a_vec.iter() {
            let db_param = DBParam::new(m_pbh, m_a);
            let trial_dir = db_param.folder_path();
            std::fs::create_dir_all(&trial_dir)?;

            let matrix = rand(ROW, COL);

            let data = matrix.data.clone();
            let row = matrix.row as u64;
            let col = matrix.col as u64;
            let mut df = DataFrame::new(vec![]);
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
    let target_m_a = 9e-2;
    for run_dir in std::fs::read_dir(db_path)? {
        let run_dir = run_dir?;
        let run_dir_path = run_dir.path();
        let run_dir_str = run_dir_path.to_str().unwrap();
        let re = Regex::new(r"m_pbh_(?P<m_pbh>\d+)")?;
        let m_pbh_u64 = re.captures(run_dir_str).unwrap()["m_pbh"].parse::<u64>()?;
        let m_pbh = f64::from_bits(m_pbh_u64);

        for trial_dir in std::fs::read_dir(run_dir_path)? {
            let trial_dir = trial_dir?;
            let trial_dir_path = trial_dir.path();
            let trial_dir_str = trial_dir_path.to_str().unwrap();
            let re = Regex::new(r"m_a_(?P<m_a>\d+)")?;
            let m_a_u64 = re.captures(trial_dir_str).unwrap()["m_a"].parse::<u64>()?;
            let m_a = f64::from_bits(m_a_u64);

            if m_a == target_m_a {
                let file_name = format!("{}/data.nc", trial_dir_str);
                let mut df = DataFrame::read_nc(&file_name)?;
                let data: Vec<f64> = df["data"].to_vec();
                let row: u64 = df["row"].at_raw(0);
                let col: u64 = df["col"].at_raw(0);
                let matrix = matrix(data, row as usize, col as usize, Row);
                println!("m_pbh: {:.2e}, m_a: {:.2e}", m_pbh, target_m_a);
                println!("{:?}", matrix.row(0));

                let new_matrix = zeros(ROW, COL);
                df["data"] = Series::new(new_matrix.data);
                df.write_nc(&file_name)?;
            }
        }
    }

    Ok(())
}

fn read() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = std::path::Path::new("data/matrix/");

    let target_m_a = 9e-2;
    for run_dir in std::fs::read_dir(db_path)? {
        let run_dir = run_dir?;
        let run_dir_path = run_dir.path();
        let run_dir_str = run_dir_path.to_str().unwrap();
        let re = Regex::new(r"m_pbh_(?P<m_pbh>\d+)")?;
        let m_pbh_u64 = re.captures(run_dir_str).unwrap()["m_pbh"].parse::<u64>()?;
        let m_pbh = f64::from_bits(m_pbh_u64);

        for trial_dir in std::fs::read_dir(run_dir_path)? {
            let trial_dir = trial_dir?;
            let trial_dir_path = trial_dir.path();
            let trial_dir_str = trial_dir_path.to_str().unwrap();
            let re = Regex::new(r"m_a_(?P<m_a>\d+)")?;
            let m_a_u64 = re.captures(trial_dir_str).unwrap()["m_a"].parse::<u64>()?;
            let m_a = f64::from_bits(m_a_u64);

            if m_a == target_m_a {
                let file_name = format!("{}/data.nc", trial_dir_str);
                let df = DataFrame::read_nc(&file_name)?;
                let data: Vec<f64> = df["data"].to_vec();
                let row: u64 = df["row"].at_raw(0);
                let col: u64 = df["col"].at_raw(0);
                let matrix = matrix(data, row as usize, col as usize, Row);
                println!("m_pbh: {:.2e}, m_a: {:.2e}", m_pbh, target_m_a);
                println!("{:?}", matrix.row(0));
            }
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
