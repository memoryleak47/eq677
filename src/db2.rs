use crate::*;
use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

static DB: OnceLock<RwLock<DB>> = OnceLock::new();

pub type MagmaName = (usize, usize);

struct DB {
    magmas: Vec<Vec<MatrixMagma>>,
    map: HashMap<MatrixMagma, MagmaName>,
}

pub fn db_get((i, j): MagmaName) -> MatrixMagma {
    let db_handle = DB.get().unwrap().read().unwrap();
    db_handle.magmas[i][j].clone()
}

pub fn db_add(m: MatrixMagma) -> MagmaName {
    let i = m.n;
    let db_handle = DB.get().unwrap().read().unwrap();
    if let Some((i, j)) = db_handle.map.get(&m) { return (*i, *j); }
    drop(db_handle);

    let mut db_handle = DB.get().unwrap().write().unwrap();
    if let Some((i, j)) = db_handle.map.get(&m) { return (*i, *j); }
    let magmas = &mut db_handle.magmas;
    while magmas.len() <= i {
        magmas.push(Vec::new());
    }
    let j = magmas[i].len();
    magmas[i].push(m.clone());
    db_handle.map.insert(m, (i, j));
    (i, j)
}
