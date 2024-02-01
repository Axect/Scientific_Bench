use peroxide::fuga::*;
use serde::{Deserialize, Serialize};
use native_db::*;
use native_model::{native_model, Model};

const ROW: usize = 500;
const COL: usize = 4;

#[derive(Debug, Serialize, Deserialize)]
#[native_model(id = 1, version = 1)]
#[native_db]
struct DBMatrix {
    #[primary_key]
    pub m_pbh: f64,
    #[secondary_key]
    pub m_a: f64,
    pub matrix: Matrix,
}

impl DBMatrix {
    pub fn from_param_and_matrix(m_pbh: f64, m_a: f64, matrix: Matrix) -> Self {
        Self {
            m_pbh,
            m_a,
            matrix,
        }
    }
}

fn write() -> Result<(), Box<dyn std::error::Error>> {
    // Remove "data/native.db" if exists
    let file = std::fs::File::open("data/native.db");
    if file.is_ok() {
        std::fs::remove_file("data/native.db")?;
    }

    let mut builder = DatabaseBuilder::new();
    builder.define::<DBMatrix>()?;

    let db = builder.create("data/native.db")?;
    let rw = db.rw_transaction().unwrap();

    let m_pbh_vec = (1 .. 100).map(|x| (x as f64) * 1e+15).collect::<Vec<_>>();
    let m_a_vec = seq_with_precision(1e-3, 1e-1, 1e-3, 3);

    for &m_pbh in m_pbh_vec.iter() {
        for &m_a in m_a_vec.iter() {
            let matrix = rand(ROW, COL);
            rw.insert(DBMatrix::from_param_and_matrix(m_pbh, m_a, matrix))?;
        }
    }

    rw.commit()?;

    Ok(())
}

fn update() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = DatabaseBuilder::new();
    builder.define::<DBMatrix>()?;

    let db = builder.open("data/native.db")?;
    let r = db.r_transaction().unwrap();
    let target_m_a = 9e-2;

    let mut key_to_update = vec![];
    for item in r.scan().secondary::<DBMatrix>(DBMatrixKey::m_a)?.start_with(target_m_a) {
        println!("m_pbh: {:.2e}, m: {:.2e}", item.m_pbh, item.m_a);
        item.matrix.row(0).print();

        key_to_update.push(item);
    }

    let rw = db.rw_transaction().unwrap();
    for item in key_to_update {
        let m_pbh = item.m_pbh;
        let m_a = item.m_a;
        let matrix = zeros(ROW, COL);

        rw.update(
            item, DBMatrix::from_param_and_matrix(m_pbh, m_a, matrix)
        )?;
    }
    rw.commit()?;

    Ok(())
}

fn read() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = DatabaseBuilder::new();
    builder.define::<DBMatrix>()?;
    let db = builder.open("data/native.db")?;
    let r = db.r_transaction()?;

    for item in r.scan().secondary::<DBMatrix>(DBMatrixKey::m_a)?.start_with(9e-2) {
        println!("m_pbh: {:.2e}, m: {:.2e}", item.m_pbh, item.m_a);
        item.matrix.row(0).print();
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let read_or_write = std::env::args().nth(1).unwrap();

    match read_or_write.as_str().trim() {
        "read" => read()?,
        "update" => update()?,
        "write" => write()?,
        _ => panic!("Invalid argument"),
    }

    Ok(())
}
