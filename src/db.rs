use crate::*;
use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};
use lazy_static::lazy_static;

lazy_static! {
    static ref DB_REF: RwLock<Vec<DB>> = RwLock::new(init_db());
}

include!(concat!(env!("OUT_DIR"), "/db_sources.rs"));

pub fn db_search() {
    for (name, m) in db() {
        let s = format!("db: {name}");
        present_model(m.n as usize, &s, |x, y| m.f(x, y));
    }
}

pub fn db_cart_search() {
    for (name1, m1) in db() {
        for (name2, m2) in db() {
            if m1.n * m2.n > 1000 { continue }

            let m = cartesian(&m1, &m2);
            let s = format!("db-cartesian: {name1} ⨯ {name2}");
            present_model(m.n, &s, |x, y| m.f(x, y));
        }
    }
}

pub fn db() -> Vec<(MagmaName, MatrixMagma)> {
    let mut out = Vec::new();
    let db = DB_REF.read().unwrap();
    for (i, dbs) in db.iter().enumerate() {
        for (j, m) in dbs.magmas.iter().enumerate() {
            out.push((MagmaName(i, j), m.clone()));
        }
    }
    out
}

#[test]
fn db_canon() {
    for (name, m) in db() {
        if m.canonicalize2() != m {
            panic!("magma {name} is not canonicalized!");
        }
    }
}

#[test]
fn db_naming() {
    for (name, m) in db() {
        let n = m.n;
        if name.0 != n {
            println!("Name is {name}, but n={n}");
            assert!(false);
        }
    }
}

#[test]
fn db_unique() {
    use std::collections::HashMap;

    let mut map: HashMap<MatrixMagma, MagmaName> = HashMap::default();
    for (name, m) in db() {
        if let Some(name2) = map.insert(m, name.clone()) {
            panic!("Redundant Magmas {name} = {name2}");
        }
    }
}

#[test]
fn db_is677() {
    for (_, m) in db() {
        assert!(m.is677());
    }
}

#[test]
fn db_is255() {
    for (_, m) in db() {
        assert!(m.is255());
    }
}

pub fn init_db() -> Vec<DB> {
    let mut handle = Vec::new();
    for ((i, j), s) in DB_SOURCES.iter() {
        let (i, j) = (*i, *j);
        let m = MatrixMagma::parse(s);
        while handle.len() <= i {
            handle.push(DB {
                magmas: Default::default(),
                map: Default::default(),
            });
        }
        handle[i].magmas.push(m.clone());
        handle[i].map.insert(m, j);
    }
    handle
}

// Each magma size has its own DB.
#[derive(Clone)]
struct DB {
    magmas: Vec<MatrixMagma>,
    map: HashMap<MatrixMagma, usize>,
}

pub fn db_get(MagmaName(i, j): MagmaName) -> MatrixMagma {
    let handle = DB_REF.read().unwrap();
    handle[i].magmas[j].clone()
}

pub fn db_intern(m: &MatrixMagma) -> (MagmaName, /*fresh*/ bool) {
    let m = m.canonicalize2();
    let i = m.n;
    if i > 100 { panic!("Interning is only supported up to n=100"); }

    let handle = DB_REF.read().unwrap();
    if let Some(j) = handle[i].map.get(&m) { return (MagmaName(i, *j), false); }
    drop(handle);

    let mut handle = DB_REF.write().unwrap();
    if let Some(j) = handle[i].map.get(&m) { return (MagmaName(i, *j), false); }
    while handle.len() <= i {
        handle.push(DB {
            magmas: Default::default(),
            map: Default::default(),
        });
    }
    let j = handle[i].magmas.len();
    handle[i].magmas.push(m.clone());
    let s = m.to_string();
    handle[i].map.insert(m, j);
    let path = format!("db/{i}/{j}");
    std::fs::write(&path, s).unwrap();
    (MagmaName(i, j), true)
}


#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MagmaName(pub usize, pub usize);

use std::fmt::*;

impl Display for MagmaName {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}/{}", self.0, self.1)
    }
}

