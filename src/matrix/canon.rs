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

    pub fn permute(&self, p: Perm) -> Self {
        let mut c = Self::zeros(self.n);
        for x in 0..self.n {
            for y in 0..self.n {
                // f(x, y) = z
                // -> f(p[x], p[y]) = p[z]
                let z = self.f(x, y);
                c.set_f(p[x], p[y], p[z]);
            }
        }
        c
    }

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
                    candidates.extend(choose_c_rev(c, cz));
                }

                // filter out suboptimal partial perms.
                let mut optimal = usize::MAX;
                for c in std::mem::take(&mut candidates) {
                    let cx = idx(&c, x);
                    let cy = idx(&c, y);
                    let cz = self.f(cx, cy);
                    let z = idx_rev(&c, cz);
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
}

// returns c(x)
fn idx(c: &[usize], x: usize) -> usize {
    c[x]
}

// returns c^-1(x)
fn idx_rev(c: &[usize], x: usize) -> usize {
    for y in 0..c.len() {
        if idx(c, y) == x { return y; }
    }
    usize::MAX
}

// after this function, `c(x)` is defined.
fn choose_c(c: Vec<usize>, x: usize) -> Vec<Perm> {
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
fn choose_c_rev(c: Vec<usize>, x: usize) -> Vec<Perm> {
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
