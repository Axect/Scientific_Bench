use peroxide::fuga::*;
use serde::{Serialize, Deserialize};
use fjall::{Config, PartitionCreateOptions};
use std::path::Path;

const I: usize = 100;
const J: usize = 500;
const ROW: usize = 500;
const COL: usize = 4;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
struct DBID {
    id: (usize, usize),
    m: f64,
}

impl DBID {
    fn new(id: (usize, usize), m: f64) -> Self {
        Self { id, m }
    }

    fn id(&self) -> (usize, usize) {
        self.id
    }

    fn m(&self) -> f64 {
        self.m
    }
}

fn write() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open("data/fjall.db");
    if file.is_ok() {
        std::fs::remove_dir_all("data/fjall.db")?;
    }
    let path = Path::new("data/fjall.db");

    let keyspace = Config::new(path).open()?;

    let u = Uniform(0.0, 1.0);
    let mut m_vec = u.sample(I * J);
    m_vec[9301] = 0.1;
    for i in 0 .. I {
        let items = keyspace.open_partition(&format!("data_{:03}", i), PartitionCreateOptions::default())?;
        let mut batch = keyspace.batch();
        for j in 0 .. J {
            let m = m_vec[i * J + j];
            let matrix = rand(ROW, COL);
            let dbid = DBID::new((i, j), m);

            batch.insert(&items, rmp_serde::to_vec(&dbid)?, rmp_serde::to_vec(&matrix)?);
        }
        batch.commit()?;
    }
    keyspace.persist()?;

    Ok(())
}

fn update() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("data/fjall.db");
    let keyspace = Config::new(path).open()?;
    let mut target_item = 0usize;
    let mut key_to_update = Vec::new();

    for i in 0 .. I {
        let items = keyspace.open_partition(&format!("data_{:03}", i), PartitionCreateOptions::default())?;
        for item in items.iter().into_iter().flatten() {
            let (key, value) = item;
            let dbid: DBID = rmp_serde::from_slice(&key)?;
            if dbid.m() == 0.1 {
                println!("id: {:?}, m: {:.4}", dbid.id(), dbid.m());
                let matrix: Matrix = rmp_serde::from_slice(&value)?;
                matrix.row(0).print();
                target_item = i;
                key_to_update.push(key);
                break;
            }
        }
    }

    let items = keyspace.open_partition(&format!("data_{:03}", target_item), PartitionCreateOptions::default())?;
    for key in key_to_update {
        let dbid: DBID = rmp_serde::from_slice(&key)?;
        let matrix = rand(ROW, COL);
        println!("id: {:?}, m: {:.4}", dbid.id(), dbid.m());
        matrix.row(0).print();
        items.insert(key, rmp_serde::to_vec(&matrix)?)?;
    }

    keyspace.persist()?;

    Ok(())
}

fn read() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("data/fjall.db");
    let keyspace = Config::new(path).open()?;

    for i in 0 .. I {
        let items = keyspace.open_partition(&format!("data_{:03}", i), PartitionCreateOptions::default())?;
        for item in items.iter().into_iter().flatten() {
            let (key, value) = item;
            let dbid: DBID = rmp_serde::from_slice(&key)?;
            if dbid.m() == 0.1 {
                println!("id: {:?}, m: {:.4}", dbid.id(), dbid.m());
                let matrix: Matrix = rmp_serde::from_slice(&value)?;
                matrix.row(0).print();
                break;
            }
        }
    }

    Ok(())
}

fn main() {
    let read_or_update_or_write = std::env::args().nth(1).unwrap();

    match read_or_update_or_write.as_str() {
        "write" => write().unwrap(),
        "update" => update().unwrap(),
        "read" => read().unwrap(),
        _ => (),
    }
}
