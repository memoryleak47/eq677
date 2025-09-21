use crate::*;

pub fn db() -> Vec<MatrixMagma> {
    vec![
        // size 5
        MatrixMagma::parse("
            0 3 4 1 2
            2 1 0 4 3
            3 4 2 0 1
            4 2 1 3 0
            1 0 3 2 4
        "),

        // size 7
        MatrixMagma::parse("
            0 1 2 3 4 5 6
            2 4 1 5 3 6 0
            3 6 5 2 0 1 4
            1 3 4 6 5 0 2
            5 0 6 1 2 4 3
            6 2 0 4 1 3 5
            4 5 3 0 6 2 1
        "),

        MatrixMagma::parse("
            0 4 5 6 3 1 2
            2 0 3 4 6 5 1
            3 5 0 1 2 4 6
            1 2 6 0 4 3 5
            5 1 4 2 0 6 3
            6 3 2 5 1 0 4
            4 6 1 3 5 2 0
        "),

        // size 9
        MatrixMagma::parse("
            0 2 3 5 7 8 1 6 4
            1 3 6 7 4 0 8 2 5
            2 4 5 1 8 6 3 7 0
            3 5 7 8 0 2 6 4 1
            4 0 1 3 6 7 5 8 2
            5 1 8 6 2 4 7 0 3
            6 7 4 0 1 3 2 5 8
            7 8 0 2 3 5 4 1 6
            8 6 2 4 5 1 0 3 7
        "),
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
