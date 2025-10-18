use crate::*;

pub fn db() -> Vec<MatrixMagma> {
    vec![
        // size 0
        MatrixMagma::parse(""),

        // size 1
        MatrixMagma::parse("0"),

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

        MatrixMagma::parse("
            0  3  4  7  8  10 9  6  5  2  1
            2  1  6  5  10 9  7  8  3  4  0
            1  5  2  9  6  8  10 4  7  0  3
            4  0  8  3  9  7  5  10 1  6  2
            3  7  0  10 4  6  8  2  9  1  5
            6  2  10 1  7  5  3  9  0  8  4
            5  9  1  8  2  4  6  0  10 3  7
            8  4  9  0  5  3  1  7  2  10 6
            7  10 3  6  0  2  4  1  8  5  9
            10 6  7  2  3  1  0  5  4  9  8
            9  8  5  4  1  0  2  3  6  7  10
        "),

        MatrixMagma::parse("
            0  6  7  8  9  10 4  5  1  2  3
            2  1  5  0  7  9  10 8  3  6  4
            3  10 2  1  0  8  5  6  9  4  7
            4  9  6  3  2  0  8  1  7  10 5
            5  0  10 7  4  3  1  9  2  8  6
            1  4  0  6  8  5  7  2  10 3  9
            7  8  4  9  3  1  6  10 0  5  2
            8  2  9  5  10 4  3  7  6  0  1
            9  5  3  10 1  6  2  4  8  7  0
            10 7  1  4  6  2  0  3  5  9  8
            6  3  8  2  5  7  9  0  4  1  10
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

// Helpers:

impl MatrixMagma {
    fn is_left_cancellative(&self) -> bool {
        for a in 0..self.n {
            for b in 0..self.n {
                for c in 0..self.n {
                    // a*b = a*c -> b = c.
                    if b != c && self.f(a, b) == self.f(a, c) { return false }
                }
            }
        }
        true
    }

    fn is_right_cancellative(&self) -> bool {
        for a in 0..self.n {
            for b in 0..self.n {
                for c in 0..self.n {
                    // b*a = c*a -> b = c
                    if b != c && self.f(b, a) == self.f(c, a) { return false }
                }
            }
        }
        true
    }

    fn is_diag_constant(&self) -> bool {
        for x in 0..self.n {
            for y in 0..self.n {
                if self.f(x, x) != self.f(y, y) { return false }
            }
        }
        true
    }

    fn is_diag_bijective(&self) -> bool {
        for x in 0..self.n {
            for y in 0..self.n {
                if x == y { continue }
                if self.f(x, x) == self.f(y, y) { return false }
            }
        }
        true
    }

    fn is_idempotent(&self) -> bool {
        for x in 0..self.n {
            let xx = self.f(x, x);
            if self.f(xx, xx) != xx { return false }
        }
        true
    }
}

#[test]
fn db_left_cancellative() {
    for m in db() {
        assert!(m.is_left_cancellative());
    }
}

// Conjectures:

#[test]
fn dbconj_right_cancellative() {
    // We know this conjecture is false.
    // But we haven't found a small model for it yet.

    for m in db() {
        assert!(m.is_right_cancellative());
    }
}

#[test]
fn dbconj_odd() {
    // We know this conjecture is false.
    // But we haven't found a small model for it yet.

    for m in db() {
        assert!(m.n % 2 == 1 || m.n == 0);
    }
}

#[test]
fn dbconj_bijective_or_constant() {
    for m in db() {
        assert!(m.is_diag_bijective() || m.is_diag_constant());
    }
}

#[test]
fn dbconj_idempotence() {
    for m in db() {
        // Is there a more general statement here?
        let restrict = m.n == 5 || m.n == 11;

        assert!(!restrict || m.is_idempotent());
    }
}

#[test]
fn dbconj_exists_idempotence() {
    // This effectively states `exists x: x*x = x`.

    for m in db() {
        if m.n == 0 { continue }
        assert!(m.f(0, 0) == 0);
    }
}

#[test]
fn dbconj_diag_orbit_size() {
    for m in db() {
        if !m.is_diag_bijective() { continue }

        for x in 0..m.n {
            let mut y = x;
            let mut i = 0;
            // i is the smallest positive number, s.t. S^i x = x, where S x = x*x
            loop {
                y = m.f(y, y);
                i += 1;
                if x == y { break }
            }
            dbg!(i);
            assert!(i == 1 || i == 6 || i == 4);
            // It looks like i can not be prime!
        }
    }
}

// TODO add conjecture stating that the size of the automorphism group is dependent only on n.
