use peroxide::fuga::*;
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use std::io::{BufReader, BufWriter};
use bincode::{serialize_into, deserialize_from};

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
        let m_pbh = self.m_pbh.to_bits();
        let m_a = self.m_a.to_bits();
        format!("data/matrix/m_pbh_{}/m_a_{}", m_pbh, m_a)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ParamMap {
    m_pbh_map: HashMap<u64, HashSet<u64>>,
    m_a_map: HashMap<u64, HashSet<u64>>,
}

impl ParamMap {
    fn new() -> Self {
        Self { m_pbh_map: HashMap::new(), m_a_map: HashMap::new() }
    }

    fn insert(&mut self, m_pbh: f64, m_a: f64) {
        let m_pbh = m_pbh.to_bits();
        let m_a = m_a.to_bits();

        self.m_pbh_map.entry(m_pbh).or_default().insert(m_a);
        self.m_a_map.entry(m_a).or_default().insert(m_pbh);
    }

    #[allow(dead_code)]
    fn get_m_a(&self, m_pbh: f64) -> Option<Vec<f64>> {
        let m_pbh = m_pbh.to_bits();
        let m_a = self.m_pbh_map.get(&m_pbh);
        m_a.map(|m_a| m_a.iter().map(|&m_a| f64::from_bits(m_a)).collect())
    }

    fn get_m_pbh(&self, m_a: f64) -> Option<Vec<f64>> {
        let m_a = m_a.to_bits();
        let m_pbh = self.m_a_map.get(&m_a);
        m_pbh.map(|m_pbh| m_pbh.iter().map(|&m_pbh| f64::from_bits(m_pbh)).collect())
    }

    fn write(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::File::create("data/files_param_map.bin")?;
        let mut writer = BufWriter::new(file);
        serialize_into(&mut writer, self)?;

        Ok(())
    }

    fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open("data/files_param_map.bin")?;
        let reader = BufReader::new(file);
        let param_map = deserialize_from(reader)?;

        Ok(param_map)
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

    let mut param_map = ParamMap::new();

    for &m_pbh in m_pbh_vec.iter() {
        for &m_a in m_a_vec.iter() {
            let db_param = DBParam::new(m_pbh, m_a);
            let trial_dir = db_param.folder_path();
            std::fs::create_dir_all(&trial_dir)?;

            param_map.insert(m_pbh, m_a);

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

    param_map.write()?;

    Ok(())
}

fn update() -> Result<(), Box<dyn std::error::Error>> {
    let param_map = ParamMap::load()?;

    let target_m_a = 9e-2;
    let m_pbh_vec = param_map.get_m_pbh(target_m_a);
    match m_pbh_vec {
        None => panic!("m_pbh not found"),
        Some(m_pbh_vec) => {
            for m_pbh in m_pbh_vec {
                let db_param = DBParam::new(m_pbh, target_m_a);
                let trial_dir = db_param.folder_path();

                let mut df = DataFrame::read_nc(&format!("{}/data.nc", trial_dir))?;
                let data: Vec<f64> = df["data"].to_vec();
                let row: u64 = df["row"].at_raw(0);
                let col: u64 = df["col"].at_raw(0);
                let matrix = matrix(data, row as usize, col as usize, Row);

                println!("m_pbh: {:.2e}, m_a: {:.2e}", m_pbh, target_m_a);
                matrix.row(0).print();

                let new_matrix = zeros(ROW, COL);
                df["data"] = Series::new(new_matrix.data);
                df.write_nc(&format!("{}/data.nc", trial_dir))?;
            }
        }
    }

    Ok(())
}

fn read() -> Result<(), Box<dyn std::error::Error>> {
    let param_map = ParamMap::load()?;
    let target_m_a = 9e-2;
    let m_pbh_vec = param_map.get_m_pbh(target_m_a);
    match m_pbh_vec {
        None => panic!("m_pbh not found"),
        Some(m_pbh_vec) => {
            for m_pbh in m_pbh_vec {
                let db_param = DBParam::new(m_pbh, target_m_a);
                let trial_dir = db_param.folder_path();
                let df = DataFrame::read_nc(&format!("{}/data.nc", trial_dir))?;
                let data: Vec<f64> = df["data"].to_vec();
                let row: u64 = df["row"].at_raw(0);
                let col: u64 = df["col"].at_raw(0);
                let matrix = matrix(data, row as usize, col as usize, Row);

                println!("m_pbh: {:.2e}, m_a: {:.2e}", m_pbh, target_m_a);
                matrix.row(0).print();
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
