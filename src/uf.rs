use crate::*;
use std::collections::{HashMap, HashSet};

type E = usize;
type E2 = (E, E);
type Map = HashMap<E2, E2>;

pub fn find(a: E2, map: &Map) -> E2 {
    if map[&a] == a { return a }
    find(map[&a], map)
}

pub enum Equ {
    E255,
    E677,
}

pub fn merge(a: E2, b: E2, map: &mut Map) {
    let a = find(a, map);
    let b = find(b, map);
    if a < b {
        map.insert(a, b);
    } else {
        map.insert(b, a);
    }
}

pub fn init_uf(n: usize) -> Map {
    let mut map = HashMap::new();
    for p in e2_iter(n) {
        map.insert(p, p);
    }
    map
}

pub fn uf_to_vecs(n: usize, map: &Map) -> Vec<Vec<E2>> {
    let mut out = Vec::new();
    for xy in e2_iter(n) {
        let mut part = Vec::new();
        for ab in e2_iter(n) {
            if find(ab, &map) == xy {
                part.push(ab);
            }
        }
        if !part.is_empty() { out.push(part); }
    }
    out
}

pub fn uf(m: &MatrixMagma, eq: Equ) -> Vec<Vec<E2>> {
    let mut map = init_uf(m.n);

    for t in eq.traces(m) {
        let fst = t[0];
        for other in &t[1..] {
            merge(fst, *other, &mut map);
        }
    }

    uf_to_vecs(m.n, &map)
}

fn e2_iter(n: usize) -> impl Iterator<Item=E2> + 'static {
    (0..n).flat_map(move |x| (0..n).map(move |y| (x, y)))
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

pub fn partial_completion_search() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    for (name, m) in db() {
        println!("====================");
        println!("{name}:");
        println!("====================");
        for x in 0..m.n {
            for y in 0..m.n {
                // These are boring:
                if x == y && m.f(x, x) == x { continue }

                let mm = partial_677_magma((x, y), &m);
                if mm.count_defined() > m.n*m.n/2 {
                    let mm = mm.canonicalize2();
                    if set.insert(mm.clone()) {
                        c_complete(&mm);
                    }
                }
            }
        }
    }
}

pub fn partial_dump_magma(m: &MatrixMagma) {
    let mut set = HashSet::new();

    for x in 0..m.n {
        for y in 0..m.n {
            // These are boring:
            if x == y && m.f(x, x) == x { continue }

            let mm = partial_677_magma((x, y), &m);
            let mm = shrink_partial_to_fit(&mm);
            let mm = mm.canonicalize2();
            if set.insert(mm.clone()) {
                println!("({x}, {y}) generate:");
                mm.dump();
            }
        }
    }
}

