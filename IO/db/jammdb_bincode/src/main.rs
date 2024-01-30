use peroxide::fuga::*;
use jammdb::{DB, Data};
use bincode::{config, Encode, Decode, encode_to_vec, decode_from_slice};

const I: usize = 100;
const J: usize = 500;
const ROW: usize = 500;
const COL: usize = 4;

#[derive(Debug, Encode, Decode, Copy, Clone)]
struct DBID {
    id: (usize, usize),
    m: f64,
}

#[derive(Debug, Encode, Decode, Copy, Clone)]
enum DBShape {
    Row,
    Col
}

impl DBShape {
    fn to_shape(self) -> Shape {
        match self {
            Self::Row => Shape::Row,
            Self::Col => Shape::Col
        }
    }

    fn from_shape(shape: Shape) -> Self {
        match shape {
            Shape::Row => Self::Row,
            Shape::Col => Self::Col
        }
    }
}

#[derive(Debug, Encode, Decode, Clone)]
struct DBMatrix {
    data: Vec<f64>,
    row: usize,
    col: usize,
    shape: DBShape
}

impl DBMatrix {
    fn to_matrix(self) -> Matrix {
        matrix(self.data, self.row, self.col, self.shape.to_shape())
    }

    fn from_matrix(matrix: Matrix) -> Self {
        Self {
            data: matrix.data,
            row: matrix.row,
            col: matrix.col,
            shape: DBShape::from_shape(matrix.shape),
        }
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

fn write() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::standard();
    // Remove "data/jammbc.db" if exists
    let file = std::fs::File::open("data/jammbc.db");
    if file.is_ok() {
        std::fs::remove_file("data/jammbc.db")?;
    }

    let db = DB::open("data/jammbc.db")?;
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
            let db_matrix = DBMatrix::from_matrix(matrix);

            let dbid_ = encode_to_vec(dbid, config)?;
            let matrix_ = encode_to_vec(db_matrix, config)?;

            bucket.put(dbid_, matrix_)?;
        }
    }
    tx.commit()?;

    Ok(())
}

fn update() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::standard();
    let db = DB::open("data/jammbc.db")?;
    let tx = db.tx(true)?;
    let bucket = tx.get_bucket("data")?;

    let mut key_to_update = vec![];

    let kv_candidates = bucket.cursor().filter_map(|data| {
        if let Data::KeyValue(kv) = data {
            let (dbid, _): (DBID, usize) = decode_from_slice(kv.key(), config).unwrap();
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
        let (dbid, _): (DBID, usize) = decode_from_slice(kv.key(), config).unwrap();
        let (db_matrix, _): (DBMatrix, usize) = decode_from_slice(kv.value(), config).unwrap();
        let matrix = db_matrix.to_matrix();
        println!("id: {:?}, m: {:.4}", dbid.id, dbid.m);
        matrix.row(0).print();
        key_to_update.push(dbid);
    }

    for key in key_to_update {
        let matrix = zeros(ROW, COL);

        println!("id: {:?}, m: {:.4}", key.id, key.m);
        matrix.row(0).print();

        let matrix_ = encode_to_vec(&DBMatrix::from_matrix(matrix), config)?;
        bucket.put(encode_to_vec(key, config)?, matrix_)?;
    }

    tx.commit()?;

    Ok(())
}

fn read() -> Result<(), Box<dyn std::error::Error>> {
    let db = DB::open("data/jammbc.db")?;
    let tx = db.tx(true)?;
    let bucket = tx.get_bucket("data")?;

    bucket.cursor().for_each(|data| {
        if let Data::KeyValue(kv) = data {
            let (dbid, _): (DBID, usize) = decode_from_slice(kv.key(), config::standard()).unwrap();
            if dbid.m == 0.1 {
                println!("id: {:?}, m: {:.4}", dbid.id, dbid.m);
                let (db_matrix, _): (DBMatrix, usize) = decode_from_slice(kv.value(), config::standard()).unwrap();
                let matrix = db_matrix.to_matrix();
                matrix.row(0).print();
            }
        }
    });

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
