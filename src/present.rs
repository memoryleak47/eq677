use crate::*;
use std::collections::HashSet;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref DB: Mutex<HashSet<MatrixMagma>> = Mutex::new(HashSet::new());
    pub static ref PRINT_MUTEX: Mutex<()> = Mutex::new(());
}

const CHECK_COMPOSITE: bool = false;
const SHOW_AUTOM: bool = true;

pub fn present_model(n: usize, finder: &str, f: impl Fn(usize, usize) -> usize) {
    let magma = MatrixMagma::by_fn(n, f);

    if n < 100 {
        let canon = magma.canonicalize2();

        let mut handle = DB.lock().unwrap();
        if handle.contains(&canon) {
            return;
        }

        handle.insert(canon);
    }

    let mut print_handle = PRINT_MUTEX.lock().unwrap();

    if n < 52 {
        println!("Model of size {n} found by {finder}:");
        magma.cycle_dump();
    } else {
        println!("Model found of size {n} found by {finder}");
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
    if SHOW_AUTOM && magma.n > 1 {
        println!("----");
        for x in magma.autom_group() {
            for i in 0..magma.n {
                print!("{} → {}", i, x[i]);
                if i != magma.n -1 {
                    print!(", ");
                } else {
                    println!();
                }
            }
        }
        println!("----");
    }

    conj(&magma);
}
