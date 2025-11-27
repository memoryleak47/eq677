use crate::*;

include!(concat!(env!("OUT_DIR"), "/db_sources.rs"));

pub fn db_search() {
    for (n, m) in db() {
        let s = format!("db: {n}");
        present_model(m.n as usize, &s, |x, y| m.f(x, y));
    }
}

pub fn db_cart_search() {
    for (n1, m1) in db() {
        for (n2, m2) in db() {
            if m1.n * m2.n > 1000 { continue }

            let m = cartesian(&m1, &m2);
            let s = format!("db-cartesian: {n1} ⨯ {n2}");
            present_model(m.n, &s, |x, y| m.f(x, y));
        }
    }
}

pub fn db_get(n: &str) -> MatrixMagma {
    for (name, m) in DB_SOURCES {
        if *name == n {
            return MatrixMagma::parse(m);
        }
    }
    panic!("No magma with name {n} in db!");
}

pub fn db() -> Vec<(&'static str, MatrixMagma)> {
    let mut out = Vec::new();
    for (name, m) in DB_SOURCES {
        out.push((*name, MatrixMagma::parse(m)));
    }
    out
}

#[test]
fn db_canon() {
    for (name, m) in db() {
        if m.canonicalize2() != m {
            panic!("magma {name} is not canonicalized!");
        }
    }
}

#[test]
fn db_unique() {
    use std::collections::HashMap;

    let mut map: HashMap<MatrixMagma, &'static str> = HashMap::default();
    for (name, m) in db() {
        if let Some(name2) = map.insert(m, name.clone()) {
            panic!("Redundant Magmas {name} = {name2}");
        }
    }
}

#[test]
fn db_is677() {
    for (_, m) in db() {
        assert!(m.is677());
    }
}

#[test]
fn db_is255() {
    for (_, m) in db() {
        assert!(m.is255());
    }
}
