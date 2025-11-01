use crate::*;

mod canon;

mod canon2;
pub use canon2::*;

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

    pub fn count_defined(&self) -> usize {
        let mut c = 0;
        for x in 0..self.n {
            for y in 0..self.n {
                c += (self.f(x, y) != usize::MAX) as usize;
            }
        }
        c
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

    // Prints a magma, but displays every row x as the bijection spanned by
    // `y => x*y` written in cycle notation.
    pub fn cycle_dump(&self) {
        let ctxt = self;
        for x in 0..ctxt.n {
            print!("{x}: ");

            let mut seen = vec![false; ctxt.n as usize];
            for i in (x..ctxt.n).chain(0..x) {
                if seen[i] { continue }
                let mut cc = i;
                print!("(");
                loop {
                    print!("{cc}");
                    seen[cc] = true;
                    cc = ctxt.f(x, cc);
                    if cc == usize::MAX {
                        print!(" ...");
                        break
                    }
                    if seen[cc] { break }
                    print!(" ");
                }
                print!(") ");
            }
            println!();
        }
    }


    pub fn is_total(&self) -> bool {
        !self.data.contains(&usize::MAX)
    }
}

pub fn cartesian(m0: &MatrixMagma, m1: &MatrixMagma) -> MatrixMagma {
    MatrixMagma::by_fn(m0.n * m1.n, |x, y| {
        let (x0, x1) = (x%m0.n, x/m0.n);
        let (y0, y1) = (y%m0.n, y/m0.n);

        let (z0, z1) = (m0.f(x0, y0), m1.f(x1, y1));
        z0 + z1*m0.n
    })
}
