use crate::*;

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
