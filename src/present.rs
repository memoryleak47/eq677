use crate::*;
use std::collections::HashSet;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref DB: Mutex<HashSet<MatrixMagma>> = Mutex::new(HashSet::new());
}

pub fn present_model(n: usize, f: impl Fn(usize, usize) -> usize) {
    let magma = MatrixMagma::by_fn(n, f);
    let magma = if n < 30 { magma.canonicalize() } else { magma };

    let mut handle = DB.lock().unwrap();
    if handle.contains(&magma) {
        return;
    }

    handle.insert(magma.clone());

    println!("Model found:");
    if n < 50 {
        magma.dump();
    } else {
        println!("  ...");
    }

    conj(&magma);
}

