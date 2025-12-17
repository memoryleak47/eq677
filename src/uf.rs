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
