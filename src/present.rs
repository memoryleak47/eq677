use crate::*;
use std::collections::HashSet;
use std::sync::Mutex;

lazy_static::lazy_static! {
    pub static ref PRINT_MUTEX: Mutex<()> = Mutex::new(());
}

const CHECK_COMPOSITE: bool = false;
const SHOW_AUTOM: bool = false;

pub fn present_model(n: usize, finder: &str, f: impl Fn(usize, usize) -> usize) {
    let magma = MatrixMagma::by_fn(n, f);

    if n <= 100 {
        let (name, new) = db_intern(&magma);
        if !new { return; }
    }

    let mut print_handle = PRINT_MUTEX.lock().unwrap();

    if n <= 100 {
        println!("Model of size {n} found by {finder}:");
        magma.canonicalize2().dump();
    } else {
        println!("Model of size {n} found by {finder}");
    }

    if CHECK_COMPOSITE {
        let ms = decompose(&magma);
        if ms.len() > 0 {
            println!("decomposable into:");
            for m in ms {
                println!("---");
                m.cycle_dump();
            }
            println!("---");
        }
    }
    if SHOW_AUTOM && magma.n > 1 && magma.n < 25 {
        magma.autom_dump();
        println!();
    }

    conj(&magma);
}
