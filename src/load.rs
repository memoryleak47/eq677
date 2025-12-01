use crate::*;

use std::collections::{HashMap, HashSet};

pub fn load_file(file: &str) -> HashSet<MagmaName> {
    let mut out = HashSet::new();

    let s = std::fs::read_to_string(file).unwrap();
    for m in s.split("--") {
        let m = MatrixMagma::parse(m);
        let (n, new) = db_intern(&m);
        if new {
            println!("New magma of size {}!", m.n);
            m.dump();
        }
    }
    out
}
