use crate::*;

use std::collections::{HashMap, HashSet};

fn parse_all(s: String) -> Vec<MatrixMagma> {
    let mut out = Vec::new();

    let mut s = s;

    let mut current = String::new();
    for line in s.split("\n") {
        if line.chars().all(|x| x.is_whitespace() || x.is_digit(10)) {
            current.push_str(&line);
            current.push('\n');
        } else {
            current = current.trim().to_string();
            if !current.is_empty() {
                out.push(MatrixMagma::parse(&current));
                current = String::new();
            }
        }
    }
    out
}

pub fn load_file(file: &str) -> HashSet<MagmaName> {
    let mut out = HashSet::new();

    let s = std::fs::read_to_string(file).unwrap();
    for m in parse_all(s) {
        let (n, new) = db_intern(&m);
        if new {
            println!("New magma of size {}!", m.n);
            m.dump();
        }
    }
    out
}
