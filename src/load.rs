use crate::*;

use std::collections::{HashMap, HashSet};

pub static LINEAR_MAGMAS: &[&'static str] = &[ "31/2", "13/0", "71/0", "7/1", "71/3", "11/2", "41/9", "43/1", "97/1", "97/0", "37/0", "41/8", "25/3", "16/0", "31/5", "5/0", "7/0", "31/4", "49/1", "67/0", "49/0", "43/0", "71/2", "19/1", "41/7", "61/0", "37/1", "43/3", "81/0", "49/3", "61/3", "61/2", "9/0", "16/1", "41/10", "19/0", "11/0", "31/1", "43/2", "67/1", "11/1", "71/1", "75/0", "61/1", "75/1", "0/0", "11/3", "81/1", "31/3"];

pub fn load_file(file: &str) -> HashSet<&'static str> {
    let dbmap: HashMap<MatrixMagma, &'static str> = db().into_iter().map(|(name, m)| (m, name)).collect();
    let mut out = HashSet::new();

    let s = std::fs::read_to_string(file).unwrap();
    for m in s.split("--") {
        let m = MatrixMagma::parse(m).canonicalize2();
        if let Some(n) = dbmap.get(&m) {
            out.insert(*n);
        } else {
            println!("New magma of size {}!", m.n);
            m.dump();

            std::io::stdin()
                .read_line(&mut String::new()).unwrap();
        }
    }
    out
}
