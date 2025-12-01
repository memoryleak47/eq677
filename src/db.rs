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

pub fn db() -> Vec<(Name, MatrixMagma)> {
    let mut out = Vec::new();
    let db = DB_REF.read().unwrap();
    for (i, dbs) in db.iter().enumerate() {
        for (j, m) in dbs.magmas.iter().enumerate() {
            out.push((Name(i, j), m.clone()));
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

    let mut map: HashMap<MatrixMagma, Name> = HashMap::default();
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

pub fn db_get(Name(i, j): Name) -> MatrixMagma {
    let handle = DB_REF.read().unwrap();
    handle[i].magmas[j].clone()
}

pub fn db_intern(m: &MatrixMagma) -> (Name, /*fresh*/ bool) {
    let m = m.canonicalize2();
    let i = m.n;
    if i > 100 { panic!("Interning is only supported up to n=100"); }

    let handle = DB_REF.read().unwrap();
    if let Some(j) = handle[i].map.get(&m) { return (Name(i, *j), false); }
    drop(handle);

    let mut handle = DB_REF.write().unwrap();
    if let Some(j) = handle[i].map.get(&m) { return (Name(i, *j), false); }
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
    (Name(i, j), true)
}


#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
// The name of a magma (up to isomorphism).
pub struct Name(pub usize, pub usize);

use std::fmt::*;

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}/{}", self.0, self.1)
    }
}

pub static LINEAR_MODELS: &[Name] = &[
    Name(5, 0),
    Name(7, 0), Name(7, 1),
    Name(9, 0),
    Name(11, 0), Name(11, 1), Name(11, 2), Name(11, 3),
    Name(13, 0),
    Name(16, 0), Name(16, 1),
    Name(19, 0), Name(19, 1),
    Name(25, 3),
    Name(31, 1), Name(31, 2), Name(31, 3), Name(31, 4), Name(31, 5),
    Name(37, 0), Name(37, 1),
    Name(41, 7), Name(41, 8), Name(41, 9), Name(41, 10),
    Name(43, 0), Name(43, 1), Name(43, 2), Name(43, 3),
    Name(49, 0), Name(49, 1), Name(49, 3),
    Name(61, 0), Name(61, 1), Name(61, 2), Name(61, 3),
    Name(67, 0), Name(67, 1),
    Name(71, 0), Name(71, 1), Name(71, 2), Name(71, 3),
    Name(73, 0), Name(73, 1),
    Name(81, 0), Name(81, 1),
    Name(97, 0), Name(97, 1),
];

pub static LINEAR_EXTENSIONS: &[Name] = &[
    Name(25, 8),
    Name(35, 12), Name(35, 13), Name(35, 14), Name(35, 15), Name(35, 16), Name(35, 17), Name(35, 18), Name(35, 19), Name(35, 20), Name(35, 21),
    Name(45, 1), Name(45, 2), Name(45, 3), Name(45, 4),
    Name(49, 7), Name(49, 8), Name(49, 9), Name(49, 10), Name(49, 11), Name(49, 12),
    Name(63, 2), Name(63, 3), Name(63, 4), Name(63, 5),
    Name(65, 1), Name(65, 2), Name(65, 3), Name(65, 4), Name(65, 5), Name(65, 6),
    Name(77, 8), Name(77, 9), Name(77, 10), Name(77, 11), Name(77, 12), Name(77, 13), Name(77, 14), Name(77, 15), Name(77, 16), Name(77, 17), Name(77, 18), Name(77, 19), Name(77, 20), Name(77, 21), Name(77, 22), Name(77, 23), Name(77, 24), Name(77, 25), Name(77, 26), Name(77, 27), Name(77, 28), Name(77, 29), Name(77, 30), Name(77, 31), Name(77, 32), Name(77, 33), Name(77, 34), Name(77, 35), Name(77, 36), Name(77, 37), Name(77, 38), Name(77, 39), Name(77, 40), Name(77, 41), Name(77, 42), Name(77, 43), Name(77, 44), Name(77, 45), Name(77, 46), Name(77, 47), Name(77, 48), Name(77, 49), Name(77, 50),
    Name(81, 2), Name(81, 3),
    Name(91, 2), Name(91, 3), Name(91, 4), Name(91, 5),
    Name(99, 4), Name(99, 5), Name(99, 6), Name(99, 7), Name(99, 8), Name(99, 9), Name(99, 10), Name(99, 11), Name(99, 12), Name(99, 13), Name(99, 14), Name(99, 15), Name(99, 16), Name(99, 17), Name(99, 18), Name(99, 19), Name(99, 20), Name(99, 21), Name(99, 22), Name(99, 23), Name(99, 24), Name(99, 25), Name(99, 26), Name(99, 27), Name(99, 28), Name(99, 29), Name(99, 30), Name(99, 31), Name(99, 32), Name(99, 33),
];
