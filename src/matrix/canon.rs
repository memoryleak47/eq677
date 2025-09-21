use crate::*;

type Perm = Vec<usize>;

impl MatrixMagma {
    pub fn canonicalize(&self) -> Self {
        all_perms(self.n).into_iter().map(|p| self.permute(p)).min().unwrap()
    }

    pub fn transpose(&self) -> Self {
        let mut m = MatrixMagma::zeros(self.n);
        for x in 0..self.n {
            for y in 0..self.n {
                m.set_f(y, x, self.f(x, y));
            }
        }
        m
    }

    fn permute(&self, p: Perm) -> Self {
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
}

fn all_perms(n: usize) -> Vec<Perm> {
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
