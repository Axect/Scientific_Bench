use peroxide::fuga::*;
use serde::{Deserialize, Serialize};
use kv::*;

const I: usize = 100;
const J: usize = 500;
const ROW: usize = 500;
const COL: usize = 4;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
struct DBID {
    id: (usize, usize),
    m: f64,
}

trait WithRaw: Sized + Serialize + for<'a> Deserialize<'a> {
    fn to_raw(&self) -> Result<Raw, Box<dyn std::error::Error>> {
        Ok(Raw::from(rmp_serde::to_vec(self)?))
    }
    fn from_raw(raw: &Raw) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(rmp_serde::from_slice(raw)?)
    }
}

impl DBID {
    fn new(id: (usize, usize), m: f64) -> Self {
        Self {
            id,
            m,
        }
    }
}

impl WithRaw for DBID {}
impl WithRaw for Matrix {}

fn write() -> Result<(), Box<dyn std::error::Error>> {
    // Remove "data/jamm.db" if exists
    let file = std::fs::File::open("data/kv");
    if file.is_ok() {
        std::fs::remove_dir_all("data/kv")?;
    }

    let cfg = Config::new("data/kv");
    let store = Store::new(cfg)?;
    let bucket = store.bucket::<Raw, Raw>(Some("data"))?;

    let u = Uniform(0.0, 1.0);
    let mut m_vec = u.sample(I*J);
    m_vec[9301] = 0.1;
    for i in 0 .. I {
        for j in 0 .. J {
            let m = m_vec[i * J + j];
            let matrix = rand(ROW, COL);
            let dbid = DBID::new((i, j), m);

            let key = dbid.to_raw()?;
            let value = matrix.to_raw()?;

            bucket.set(&key, &value)?;
        }
    }

    Ok(())
}

fn update() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = Config::new("data/kv");
    let store = Store::new(cfg)?;
    let bucket = store.bucket::<Raw, Raw>(Some("data"))?;

    let kv_candidates = bucket.iter().filter_map(|data| {
        if let Ok(item) = data {
            match (item.key(), item.value()) {
                (Ok(key), Ok(value)) => {
                    let dbid = DBID::from_raw(&key).unwrap();
                    if dbid.m == 0.1 {
                        Some((key, value))
                    } else {
                        None
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    });

    for (key, value) in kv_candidates {
        let dbid = DBID::from_raw(&key).unwrap();
        let matrix = Matrix::from_raw(&value).unwrap();
        println!("id: {:?}, m: {:.4}", dbid.id, dbid.m);
        matrix.row(0).print();

        let new_matrix = zeros(ROW, COL);
        bucket.set(&key, &new_matrix.to_raw()?)?;
    }

    Ok(())
}

fn read() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = Config::new("data/kv");
    let store = Store::new(cfg)?;
    let bucket = store.bucket::<Raw, Raw>(Some("data"))?;

    let kv_candidates = bucket.iter().filter_map(|data| {
        if let Ok(item) = data {
            match (item.key(), item.value()) {
                (Ok(key), Ok(value)) => {
                    let dbid = DBID::from_raw(&key).unwrap();
                    if dbid.m == 0.1 {
                        Some((key, value))
                    } else {
                        None
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    });

    for (key, value) in kv_candidates {
        let dbid = DBID::from_raw(&key).unwrap();
        let matrix = Matrix::from_raw(&value).unwrap();
        println!("id: {:?}, m: {:.4}", dbid.id, dbid.m);
        matrix.row(0).print();
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
