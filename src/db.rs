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

        // size 11
        MatrixMagma::parse("
            0  2  3  4  5  6  7  8  9  10 1
            2  1  5  10 9  7  0  3  6  8  4
            3  5  2  6  1  10 8  0  4  7  9
            4  10 6  3  7  2  1  9  0  5  8
            5  9  1  7  4  8  3  2  10 0  6
            6  7  10 2  8  5  9  4  3  1  0
            7  0  8  1  3  9  6  10 5  4  2
            8  3  0  9  2  4  10 7  1  6  5
            9  6  4  0  10 3  5  1  8  2  7
            10 8  7  5  0  1  4  6  2  9  3
            1  4  9  8  6  0  2  5  7  3  10
        "),

        MatrixMagma::parse("
            0  6  7  8  9  10 2  3  4  5  1
            2  1  6  4  0  7  8  10 5  3  9
            3  8  2  7  5  0  10 9  6  1  4
            4  0  9  3  8  1  5  6  10 7  2
            5  2  0  10 4  9  3  1  7  6  8
            1  10 3  0  6  5  9  4  2  8  7
            7  5  4  1  10 8  6  2  9  0  3
            8  9  1  5  2  6  4  7  3  10 0
            9  7  10 2  1  3  0  5  8  4  6
            10 4  8  6  3  2  7  0  1  9  5
            6  3  5  9  7  4  1  8  0  2  10
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

#[test]
fn db_left_cancellative() {
    for m in db() {
        for ((a,b), c) in (0..m.n).zip(0..m.n).zip(0..m.n) {
            // a*b = a*c -> b = c.
            assert!(m.f(a, b) != m.f(a, c) || b == c);
        }
    }
}

#[test]
fn db_right_cancellative() {
    for m in db() {
        for ((a,b), c) in (0..m.n).zip(0..m.n).zip(0..m.n) {
            // b*a = c*a -> b = c
            assert!(m.f(b, a) != m.f(c, a) || b == c);
        }
    }
}
