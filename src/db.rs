use crate::*;

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
fn db_canon() {
    for x in db() {
        assert_eq!(x.canonicalize(), x);
    }
}

#[test]
fn db_unique() {
    use std::collections::HashSet;

    let db = db();
    let n = db.len();
    let s: HashSet<_> = db.into_iter().collect();
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
