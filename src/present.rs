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

    let mut new_str = "";
    if n <= 100 {
        let (name_, new_) = db_intern(&magma);
        if new_ {
            new_str = "New ";
        }
    }

    let mut print_handle = PRINT_MUTEX.lock().unwrap();

    if n <= 100 {
        println!("{new_str}Model of size {n} found by {finder}:");
        magma.canonicalize2().dump();
    } else {
        println!("{new_str}Model of size {n} found by {finder}");
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
