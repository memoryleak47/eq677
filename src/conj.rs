use crate::*;

#[test]
fn test_db_conj() {
    for m in db() {
        conj(&m)
    }
}

pub fn conj(m: &MatrixMagma) {
    assert!(m.is677());
    assert!(m.is255());
    assert!(m.is_left_cancellative());

    conj_idempotence(m);
    conj_diag_orbit_size(m);
    conj_cycle_size(m);
    conj_cycle2(m);
    conj_cycles_divide_n(m);
    conj_cycles_summary(m);
    conj_unique_cycle_size(m);
    conj_bijective_or_constant(m);

    false_conj_odd(m);
    false_conj_right_cancellative(m);
    // false_conj_exists_idempotence(m);
}

// Conjectures:

// conj_cycles_summary is a stronger version of this.
fn conj_cycles_divide_n(m: &MatrixMagma) {
    if m.n % 7 == 0 { return } // Why %7?

    let mut s = 0;
    for x in 0..m.n {
        for z in 0..m.n {
            s += (c_mini(m, x, z) == z) as usize;
        }
    }
    assert!(s % m.n == 0);
}

fn conj_cycles_summary(m: &MatrixMagma) {
    if m.n % 7 == 0 { return } // Why %7?

    let a = c_summary(m, 0);
    for x in 1..m.n {
        let b = c_summary(m, x);
        assert_eq!(a, b);
    }
}

// This property seems often true, but not always true.
fn conj_unique_cycle_size(m: &MatrixMagma) {
    if m.n % 7 == 0 || m.n >= 95 { return } // Why %7?

    let mut out = Vec::new();
    for x in 0..m.n {
        out.extend(c_summary(m, x));
    }
    out.sort();
    out.retain(|x| *x >= 3);
    out.dedup();
    assert!(out.len() <= 1);
}

fn conj_idempotence(m: &MatrixMagma) {
    // Is there a more general statement here?
    let restrict = m.n == 5 || m.n == 11;

    assert!(!restrict || m.is_idempotent());
}

// TODO add conjecture stating that the size of the automorphism group is dependent only on n.

// Falsified conjectures:

fn conj_bijective_or_constant(m: &MatrixMagma) {
    // only applies to primitive models.
    if decompose(&m).len() > 0 { return }

    assert!(m.is_diag_bijective() || m.is_diag_constant());
}

fn false_conj_exists_idempotence(m: &MatrixMagma) {
    // This effectively states `exists x: x*x = x`.
    // It is intended to be run with canonicalized input.

    if m.n == 0 { return }

    for x in 0..m.n {
        if m.f(x, x) == x { return }
    }
    assert!(false);
}

fn false_conj_odd(m: &MatrixMagma) {
    assert!(m.n % 2 == 1 || m.n == 0);
}

fn false_conj_right_cancellative(m: &MatrixMagma) {
    assert!(m.is_right_cancellative());
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
        // Known values: 1, 3, 4, 5, 6, 7, 12, 18.
        // We know that 2 is impossible. 0*0 = 1 /\ 1*1 = 0 -> 0=1.
        assert!(i != 8);
    }
}

fn conj_cycle_size(m: &MatrixMagma) {
    for x in 0..m.n {
        for z in 0..m.n {
            let i = c(m, x, z);
            // It seems there are no 3-cycles?
            assert!(i < 3 || i > 5);
        }
    }
}

fn conj_cycle2(m: &MatrixMagma) {
    for x in 0..m.n {
        for y in 0..m.n {
            let a = m.f(x, y);
            if a == y { continue }

            let a = m.f(x, a);
            let a = m.f(x, a);
            assert!(a != y);
        }
    }
}

// Helpers:

// returns how often I need to left-multiply x onto z, until it becomes z again.
fn c(m: &MatrixMagma, x: usize, z: usize) -> u32 {
    let mut zz = z;
    let mut i = 0;
    loop {
        zz = m.f(x, zz);
        i += 1;
        if z == zz { break }
    }
    i
}

// Finds the minimal (i.e. canonical) element from the C(x, z) cycle.
fn c_mini(m: &MatrixMagma, x: usize, z: usize) -> usize {
    let mut zz = z;
    let mut mini = z;

    loop {
        zz = m.f(x, zz);
        if z == zz { break }
        if zz < mini { mini = zz; }
    }
    mini
}

fn c_summary(m: &MatrixMagma, x: usize) -> Vec<u32> {
    let mut v = Vec::new();
    for z in 0..m.n {
        // We only consider the representatives of each cycle!
        if c_mini(m, x, z) == z {
            v.push(c(m, x, z));
        }
    }
    v.sort();
    v
}

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
