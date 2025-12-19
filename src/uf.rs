use crate::*;
use std::collections::{HashMap, HashSet};

type E = usize;
type E2 = (E, E);
type Map = HashMap<E2, E2>;

fn find(a: E2, map: &Map) -> E2 {
    if map[&a] == a { return a }
    find(map[&a], map)
}

pub enum Equ {
    E255,
    E677,
}

fn merge(a: E2, b: E2, map: &mut Map) {
    let a = find(a, map);
    let b = find(b, map);
    if a < b {
        map.insert(a, b);
    } else {
        map.insert(b, a);
    }
}

pub fn uf(m: &MatrixMagma, eq: Equ) -> Vec<Vec<E2>> {
    let mut map = HashMap::new();
    for x in 0..m.n {
        for y in 0..m.n {
            map.insert((x, y), (x, y));
        }
    }

    for t in eq.traces(m) {
        let fst = t[0];
        for other in &t[1..] {
            merge(fst, *other, &mut map);
        }
    }

    let mut out = Vec::new();
    for x in 0..m.n {
        for y in 0..m.n {
            let xy = (x, y);
            let mut part = Vec::new();
            for a in 0..m.n {
                for b in 0..m.n {
                    let ab = (a, b);
                    if find(ab, &map) == xy {
                        part.push(ab);
                    }
                }
            }
            if !part.is_empty() { out.push(part); }
        }
    }

    out
}

fn pick_random(magmas: &[MatrixMagma]) -> MatrixMagma {
    use rand::prelude::*;

    let mut rng = rand::rng();
    let i: usize = rng.gen_range(0..magmas.len());
    magmas[i].clone()
}

pub fn uf_search() {
    for (_, m) in db() {
        if m.n < 2 { continue }
        if m.n > 50 { continue }
        let uf: Vec<Vec<E2>> = uf(&m, Equ::E677);

        for k in 2..(200/m.n+2) {
            let mut mag_opts = Vec::new();
            for (_, m) in db() {
                if m.n != k { continue }
                mag_opts.push(m);
            }
            if mag_opts.is_empty() { continue }

            let mut mags = Vec::new();
            for _ in 0..uf.len() {
                let m2 = pick_random(&mag_opts);
                let m2 = m2.shuffle();
                mags.push(m2);
            }

            let mag = GenericMagma {
                elems: itertools::iproduct!(0..m.n, 0..k).collect(),
                f_def: |x: E2, y: E2| {
                    let i = uf.iter().position(|u| u.contains(&(x.0, y.0))).unwrap();
                    let mm = &mags[i];

                    let z0 = m.f(x.0, y.0);
                    let z1 = mm.f(x.1, y.1);

                    (z0, z1)
                },
            }.to_matrix();
            assert!(mag.is677());
            present_model(mag.n, "kk", |x, y| mag.f(x, y));
        }
    }
}

pub fn partial_dump() {
    for (name, m) in db() {
        let mut set = HashSet::new();
        println!();
        println!("====================");
        println!("{name}");

        let stdin = std::io::stdin();
        stdin.read_line(&mut String::new()).unwrap();

        for x in 0..m.n {
            for y in 0..m.n {
                // These are boring:
                if x == y && m.f(x, x) == x { continue }

                let mm = partial_677_magma((x, y), &m);
                let mm = mm.canonicalize2();
                if set.insert(mm.clone()) {
                    mm.dump();
                    println!("---");
                }
            }
        }
    }
}

pub fn partial_677_magma(xy: E2, m: &MatrixMagma) -> MatrixMagma {
    let mut v = vec![xy];

    'outer: loop {
        let mut dirty = false;
        for (x, y) in v.clone() {
            for xy2 in trace677(m, x, y) {
                if !v.contains(&xy2) {
                    v.push(xy2);
                    dirty = true;
                }
            }
        }
        if !dirty { break }
    }

    let mut out = MatrixMagma::undefined(m.n);
    for (x, y) in v {
        let z = m.f(x, y);
        out.set_f(x, y, z);
    }

    out
}

// x*(y*((x*y)*x))
pub fn trace677(m: &MatrixMagma, x: E, y: E) -> [E2; 4] {
    let xy = m.f(x, y);
    let xyx = m.f(xy, x);
    let yxyx = m.f(y, xyx);
    [(x, y), (xy, x), (y, xyx), (x, yxyx)]
}

// ((x*x)*x)*x
pub fn trace255(m: &MatrixMagma, x: E) -> [E2; 3] {
    let xx = m.f(x, x);
    let xxx = m.f(xx, x);
    [(x, x), (xx, x), (xxx, x)]
}

impl Equ {
    // A trace always starts with the source pair, whose m.f-computation you are tracing.
    fn traces(self, m: &MatrixMagma) -> Vec<Vec<E2>> {
        let mut out = Vec::new();
        match self {
            Equ::E677 => {
                for x in 0..m.n {
                    for y in 0..m.n {
                        let v = trace677(m, x, y).into_iter().collect();
                        out.push(v);
                    }
                }
            },
            Equ::E255 => {
                for x in 0..m.n {
                    let v = trace255(m, x).into_iter().collect();
                    out.push(v);
                }
            },
        }
        out
    }
}
