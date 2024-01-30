use peroxide::fuga::*;
use jammdb::{DB, Data};
use serde::{Serialize, Deserialize};
use postcard::{from_bytes, to_allocvec};

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
    // Remove "data/jammpc.db" if exists
    let file = std::fs::File::open("data/jammpc.db");
    if file.is_ok() {
        std::fs::remove_file("data/jammpc.db")?;
    }

    let db = DB::open("data/jammpc.db")?;
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

            let dbid_ = to_allocvec(&dbid)?;
            let matrix_ = to_allocvec(&matrix)?;

            bucket.put(dbid_, matrix_)?;
        }
    }
    tx.commit()?;

    Ok(())
}

fn update() -> Result<(), Box<dyn std::error::Error>> {
    let db = DB::open("data/jammpc.db")?;
    let tx = db.tx(true)?;
    let bucket = tx.get_bucket("data")?;

    let mut key_to_update = vec![];

    let kv_candidates = bucket.cursor().filter_map(|data| {
        if let Data::KeyValue(kv) = data {
            let dbid: DBID = from_bytes(kv.key()).unwrap();
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
        let dbid: DBID = from_bytes(kv.key()).unwrap();
        let matrix: Matrix = from_bytes(kv.value()).unwrap();
        println!("id: {:?}, m: {:.4}", dbid.id, dbid.m);
        matrix.row(0).print();
        key_to_update.push(dbid);
    }

    for key in key_to_update {
        let matrix = zeros(ROW, COL);
        let matrix_ = to_allocvec(&matrix)?;

        println!("id: {:?}, m: {:.4}", key.id, key.m);
        matrix.row(0).print();

        bucket.put(to_allocvec(&key)?, matrix_)?;
    }

    tx.commit()?;

    Ok(())
}

fn read() -> Result<(), Box<dyn std::error::Error>> {
    let db = DB::open("data/jammpc.db")?;
    let tx = db.tx(true)?;
    let bucket = tx.get_bucket("data")?;

    let kv_candidates = bucket.cursor().filter_map(|data| {
        if let Data::KeyValue(kv) = data {
            let dbid: DBID = from_bytes(kv.key()).unwrap();
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
        let dbid: DBID = from_bytes(kv.key()).unwrap();
        let matrix: Matrix = from_bytes(kv.value()).unwrap();
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
