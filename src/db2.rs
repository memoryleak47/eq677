use crate::*;
use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

static DB: RwLock<Vec<DB>> = RwLock::new(Vec::new());

pub type MagmaName = (usize, usize);

// Each magma size has its own DB.
#[derive(Clone)]
struct DB {
    magmas: Vec<MatrixMagma>,
    map: HashMap<MatrixMagma, usize>,
}

pub fn db_get((i, j): MagmaName) -> MatrixMagma {
    let handle = DB.read().unwrap();
    handle[i].magmas[j].clone()
}

pub fn db_add(m: MatrixMagma) -> MagmaName {
    let i = m.n;
    let handle = DB.read().unwrap();
    if let Some(j) = handle[i].map.get(&m) { return (i, *j); }
    drop(handle);

    let mut handle = DB.write().unwrap();
    if let Some(j) = handle[i].map.get(&m) { return (i, *j); }
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
    (i, j)
}
