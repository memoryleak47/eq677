use crate::*;
use z3::{ast::Int, *};
use std::fmt::Write;

pub fn tinv_chk2(m: &MatrixMagma) -> bool {
    let n = m.n;

    let mut s = String::new();
    writeln!(s, "(declare-fun phi (Int) Int)");
    writeln!(s, "(declare-fun f (Int Int) Int)");

    // define f.
    for x in 0..n {
        for y in 0..n {
            let z = m.f(x, y);
            writeln!(s, "(assert (= {z} (f {x} {y})))");
        }
    }

    // phi is only one big cycle:
    for x in 0..n {
        if x != 0 { continue }

        for i in 1..n {
            let mut st = format!("{x}");
            for _ in 0..i {
                st = format!("(phi {st})");
            }
            writeln!(s, "(assert (not (= {st} {x})))");
        }
    }

    // phi in range:
    for x in 0..n {
        writeln!(s, "(assert (>= (phi {x}) 0))");
        writeln!(s, "(assert (< (phi {x}) {n}))");
    }

    // phi injective.
    for x in 0..n {
        for y in 0..n {
            if x != y {
                writeln!(s, "(assert (not (= (phi {x}) (phi {y}))))");
            }
        }
    }

    // phi homomorphism:
    for x in 0..n {
        for y in 0..n {
            writeln!(s, "(assert (= (phi (f {x} {y})) (f (phi {x}) (phi {y}))))");
        }
    }

    writeln!(s, "(check-sat)");
    writeln!(s, "(get-model)");

    // println!("{s}");

    let solver = Solver::new();
    solver.from_string(s);

    matches!(solver.check(), SatResult::Sat)
}
