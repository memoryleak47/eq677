use crate::*;

#[test]
fn test_db_conj() {
    for m in db() {
        conj(&m)
    }
}

pub fn conj(m: &MatrixMagma) {
    conj_left_cancellative(m);
    conj_idempotence(m);
    conj_diag_orbit_size(m);
}

// Conjectures:

// This is actually a trivially true conjecture. Just for testing.
fn conj_left_cancellative(m: &MatrixMagma) {
    assert!(m.is_left_cancellative());
}

fn conj_idempotence(m: &MatrixMagma) {
    // Is there a more general statement here?
    let restrict = m.n == 5 || m.n == 11;

    assert!(!restrict || m.is_idempotent());
}


fn conj_diag_orbit_size(m: &MatrixMagma) {
    if !m.is_diag_bijective() { return }

    for x in 0..m.n {
        let mut y = x;
        let mut i = 0;
        // i is the smallest positive number, s.t. S^i x = x, where S x = x*x
        loop {
            y = m.f(y, y);
            i += 1;
            if x == y { break }
        }
        assert!(i == 1 || i == 6 || i == 4);
        // It looks like i can not be prime!
    }
}

// TODO add conjecture stating that the size of the automorphism group is dependent only on n.

// Falsified conjectures:

fn false_conj_bijective_or_constant(m: &MatrixMagma) {
    assert!(m.is_diag_bijective() || m.is_diag_constant());
}

fn false_conj_exists_idempotence(m: &MatrixMagma) {
    // This effectively states `exists x: x*x = x`.
    // It is intended to be run with canonicalized input.

    if m.n == 0 { return }
    assert!(m.f(0, 0) == 0);
}

fn false_conj_odd(m: &MatrixMagma) {
    assert!(m.n % 2 == 1 || m.n == 0);
}

fn false_conj_right_cancellative(m: &MatrixMagma) {
    assert!(m.is_right_cancellative());
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
