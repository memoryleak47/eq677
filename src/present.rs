use crate::*;
use std::collections::HashSet;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref DB: Mutex<HashSet<MatrixMagma>> = Mutex::new(HashSet::new());
}

pub fn present_model(n: usize, finder: &str, f: impl Fn(usize, usize) -> usize) {
    let mut magma = MatrixMagma::by_fn(n, f);
    if n < 30 {
        magma = magma.canonicalize();

        let mut handle = DB.lock().unwrap();
        if handle.contains(&magma) {
            return;
        }

        handle.insert(magma.clone());
    }

    if n < 50 {
        // Locking the handle prevents scrambling of stdout.
        let handle = DB.lock().unwrap();
        println!("Model of size {n} found by {finder}:");
        magma.cycle_dump();
    } else {
        let handle = DB.lock().unwrap();
        println!("Model found of size {n} found by {finder}");
    }

    conj(&magma);
}

