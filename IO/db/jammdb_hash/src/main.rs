use peroxide::fuga::*;
use jammdb::DB;
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use std::io::{BufReader, BufWriter};
use bincode::{serialize_into, deserialize_from};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
struct DBParam {
    m_pbh: f64,
    m_a: f64,
}

impl DBParam {
    fn new(m_pbh: f64, m_a: f64) -> Self {
        Self { m_pbh, m_a }
    }

    fn serialize_(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let data = rmp_serde::to_vec(self)?;
        Ok(data)
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
        let file = std::fs::File::create("data/jamm_param_map.bin")?;
        let mut writer = BufWriter::new(file);
        serialize_into(&mut writer, self)?;

        Ok(())
    }

    fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open("data/jamm_param_map.bin")?;
        let reader = BufReader::new(file);
        let param_map = deserialize_from(reader)?;

        Ok(param_map)
    }
}

fn write() -> Result<(), Box<dyn std::error::Error>> {
    // Remove "data/jammhash.db" if exists
    let file = std::fs::File::open("data/jammhash.db");
    if file.is_ok() {
        std::fs::remove_file("data/jammhash.db")?;
    }

    let db = DB::open("data/jammhash.db")?;
    let tx = db.tx(true)?;
    let bucket = tx.create_bucket("data")?;

    let m_pbh_vec = (1 .. 100).map(|x| (x as f64) * 1e+15).collect::<Vec<_>>();
    let m_a_vec = seq_with_precision(1e-3, 1e-1, 1e-3, 3);

    let mut param_map = ParamMap::new();

    for &m_pbh in &m_pbh_vec {
        for &m_a in &m_a_vec {
            let matrix = rand(500, 4);
            let db_param = rmp_serde::to_vec(&DBParam::new(m_pbh, m_a))?;
            let db_matrix = rmp_serde::to_vec(&matrix)?;

            param_map.insert(m_pbh, m_a);
            bucket.put(db_param, db_matrix)?;
        }
    }
    tx.commit()?;

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
            let db = DB::open("data/jammhash.db")?;
            let tx = db.tx(true)?;
            let bucket = tx.get_bucket("data")?;

            for m_pbh in m_pbh_vec {
                let db_param = DBParam::new(m_pbh, target_m_a);
                if let Some(old_data) = bucket.get(db_param.serialize_()?) {
                    let value = old_data.kv().value();
                    let matrix: Matrix = rmp_serde::from_slice(value)?;
                    println!("m_pbh: {:.2e}, m_a: {:.2e}", m_pbh, target_m_a);
                    println!("{:?}", matrix.row(0));

                    let new_matrix = zeros(500, 4);
                    bucket.put(db_param.serialize_()?, rmp_serde::to_vec(&new_matrix)?)?;
                }
            }
            tx.commit()?;
        }
    }

    Ok(())
}

fn read() -> Result<(), Box<dyn std::error::Error>> {
    let param_map = ParamMap::load()?;
    let target_m_a = 9e-2;
    let m_pbh_vec = param_map.get_m_pbh(target_m_a).unwrap();

    let db = DB::open("data/jammhash.db")?;
    let tx = db.tx(true)?;
    let bucket = tx.get_bucket("data")?;

    for m_pbh in m_pbh_vec {
        let db_param = DBParam::new(m_pbh, target_m_a);
        if let Some(data) = bucket.get(db_param.serialize_()?) {
            let value = data.kv().value();
            let matrix: Matrix = rmp_serde::from_slice(value)?;
            println!("m_pbh: {:.2e}, m_a: {:.2e}", m_pbh, target_m_a);
            println!("{:?}", matrix.row(0));
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
        _ => (),
    }

    Ok(())
}
