use crate::*;
use std::collections::HashSet;

pub fn glue5_chk(m: &MatrixMagma) -> bool {
    if m.n < 5 { return false }
    if !m.is_idempotent() { return false }

    for x in 0..m.n {
        for y in 0..m.n {
            if x == y { continue }
            if !induces5(m, x, y) { return false }
        }
    }
    true
}

pub fn contains5(m: &MatrixMagma) -> bool {
    for x in 0..m.n {
        for y in 0..m.n {
            if induces5(m, x, y) { return true}
        }
    }
    false
}

fn induces5(m: &MatrixMagma, x: usize, y: usize) -> bool {
    let mut set = HashSet::new();
    set.insert(x);
    set.insert(y);
    complete(m, &mut set);

    // no need to check what magma it is.
    // If it's an induced submagma of size 5, it has to be 5/0.
    set.len() == 5
}

fn complete(m: &MatrixMagma, set: &mut HashSet<usize>) {
    loop {
        let n = set.len();
        let s = set.clone();
        for x in s.iter() {
            for y in s.iter() {
                set.insert(m.f(*x, *y));
            }
        }

        if n == set.len() { return }
    }
}
