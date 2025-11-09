use crate::*;

type Complex = (usize, usize);

fn mul(x: Complex, y: Complex, p: usize) -> Complex {
    ((x.0*y.0 + (p*p - x.1*y.1))%p, (x.0*y.1 + x.1*y.0)%p)
}

fn add(x: Complex, y: Complex, p: usize) -> Complex {
    ((x.0 + y.0)%p, (x.1 + y.1)%p)
}

pub fn complex_search() {
    for p in 0.. {
        dbg!(p);
        for (c0, c1, c2, c3, c4, c5) in itertools::iproduct!(0..p, 0..p, 0..p, 0..p, 0..p, 0..p) {
            let f_def = |x: Complex, y: Complex| -> Complex {
                add(mul(x, (c0, c1), p), mul(y, (c2, c3), p), p)
            };

            let f = GenericMagma {
                elems: (0..p).map(move |x| (0..p).map(move |y| (x, y))).flatten().collect(),
                f_def,
            };
            if f.is677() {
                let m = f.to_matrix();
                present_model(p*p, "complex", |x, y| m.f(x, y));
            }
        }
    }
}

pub struct GenericMagma<E, F> {
    pub elems: Vec<E>,
    pub f_def: F,
}

impl<E, F> Magma for GenericMagma<E, F> where F: Fn(E, E) -> E, E: Copy + Eq {
    type Elem = E;

    fn elems(&self) -> impl Iterator<Item=E> { self.elems.iter().copied() }
    fn f(&self, x: E, y: E) -> E { (self.f_def)(x, y) }
}

impl<E, F> GenericMagma<E, F> where F: Fn(E, E) -> E, E: Copy + Eq {
    fn to_matrix(&self) -> MatrixMagma {
        MatrixMagma::by_fn(self.elems.len(), |x, y| {
            let x = self.elems[x];
            let y = self.elems[y];
            let z = self.f(x, y);
            self.elems.iter().position(|zz| *zz == z).unwrap()
        })
    }
}
