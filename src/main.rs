struct Magma {
    n: usize, // size
    data: Box<[usize]>, // of length n^2
}

impl Magma {
    fn zeros(n: usize) -> Self {
        Magma {
            n,
            data: std::iter::repeat(0).take(n*n).collect(),
        }
    }

    fn by_fn(n: usize, f: impl Fn(usize, usize) -> usize) -> Self {
        let mut m = Magma::zeros(n);
        for x in 0..n {
            for y in 0..n {
                m.set_f(x, y, f(x, y));
            }
        }
        m
    }

    fn idx(x: usize, y: usize, n: usize) -> usize {
        x + n * y
    }

    fn f(&self, x: usize, y: usize) -> usize {
        assert!(x < self.n);
        assert!(y < self.n);
        self.data[Magma::idx(x, y, self.n)]
    }

    fn set_f(&mut self, x: usize, y: usize, val: usize) {
        self.data[Magma::idx(x, y, self.n)] = val;
    }

    fn is667(&self) -> bool {
        for x in 0..self.n {
            for y in 0..self.n {
                if x != self.f(y, self.f(x, self.f(self.f(y, x), y))) {
                    return false;
                }
            }
        }
        true
    }

    fn is225(&self) -> bool {
        for x in 0..self.n {
            if x != self.f(self.f(self.f(x, x), x), x) {
                return false;
            }
        }
        true
    }
}

fn main() {
    let m = Magma::by_fn(5, |x, y| (2*x + 4*y) % 5);
    dbg!(m.is667());
    dbg!(m.is225());
    
}
