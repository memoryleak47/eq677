use crate::*;

#[derive(Clone)]
struct Ctxt {
    eqs: Vec<(TTerm, TTerm)>,
    distincts: Vec<TTerm>,
    open: Vec<TTerm>,
}

pub fn one_orbit2_run() {
    let mut ctxts = vec![new_ctxt()];
    for _ in 0..100 {
        ctxts = ctxts.into_iter().map(branch_step).flatten().collect();
    }
}

fn branch_step(ctxt: Ctxt) -> Vec<Ctxt> {
    todo!()
}

fn new_ctxt() -> Ctxt {
    Ctxt {
        eqs: Vec::new(),
        distincts: Vec::new(),
        open: Vec::new(),
    }
}

fn twee_chk(ctxt: &Ctxt) -> bool {
    let mut eqs = Vec::new();
    let mut diseqs = Vec::new();

    {
        let x = mk_X();
        let y = mk_Y();

        // eq677
        let rhs = mk_f(&y, &mk_f(&x, &mk_f(&mk_f(&y, &x), &y)));
        eqs.push((x.clone(), rhs));

        // eq677 II
        let yx = mk_f(&y, &x);
        let rhs = mk_f(&yx, &mk_f(&mk_f(&y, &yx), &y));
        eqs.push((x, rhs));
    }

    // equalities
    for (a, b) in &ctxt.eqs {
        eqs.push((a.clone(), b.clone()));
    }

    // disequalities:
    for (i, a) in ctxt.distincts.iter().enumerate() {
        for (j, b) in ctxt.distincts.iter().enumerate() {
            if i < j {
                diseqs.push((a.clone(), b.clone()));
            }
        }
    }

    twee_call(&eqs, &diseqs, 50).is_ok()
}

// ========
// builders:
// ========

use egg::Symbol;

fn mk_f(x: &TTerm, y: &TTerm) -> TTerm {
    let f = Symbol::from("f");
    TTerm::Fun(f, Box::new([x.clone(), y.clone()]))
}

#[allow(non_snake_case)]
fn mk_X() -> TTerm {
    let x = Symbol::from("X");
    TTerm::Var(x)
}

#[allow(non_snake_case)]
fn mk_Y() -> TTerm {
    let y = Symbol::from("Y");
    TTerm::Var(y)
}

