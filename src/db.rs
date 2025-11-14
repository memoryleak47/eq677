use crate::*;

include!(concat!(env!("OUT_DIR"), "/db_sources.rs"));

pub fn db_search() {
    for m in db() {
        present_model(m.n as usize, "db", |x, y| m.f(x, y));
    }
}

pub fn db() -> Vec<MatrixMagma> {
    let mut out = Vec::new();
    for x in DB_SOURCES {
        out.push(MatrixMagma::parse(x));
    }
    out
}

#[test]
fn db_unique() {
    use std::collections::HashSet;

    let db = db();
    let n = db.len();
    let s: HashSet<_> = db.iter().map(|x| x.canonicalize2()).collect();
    assert_eq!(n, s.len());
}

#[test]
fn db_is677() {
    for m in db() {
        assert!(m.is677());
    }
}

#[test]
fn db_is255() {
    for m in db() {
        assert!(m.is255());
    }
}
