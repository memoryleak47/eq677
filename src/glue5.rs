use crate::*;
use std::collections::HashSet;

pub fn glue5_chk(m: &MatrixMagma) -> bool {
    if m.n < 5 { return false }
    if !m.is_idempotent() { return false }

    for x in 0..m.n {
        for y in 0..m.n {
            if x == y { continue }

            let mut set = HashSet::new();
            set.insert(x);
            set.insert(y);
            complete(m, &mut set);
            if set.len() != 5 { return false }
        }
    }
    true
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
