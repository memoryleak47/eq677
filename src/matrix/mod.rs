use crate::*;

mod canon;

#[derive(Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
// This magma might be partial; we use usize::MAX to encoding missing entries.
pub struct MatrixMagma {
    pub n: usize, // size
    pub data: Box<[usize]>, // of length n^2
}

impl Magma for MatrixMagma {
    type Elem = usize;
    fn elems(&self) -> impl Iterator<Item=usize> {
        0..self.n
    }

    fn f(&self, x: usize, y: usize) -> usize {
        assert!(x < self.n);
        assert!(y < self.n);
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

    pub fn parse(s: &str) -> Self {
        let s = s.replace(",", " ");
        let s = s.trim();
        let mut lines: Vec<_> = s.lines().collect();

        let mut m = MatrixMagma::zeros(lines.len());

        for (x, line) in lines.into_iter().enumerate() {
            for (y, s) in line.split_whitespace().enumerate() {
                if s == "-" {
                    m.set_f(x, y, usize::MAX);
                } else {
                    let i: usize = s.parse().unwrap();
                    m.set_f(x, y, i);
                }
            }
        }

        m
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
        let div = self.n/10 + 2;
        for x in 0..self.n {
            for y in 0..self.n {
                let z = self.f(x, y);
                if z == usize::MAX {
                    print!("{:<width$}", '-', width = div);
                } else {
                    print!("{:<width$}", z, width = div);
                }
            }
            println!("");
        }
    }

    pub fn is_total(&self) -> bool {
        !self.data.contains(&usize::MAX)
    }
}