pub fn partial_dump_db() {
    for (name, m) in db() {
        println!();
        println!("====================");
        println!("{name}:");
        println!("====================");

        partial_dump_magma(&m);
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

// Finds the elements that are actually used in this magma.
pub fn elements_of_partial(m: &MatrixMagma) -> Vec<usize> {
    let mut v = Vec::new();
    for x in 0..m.n {
        for y in 0..m.n {
            let z = m.f(x, y);
            if z != usize::MAX {
                if !v.contains(&x) { v.push(x); }
                if !v.contains(&y) { v.push(y); }
                if !v.contains(&z) { v.push(z); }
            }
        }
    }
    v
}

pub fn shrink_partial_to_fit(m: &MatrixMagma) -> MatrixMagma {
    let v = elements_of_partial(m);
    GenericMagma {
        elems: v,
        f_def: |x, y| m.f(x, y),
    }.to_matrix()
}

// Constructs a partial model db.
pub fn partial_db() -> HashSet<MatrixMagma> {
    let mut set: HashSet<MatrixMagma> = HashSet::new();
    for (name, m) in db() {
        println!("Looking in {name}:");
        for x in 0..m.n {
            for y in 0..m.n {
                let mm = partial_677_magma((x, y), &m);
                let mm = shrink_partial_to_fit(&mm);
                let mm = mm.canonicalize2();
                if set.insert(mm.clone()) {
                    println!("size={}, defined={}:", mm.n, mm.count_defined());
                    mm.dump();
                }
            }
        }
    }
    set
}

pub fn rebuild_c_classes(m: &MatrixMagma, map: &mut Map) {
    loop {
        let mut dirty = false;
        for p in e2_iter(m.n) {
            let r = find(p, map);
            let t1 = trace677(m, p.0, p.1);
            let t2 = trace677(m, r.0, r.1);
            for (p1, p2) in t1.into_iter().zip(t2.into_iter()) {
                if find(p1, map) != find(p2, map) {
                    dirty = true;
                    merge(p1, p2, map);
                }
            }
        }
        if !dirty { break }
    }
}

fn get_colors() -> Vec<String> {
    let mut combinations = Vec::new();

    // Define the valid color range (excluding the black codes identified previously)
    let valid_indices: Vec<u8> = (0..=255)
        .filter(|&i| !matches!(i, 0 | 8 | 16 | 232..=236))
        .collect();

    for &fg in &valid_indices {
        for &bg in &valid_indices {
            // Skip if foreground and background are the same
            if fg == bg {
                continue;
            }
            // Format: \x1b[38;5;FG;48;5;BGm
            combinations.push(format!("\x1b[38;5;{};48;5;{}m", fg, bg));
        }
    }

    shuf(0, &mut combinations);
    combinations
}

fn leaders(n: usize, uf: &Map) -> Vec<E2> {
    e2_iter(n).filter(|x| *x == find(*x, uf)).collect()
}

pub fn colored_dump(m: &MatrixMagma, map: &Map) {
    let mut div = m.n.ilog10() as usize + 2;
    if FIXED_WIDTH { div = div.max(3); }

    let colors = get_colors();
    let l = leaders(m.n, map);
    let mut colormap = HashMap::new();
    if l.len() > colors.len() {
        println!("Warning: duplicate color usage!");
    }
    for (i, ll) in l.iter().enumerate() {
        colormap.insert(ll, colors[i%colors.len()].clone());
    }

    for x in 0..m.n {
        for y in 0..m.n {
            let z = m.f(x, y);
            let ll = find((x, y), &map);
            print!("{}", &colormap[&ll]);
            if z == usize::MAX {
                print!("{:<width$}", '-', width = div);
            } else {
                print!("{:<width$}", fmt(z), width = div);
            }
            print!("\x1b[0m");
        }
        println!();
    }
}

fn shuf<T>(seed: u8, v: &mut Vec<T>) {
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    let seed = [seed; 32]; // 32-byte seed
    let mut rng = StdRng::from_seed(seed);
    v.shuffle(&mut rng);
}

pub fn random_classes(m: &MatrixMagma) -> Map {
    let mut uf = init_uf(m.n);
    rebuild_c_classes(&m, &mut uf);

    'outer: for i in 0.. {
        dbg!(i);
        let mut lv1 = leaders(m.n, &uf);
        let mut lv2 = lv1.clone();
        shuf(0, &mut lv1);
        shuf(2, &mut lv2);

        for l1 in lv1.iter() {
            for l2 in lv2.iter() {
                if l1 == l2 { continue }
                let mut uf2 = uf.clone();
                merge(*l1, *l2, &mut uf2);
                rebuild_c_classes(&m, &mut uf2);
                if leaders(m.n, &uf2).len() > m.n + 5 {
                    uf = uf2;
                    continue 'outer;
                }
            }
        }
        println!("Stopped iteration with i={i}");
        break;
    }

    uf
}

fn useful_classes_from(c1: E2, c2: E2, m: &MatrixMagma) -> Vec<Map> {
    let mut uf = init_uf(m.n);
    for p in e2_iter(m.n) {
        if p == c2 { break }
        merge(c1, p, &mut uf);
    }
    rebuild_c_classes(m, &mut uf);
    if find(c1, &uf) == find(c2, &uf) { return Vec::new() }

    let mut opts = vec![uf];
    for p in e2_iter(m.n) {
        for map in std::mem::take(&mut opts) {
            let p_lead = find(p, &map);
            if p_lead == find(c1, &map) || p_lead == find(c2, &map) {
                opts.push(map);
                continue
            }
            for o in [c1, c2] {
                let mut map = map.clone();
                merge(o, p, &mut map);
                rebuild_c_classes(m, &mut map);
                if find(c1, &map) != find(c2, &map) {
                    opts.push(map);
                }
            }
        }
    }
    opts
}

pub fn useful_classes(m: &MatrixMagma) -> Vec<Map> {
    let mut opts = vec![];
    let c1 = (0, 0);
    for c2 in e2_iter(m.n) {
        if c1 == c2 { continue }
        opts.extend(useful_classes_from(c1, c2, m));
    }

    opts
}
