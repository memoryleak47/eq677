use crate::*;
use std::collections::HashSet;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref DB: Mutex<HashSet<MatrixMagma>> = Mutex::new(HashSet::new());
}

pub fn present_model(n: usize, f: impl Fn(usize, usize) -> usize) {
    let magma = MatrixMagma::by_fn(n, f);
    let magma = magma.canonicalize();

    let mut handle = DB.lock().unwrap();
    if handle.contains(&magma) {
        println!("duplicate model found! suboptimal symmetry breaking!");
        return;
    }

    handle.insert(magma.clone());

    println!("Model found:");
    magma.dump();
    // ctxt.dump();

    assert!(magma.is677());
    assert!(magma.is255());
    conj(&magma);
}

