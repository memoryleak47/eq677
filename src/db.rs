use crate::*;

pub fn db_search() {
    for m in db() {
        present_model(m.n as usize, "db", |x, y| m.f(x, y));
    }
}

pub fn db() -> Vec<MatrixMagma> {
    vec![
        MatrixMagma::parse(include_str!("../db/0_0")),
        MatrixMagma::parse(include_str!("../db/1_0")),
        MatrixMagma::parse(include_str!("../db/5_0")),
        MatrixMagma::parse(include_str!("../db/7_0")),
        MatrixMagma::parse(include_str!("../db/7_1")),
        MatrixMagma::parse(include_str!("../db/9_0")),
        MatrixMagma::parse(include_str!("../db/11_0")),
        MatrixMagma::parse(include_str!("../db/11_1")),
        MatrixMagma::parse(include_str!("../db/11_2")),
        MatrixMagma::parse(include_str!("../db/11_3")),
    ]
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