pub static LINEAR_MODELS: &[MagmaName] = &[
    MagmaName(5, 0), MagmaName(7, 0), MagmaName(7, 1), MagmaName(9, 0), MagmaName(11, 0), MagmaName(11, 1), MagmaName(11, 2), MagmaName(11, 3), MagmaName(13, 0), MagmaName(16, 0), MagmaName(16, 1), MagmaName(19, 0), MagmaName(19, 1), MagmaName(25, 3), MagmaName(31, 1), MagmaName(31, 2), MagmaName(31, 3), MagmaName(31, 4), MagmaName(31, 5), MagmaName(37, 0), MagmaName(37, 1), MagmaName(41, 7), MagmaName(41, 8), MagmaName(41, 9), MagmaName(41, 10), MagmaName(43, 0), MagmaName(43, 1), MagmaName(43, 2), MagmaName(43, 3), MagmaName(49, 0), MagmaName(49, 1), MagmaName(49, 3), MagmaName(61, 0), MagmaName(61, 1), MagmaName(61, 2), MagmaName(61, 3), MagmaName(67, 0), MagmaName(67, 1), MagmaName(71, 0), MagmaName(71, 1), MagmaName(71, 2), MagmaName(71, 3), MagmaName(73, 0), MagmaName(73, 1), MagmaName(81, 0), MagmaName(81, 1), MagmaName(97, 0), MagmaName(97, 1),
];

pub static LINEAR_EXTENSIONS: &[MagmaName] = &[
    MagmaName(25, 8), MagmaName(35, 12), MagmaName(35, 13), MagmaName(35, 14), MagmaName(35, 15), MagmaName(35, 16), MagmaName(35, 17), MagmaName(35, 18), MagmaName(35, 19), MagmaName(35, 20), MagmaName(35, 21), MagmaName(45, 1), MagmaName(45, 2), MagmaName(45, 3), MagmaName(45, 4), MagmaName(49, 7), MagmaName(49, 8), MagmaName(49, 9), MagmaName(49, 10), MagmaName(49, 11), MagmaName(49, 12), MagmaName(63, 2), MagmaName(63, 3), MagmaName(63, 4), MagmaName(63, 5), MagmaName(65, 1), MagmaName(65, 2), MagmaName(65, 3), MagmaName(65, 4), MagmaName(65, 5), MagmaName(65, 6), MagmaName(77, 8), MagmaName(77, 9), MagmaName(77, 10), MagmaName(77, 11), MagmaName(77, 12), MagmaName(77, 13), MagmaName(77, 14), MagmaName(77, 15), MagmaName(77, 16), MagmaName(77, 17), MagmaName(77, 18), MagmaName(77, 19), MagmaName(77, 20), MagmaName(77, 21), MagmaName(77, 22), MagmaName(77, 23), MagmaName(77, 24), MagmaName(77, 25), MagmaName(77, 26), MagmaName(77, 27), MagmaName(77, 28), MagmaName(77, 29), MagmaName(77, 30), MagmaName(77, 31), MagmaName(77, 32), MagmaName(77, 33), MagmaName(77, 34), MagmaName(77, 35), MagmaName(77, 36), MagmaName(77, 37), MagmaName(77, 38), MagmaName(77, 39), MagmaName(77, 40), MagmaName(77, 41), MagmaName(77, 42), MagmaName(77, 43), MagmaName(77, 44), MagmaName(77, 45), MagmaName(77, 46), MagmaName(77, 47), MagmaName(77, 48), MagmaName(77, 49), MagmaName(77, 50), MagmaName(81, 2), MagmaName(81, 3), MagmaName(91, 2), MagmaName(91, 3), MagmaName(91, 4), MagmaName(91, 5), MagmaName(99, 4), MagmaName(99, 5), MagmaName(99, 6), MagmaName(99, 7), MagmaName(99, 8), MagmaName(99, 9), MagmaName(99, 10), MagmaName(99, 11), MagmaName(99, 12), MagmaName(99, 13), MagmaName(99, 14), MagmaName(99, 15), MagmaName(99, 16), MagmaName(99, 17), MagmaName(99, 18), MagmaName(99, 19), MagmaName(99, 20), MagmaName(99, 21), MagmaName(99, 22), MagmaName(99, 23), MagmaName(99, 24), MagmaName(99, 25), MagmaName(99, 26), MagmaName(99, 27), MagmaName(99, 28), MagmaName(99, 29), MagmaName(99, 30), MagmaName(99, 31), MagmaName(99, 32), MagmaName(99, 33),
];
