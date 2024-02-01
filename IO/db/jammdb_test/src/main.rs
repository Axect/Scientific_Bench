use peroxide::fuga::*;
use serde::{Deserialize, Serialize};
use jammdb::{DB, Data};

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

    let m_pbh_vec = (1 .. 100).map(|x| (x as f64) * 1e+15).collect::<Vec<_>>();
    let m_a_vec = seq_with_precision(1e-3, 1e-1, 1e-3, 3);

    for &m_pbh in &m_pbh_vec {
        for &m_a in &m_a_vec {
            let matrix = rand(ROW, COL);
            let db_param = rmp_serde::to_vec(&DBParam::new(m_pbh, m_a))?;
            let db_matrix = rmp_serde::to_vec(&matrix)?;

            bucket.put(db_param, db_matrix)?;
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
            let db_param: DBParam = rmp_serde::from_slice(kv.key()).unwrap();
            if db_param.m_a == 9e-2 {
                Some(kv)
            } else {
                None
            }
        } else {
            None
        }
    });

    for kv in kv_candidates {
        let db_param: DBParam = rmp_serde::from_slice(kv.key()).unwrap();
        let matrix: Matrix = rmp_serde::from_slice(kv.value()).unwrap();
        println!("m_pbh: {:.2e}, m_a: {:.2e}", db_param.m_pbh, db_param.m_a);
        matrix.row(0).print();
        key_to_update.push(db_param);
    }

    for key in key_to_update {
        let matrix = zeros(ROW, COL);
        let matrix_ = rmp_serde::to_vec(&matrix)?;

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
            let db_param: DBParam = rmp_serde::from_slice(kv.key()).unwrap();
            if db_param.m_a == 9e-2 {
                Some(kv)
            } else {
                None
            }
        } else {
            None
        }
    });

    for kv in kv_candidates {
        let db_param: DBParam = rmp_serde::from_slice(kv.key()).unwrap();
        let matrix: Matrix = rmp_serde::from_slice(kv.value()).unwrap();
        println!("m_pbh: {:.2e}, m_a: {:.2e}", db_param.m_pbh, db_param.m_a);
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
