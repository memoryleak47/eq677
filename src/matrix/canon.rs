use crate::*;

type Perm = Vec<usize>;

// f2(x, y) = c^-1(f1(c(x), c(y)))
// c: M2 (canonical) -> M1 (original).

impl MatrixMagma {
    pub fn transpose(&self) -> Self {
        let mut m = MatrixMagma::zeros(self.n);
        for x in 0..self.n {
            for y in 0..self.n {
                m.set_f(y, x, self.f(x, y));
            }
        }
        m
    }

    pub fn permute(&self, c: Perm) -> Self {
        let mut m = Self::zeros(self.n);
        for x in 0..self.n {
            for y in 0..self.n {
                // recall: f2(x, y) = c^-1(f1(c(x), c(y)))
                let cx = idx(&c, x);
                let cy = idx(&c, y);

                assert!(cx < self.n);
                assert!(cy < self.n);

                let fxy = self.f(cx, cy);
                let mut z = usize::MAX;
                if fxy != usize::MAX {
                    z = idx_rev(&c, fxy);
                    assert!(z != usize::MAX);
                }
                m.set_f(x, y, z);
            }
        }
        m
    }

    // This function also works on partial magmas.
    pub fn canonicalize(&self) -> Self {
        let n = self.n;
        let start_perm = vec![usize::MAX; n];

        let mut candidates = vec![start_perm];
        for y in 0..n {
            // define c(y).
            for c in std::mem::take(&mut candidates) {
                candidates.extend(choose_c(c, y));
            }

            for x in 0..n {
                // define c(x).
                for c in std::mem::take(&mut candidates) {
                    candidates.extend(choose_c(c, x));
                }

                // define c^-1(f1(c(x), c(y))).
                for c in std::mem::take(&mut candidates) {
                    let cx = idx(&c, x);
                    let cy = idx(&c, y);

                    assert!(cx != usize::MAX);
                    assert!(cy != usize::MAX);

                    let cz = self.f(cx, cy);
                    // missing elements simply remain missing.
                    if cz == usize::MAX {
                        candidates.push(c);
                    } else {
                        candidates.extend(choose_c_rev(c, cz));
                    }
                }

                // filter out suboptimal partial perms.
                let mut optimal = usize::MAX;
                for c in std::mem::take(&mut candidates) {
                    let cx = idx(&c, x);
                    let cy = idx(&c, y);

                    assert!(cx != usize::MAX);
                    assert!(cy != usize::MAX);

                    let cz = self.f(cx, cy);

                    let mut z = usize::MAX;
                    if cz != usize::MAX {
                        z = idx_rev(&c, cz);
                        assert!(z != usize::MAX);
                    }
                    if z <= optimal {
                        if z < optimal {
                            candidates.clear();
                            optimal = z;
                        }
                        candidates.push(c);
                    }
                }
            }
        }

        let candidate = candidates.pop().unwrap();
        self.permute(candidate)
    }

    pub fn canonicalize_terrible(&self) -> Self {
        all_perms(self.n).into_iter().map(|p| self.permute(p)).min().unwrap()
    }
}

// returns c(x)
fn idx(c: &[usize], x: usize) -> usize {
    assert!(x < c.len());
    c[x]
}

// returns c^-1(x)
fn idx_rev(c: &[usize], x: usize) -> usize {
    assert!(x < c.len());
    for y in 0..c.len() {
        if idx(c, y) == x { return y; }
    }
    usize::MAX
}

// after this function, `c(x)` is defined.
fn choose_c(c: Perm, x: usize) -> Vec<Perm> {
    assert!(x < c.len());

    if idx(&c, x) != usize::MAX { return vec![c] }

    let mut out = Vec::new();
    for o in 0..c.len() {
        if idx_rev(&c, o) != usize::MAX { continue }

        let mut c2 = c.clone();
        c2[x] = o;
        out.push(c2);
    }

    out
}

// after this function, `c^-1(x)` is defined.
fn choose_c_rev(c: Perm, x: usize) -> Vec<Perm> {
    assert!(x < c.len());

    if idx_rev(&c, x) != usize::MAX { return vec![c] }

    let mut out = Vec::new();
    for o in 0..c.len() {
        if idx(&c, o) != usize::MAX { continue }

        let mut c2 = c.clone();
        c2[o] = x;
        out.push(c2);
    }

    out
}

pub fn all_perms(n: usize) -> Vec<Perm> {
    if n == 0 { return vec![Vec::new()]; }

    let mut outs = Vec::new();

    // we decide 'out[0] = a'.
    for p in all_perms(n-1) {
        for a in 0..n {
            let mut out = Vec::new();
            out.push(a);
            out.extend(p.iter().copied().map(|x| x + (x >= a) as usize));
            outs.push(out);
        }
    }
    outs
}

#[test]
fn test_canon() {
    let m = MatrixMagma::parse("
        3 1 - - -
        2 3 4 - -
        3 - 2 - -
        - - - - -
        - - - - 2
    ");
    m.dump();
    println!("===================================");
    m.canonicalize().dump();
    println!("===================================");
    m.canonicalize_terrible().dump();

    for p in all_perms(5) {
        let m = m.permute(p);
        assert_eq!(m.canonicalize(), m.canonicalize_terrible());
    }
}
