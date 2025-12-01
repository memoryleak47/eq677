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


#[derive(Clone, Copy)]
pub struct MagmaName(usize, usize);

use std::fmt::*;

impl Display for MagmaName {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}/{}", self.0, self.1)
    }
}
