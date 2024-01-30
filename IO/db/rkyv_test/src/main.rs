use peroxide::fuga::*;
use rkyv::{Archive, Serialize, Deserialize, to_bytes, from_bytes_unchecked};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

const I: usize = 100;
const J: usize = 500;
const ROW: usize = 500;
const COL: usize = 4;

#[derive(Debug, Archive, Serialize, Deserialize, Copy, Clone, PartialEq)]
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

    fn to_tuple(self) -> (usize, usize, u64) {
        (self.id.0, self.id.1, self.m.to_bits())
    }

    fn from_tuple(tuple: (usize, usize, u64)) -> Self {
        Self::new((tuple.0, tuple.1), f64::from_bits(tuple.2))
    }
}

#[derive(Debug, Archive, Serialize, Deserialize, Copy, Clone)]
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

#[derive(Debug, Archive, Serialize, Deserialize, Clone)]
struct DBMatrix {
    data: Vec<f64>,
    row: usize,
    col: usize,
    shape: DBShape
}

impl DBMatrix {
    fn to_matrix(&self) -> Matrix {
        matrix(self.data.clone(), self.row, self.col, self.shape.to_shape())
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

type DBKey = (usize, usize, u64);
type DB = HashMap<DBKey, DBMatrix>;

fn write() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open("data/rkyv.db");
    if file.is_ok() {
        std::fs::remove_file("data/rkyv.db")?;
    }
    let path = Path::new("data/rkyv.db");
    std::fs::File::create(path)?;

    let mut db_map = DB::new();

    let u = Uniform(0.0, 1.0);
    let mut m_vec = u.sample(I * J);
    m_vec[9301] = 0.1;
    for i in 0 .. I {
        for j in 0 .. J {
            let m = m_vec[i * J + j];
            let matrix = rand(ROW, COL);
            let dbid = DBID::new((i, j), m);
            let db_matrix = DBMatrix::from_matrix(matrix);

            db_map.insert(dbid.to_tuple(), db_matrix);
        }
    }
    let bytes = to_bytes::<_, 8192>(&db_map).unwrap();
    fs::write(path, bytes)?;

    Ok(())
}

fn update() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("data/rkyv.db");
    let bytes = fs::read(path)?;
    let mut db: DB = unsafe { from_bytes_unchecked(&bytes)? };

    for (key, db_matrix) in db.iter_mut() {
        let dbid = DBID::from_tuple(*key);
        if dbid.m() == 0.1 {
            *db_matrix = DBMatrix::from_matrix(zeros(ROW, COL));
        }
    }
    let bytes = to_bytes::<_, 8192>(&db).unwrap();
    fs::write(path, bytes)?;

    Ok(())
}

fn read() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("data/rkyv.db");
    let bytes = fs::read(path)?;
    let db: DB = unsafe { from_bytes_unchecked(&bytes)? };

    for (key, db_matrix) in db.iter() {
        let dbid = DBID::from_tuple(*key);
        if dbid.m() == 0.1 {
            println!("id: {:?}, m: {:.4}", dbid.id(), dbid.m());
            let matrix = db_matrix.to_matrix();
            matrix.row(0).print();
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
