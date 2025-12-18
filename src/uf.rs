use crate::*;
use std::collections::HashMap;

type E = usize;
type E2 = (E, E);
type Map = HashMap<E2, E2>;

fn find(a: E2, map: &Map) -> E2 {
    if map[&a] == a { return a }
    find(map[&a], map)
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

pub fn uf(m: &MatrixMagma) -> Vec<Vec<E2>> {
    let mut map = HashMap::new();
    for x in 0..m.n {
        for y in 0..m.n {
            map.insert((x, y), (x, y));
        }
    }

    for x in 0..m.n {
        for y in 0..m.n {
            // f(y, f(x, f(f(y, x), y)))
            // a    b    c d

            let d = m.f(y, x);
            let c = m.f(d, y);
            let b = m.f(x, c);
            let a = m.f(y, b);
            merge((y, x), (d, y), &mut map);
            merge((y, x), (x, c), &mut map);
            merge((y, x), (y, b), &mut map);
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
        let uf: Vec<Vec<E2>> = uf(&m);

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
            if mag.is677() {
                present_model(mag.n, "kk", |x, y| mag.f(x, y));
            }
        }
    }
}
