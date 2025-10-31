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

pub fn twee(m: &MatrixMagma) -> MatrixMagma {
    let f = run(&["mktemp", "-d"]);
    let f = f.trim();
    let f = format!("{f}/file.p");

    let input = twee_input(m);
    std::fs::write(&f, input).unwrap();

    let out = run(&["twee", &f, "--max-term-size", "20"]);
    let out = out.split("Here is the final rewrite system:\n").last().unwrap();
    let out = out.split("RESULT: GaveUp").next().unwrap();

    println!("{out}");

    todo!()
}

pub fn twee_testing() {
    let m = MatrixMagma::parse("
        0 2 1 - -
        - - - - -
        - - - - -
        - - - - -
        - - - - -
    ");
    twee(&m);
}
