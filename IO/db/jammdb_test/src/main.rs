use peroxide::fuga::*;
use serde::{Deserialize, Serialize};
use jammdb::{DB, Data};

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
        Self {
            id,
            m,
        }
    }
}

fn write() -> Result<(), Box<dyn std::error::Error>> {
    // Remove "data/jamm.db" if exists
    let file = std::fs::File::open("data/jamm.db");
    if file.is_ok() {
        std::fs::remove_file("data/jamm.db")?;
    }

    let db = DB::open("data/jamm.db")?;
    let tx = db.tx(true)?;
    let bucket = tx.create_bucket("data")?;

    let u = Uniform(0.0, 1.0);
    let mut m_vec = u.sample(I*J);
    m_vec[9301] = 0.1;
    for i in 0 .. I {
        for j in 0 .. J {
            let m = m_vec[i * J + j];
            let matrix = rand(ROW, COL);
            let dbid = DBID::new((i, j), m);

            let dbid_ = rmp_serde::to_vec(&dbid)?;
            let matrix_ = rmp_serde::to_vec(&matrix)?;

            bucket.put(dbid_, matrix_)?;
        }
    }
    tx.commit()?;

    Ok(())
}

fn update() -> Result<(), Box<dyn std::error::Error>> {
    let db = DB::open("data/jamm.db")?;
    let tx = db.tx(true)?;
    let bucket = tx.get_bucket("data")?;

    let mut key_to_update = vec![];

    let kv_candidates = bucket.cursor().filter_map(|data| {
        if let Data::KeyValue(kv) = data {
            let dbid: DBID = rmp_serde::from_slice(kv.key()).unwrap();
            if dbid.m == 0.1 {
                Some(kv)
            } else {
                None
            }
        } else {
            None
        }
    });

    for kv in kv_candidates {
        let dbid: DBID = rmp_serde::from_slice(kv.key()).unwrap();
        let matrix: Matrix = rmp_serde::from_slice(kv.value()).unwrap();
        println!("id: {:?}, m: {:.4}", dbid.id, dbid.m);
        matrix.row(0).print();
        key_to_update.push(dbid);
    }

    for key in key_to_update {
        let matrix = zeros(ROW, COL);
        let matrix_ = rmp_serde::to_vec(&matrix)?;

        println!("id: {:?}, m: {:.4}", key.id, key.m);
        matrix.row(0).print();

        bucket.put(rmp_serde::to_vec(&key)?, matrix_)?;
    }

    tx.commit()?;

    Ok(())
}

fn read() -> Result<(), Box<dyn std::error::Error>> {
    let db = DB::open("data/jamm.db")?;
    let tx = db.tx(true)?;
    let bucket = tx.get_bucket("data")?;

    let kv_candidates = bucket.cursor().filter_map(|data| {
        if let Data::KeyValue(kv) = data {
            let dbid: DBID = rmp_serde::from_slice(kv.key()).unwrap();
            if dbid.m == 0.1 {
                Some(kv)
            } else {
                None
            }
        } else {
            None
        }
    });

    for kv in kv_candidates {
        let dbid: DBID = rmp_serde::from_slice(kv.key()).unwrap();
        let matrix: Matrix = rmp_serde::from_slice(kv.value()).unwrap();
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
