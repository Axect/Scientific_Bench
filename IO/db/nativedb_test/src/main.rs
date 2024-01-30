use peroxide::fuga::*;
use serde::{Deserialize, Serialize};
use native_db::*;
use native_model::{native_model, Model};

const I: usize = 100;
const J: usize = 500;
const ROW: usize = 500;
const COL: usize = 4;

#[derive(Debug, Serialize, Deserialize)]
#[native_model(id = 1, version = 1)]
#[native_db]
struct DBMatrix {
    #[primary_key]
    pub id: (u32, u32),
    #[secondary_key]
    pub m: f64,
    pub matrix: Matrix,
}

impl DBMatrix {
    pub fn from_param_and_matrix(id: (u32, u32), m: f64, matrix: Matrix) -> Self {
        Self {
            id,
            m,
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
    let u = Uniform(0.0, 1.0);
    let mut m_vec = u.sample(I*J);
    m_vec[9301] = 0.1;
    for i in 0 .. I {
        for j in 0 .. J {
            let m = m_vec[i * J + j];
            let matrix = rand(ROW, COL);
            rw.insert(
                DBMatrix::from_param_and_matrix((i as u32, j as u32), m, matrix)
            )?;
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
    let m = 0.1;
    let mut key_to_update = vec![];
    for item in r.scan().secondary::<DBMatrix>(DBMatrixKey::m)?.start_with(m) {
        println!("id: {:?}, m: {:.4}", item.id, item.m);
        item.matrix.row(0).print();

        key_to_update.push(item);
    }

    let rw = db.rw_transaction().unwrap();
    for item in key_to_update {
        let id = item.id;
        let m = item.m;
        let matrix = zeros(ROW, COL);

        println!("id: {:?}, m: {:.4}", id, m);
        matrix.row(0).print();

        rw.update(
            item, DBMatrix::from_param_and_matrix(id, m, matrix)
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

    for item in r.scan().secondary::<DBMatrix>(DBMatrixKey::m)?.start_with(0.1) {
        println!("id: {:?}, m: {:.4}", item.id, item.m);
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
