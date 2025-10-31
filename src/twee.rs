use crate::*;

use std::process::Command;
use std::fmt::Write;

fn run(elems: &[&str]) -> String {
    let a = Command::new(&elems[0]).args(&elems[1..])
        .output()
        .unwrap()
        .stdout;
    let a = String::from_utf8(a).unwrap();
    a
}

fn twee_input(m: &MatrixMagma) -> String {
    let mut s = String::from("
        cnf(a,axiom, X = f(Y, f(X, f(f(Y, X), Y)))).
        cnf(a,axiom, X = f(f(Y, X), f(f(Y, f(Y, X)), Y))).
        cnf(eq677,axiom, aaa != bbb). % looper

    ");
    for x in 0..m.n {
        for y in 0..m.n {
            let z = m.f(x, y);
            if z != usize::MAX {
                writeln!(&mut s, "cnf(a,axiom, f(e{x}, e{y}) = e{z} ).").unwrap();
            }
        }
    }
    s
}

pub fn twee(mut m: MatrixMagma) -> Option<MatrixMagma> {
    let f = "/tmp/eq677.p";

    let input = twee_input(&m);
    std::fs::write(f, input).unwrap();

    let out = run(&["twee", f, "--max-term-size", "20"]);
    let out = twee_parse(&out);

    for e in &out {
        if let (GTerm::E(_), GTerm::E(_)) = e { return None }

        if let (GTerm::F(b), GTerm::E(z)) = e && let [GTerm::E(x), GTerm::E(y)] = &**b {
            m.set_f(*x, *y, *z);
        }
    }

    Some(m)
}

#[derive(Debug)]
enum GTerm {
    F(Box<[GTerm; 2]>),
    E(usize),
}

fn twee_parse(s: &str) -> Vec<(GTerm, GTerm)> {
    let s = s.split("Here is the final rewrite system:\n").last().unwrap();
    let s = s.split("RESULT: GaveUp").next().unwrap();

    let mut out = Vec::new();

    for line in s.split("\n") {
        if line.contains("X") { continue }
        if line.trim().is_empty() { continue }

        let mut it = line.split("->");
        let a = parse_gterm(it.next().unwrap());
        let b = parse_gterm(it.next().unwrap());
        out.push((a, b));
    }

    out
}

fn parse_gterm(s: &str) -> GTerm {
    let s = s.replace(",", " , ").replace("(", " ( ").replace(")", " ) ");
    let toks = s.trim().split(" ").filter(|x| x.trim() != "").collect::<Vec<_>>();
    let (t, []) = assemble_gterm(&toks) else { panic!() };
    t
}

fn assemble_gterm<'a>(toks: &'a [&'a str]) -> (GTerm, &'a [&'a str]) {
    let t0 = toks[0];

    if t0.starts_with("e") {
        let e: usize = t0[1..].parse().unwrap();
        return (GTerm::E(e), &toks[1..]);
    }

    if let ["f", "(", toks@..] = toks {
        let (t1, toks) = assemble_gterm(toks);
        let [",", toks@..] = toks else { panic!() };
        let (t2, toks) = assemble_gterm(toks);
        let [")", toks@..] = toks else { panic!() };
        return (GTerm::F(Box::new([t1, t2])), toks);
    }

    panic!()
}
