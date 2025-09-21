use crate::*;

mod canon;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct MatrixMagma {
    n: usize, // size
    data: Box<[usize]>, // of length n^2
}

impl Magma for MatrixMagma {
    type Elem = usize;
    fn elems(&self) -> impl Iterator<Item=usize> {
        0..self.n
    }

    fn f(&self, x: usize, y: usize) -> usize {
        self.data[MatrixMagma::idx(x, y, self.n)]
    }
}

impl MatrixMagma {
    pub fn zeros(n: usize) -> Self {
        MatrixMagma {
            n,
            data: std::iter::repeat(0).take(n*n).collect(),
        }
    }

    pub fn by_fn(n: usize, f: impl Fn(usize, usize) -> usize) -> Self {
        let mut m = MatrixMagma::zeros(n);
        for x in 0..n {
            for y in 0..n {
                m.set_f(x, y, f(x, y));
            }
        }
        m
    }

    pub fn set_f(&mut self, x: usize, y: usize, val: usize) {
        self.data[MatrixMagma::idx(x, y, self.n)] = val;
    }

    fn idx(x: usize, y: usize, n: usize) -> usize {
        x + n * y
    }

    pub fn dump(&self) {
        for x in 0..self.n {
            for y in 0..self.n {
                print!("{} ", self.f(x, y));
            }
            println!("");
        }
    }
}
