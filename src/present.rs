use crate::*;
use std::collections::HashSet;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref DB: Mutex<HashSet<MatrixMagma>> = Mutex::new(HashSet::new());
}

pub fn present_model(n: usize, finder: &str, f: impl Fn(usize, usize) -> usize) {
    let mut magma = MatrixMagma::by_fn(n, f);
    if n < 32 {
        magma = magma.canonicalize();
    }

    // Locking the handle prevents scrambling of stdout.
    let mut handle = DB.lock().unwrap();

    if n < 32 {
        if handle.contains(&magma) {
            return;
        }

        handle.insert(magma.clone());
    }

    if n < 50 {
        println!("Model of size {n} found by {finder}:");
        magma.cycle_dump();
    } else {
        println!("Model found of size {n} found by {finder}");
    }

    let ms = decompose(&magma);
    if ms.len() > 0 {
        println!("decomposable into:");
        for m in ms {
            println!("---");
            m.cycle_dump();
        }
        println!("---");
    }

    conj(&magma);
}

