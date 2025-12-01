use crate::*;
use std::collections::HashSet;
use std::sync::Mutex;

lazy_static::lazy_static! {
    pub static ref PRINT_MUTEX: Mutex<()> = Mutex::new(());
    pub static ref PRESENT_DB: Mutex<HashSet<MagmaName>> = Mutex::new(HashSet::new());
}

const CHECK_COMPOSITE: bool = false;
const SHOW_AUTOM: bool = false;

pub fn get_present_db() -> Vec<MagmaName> {
    let handle = PRESENT_DB.lock().unwrap();
    let mut v: Vec<_> = (*handle).iter().cloned().collect();
    drop(handle);
    v.sort();
    v
}

pub fn present_db_contains(name: MagmaName) -> bool {
    let handle = PRESENT_DB.lock().unwrap();
    handle.contains(&name)
}

pub fn present_db_add(name: MagmaName) -> bool {
    let mut handle = PRESENT_DB.lock().unwrap();
    handle.insert(name)
}

pub fn present_model(n: usize, finder: &str, f: impl Fn(usize, usize) -> usize) {
    let magma = MatrixMagma::by_fn(n, f);

    let mut globally_new_str = "";
    if n <= 100 {
        let (name, globally_new) = db_intern(&magma);
        if !present_db_add(name) { return; }
        if globally_new {
            globally_new_str = "New ";
        }
    }

    let mut print_handle = PRINT_MUTEX.lock().unwrap();

    if n <= 100 {
        println!("{globally_new_str}Model of size {n} found by {finder}:");
        magma.canonicalize2().dump();
    } else {
        println!("{globally_new_str}Model of size {n} found by {finder}");
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
