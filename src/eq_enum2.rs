use crate::*;
use rayon::prelude::*;

type Map<K, V> = indexmap::IndexMap<K, V>;

type ElemId = usize;
type PosId = (usize, usize);

#[derive(Clone, Copy, PartialEq, Eq)]
struct TermId(usize);

type ConstraintId = usize;

#[derive(Clone)]
struct Constraint(ElemId, TermId);

#[derive(Clone, PartialEq, Eq)]
enum Term {
    Elem(ElemId),
    F(TermId, TermId),
}

type Table = Map<PosId, ElemId>;

#[derive(Clone, Copy)]
enum ThingId {
    Constraint(ConstraintId),
    Term(TermId),
}

#[derive(Clone, Default)]
struct Ctxt {
    constraints: Vec<Constraint>,
    terms: Vec<Term>, // indexed by TermId.
    parents: Vec<Vec<ThingId>>, // indexed by TermId.
    table: Table,
    n: usize,
}

pub fn eq_run(n: usize) {
    let mut ctxt = Ctxt::default();
    ctxt.n = n;
    build_constraints(n, &mut ctxt);
    step(ctxt);
}


fn step(mut ctxt: Ctxt) {
    let all_pos = (0..ctxt.n).map(|x| (0..ctxt.n).map(move |y| (x, y))).flatten();
    let mut free_pos = all_pos.filter(|xy| ctxt.table.get(xy).is_none());
    let Some(pos) = free_pos.next() else {
        let magma = MatrixMagma::by_fn(ctxt.n, |x, y| *ctxt.table.get(&(x, y)).unwrap());
        println!("Model found:");
        magma.dump();

        assert!(magma.is677());
        assert!(magma.is255());

        return; // We are done!
    };

    for e in 0..ctxt.n {
        if (0..ctxt.n).any(|z| ctxt.table.get(&(pos.0, z)) == Some(&e)) { continue }

        let mut c = ctxt.clone();

        c.table.insert(pos, e);

        if propagate(&mut c).is_none() {
            step(c);
        }
    }
}

struct Failure;

fn propagate(ctxt: &mut Ctxt) -> Option<Failure> {
    'start: loop {
        for &Constraint(l, tid) in ctxt.constraints.iter() {
            let Term::F(x, y) = ctxt.terms[tid.0] else { panic!() };
            let Some(x) = eval_term(x, ctxt) else { continue };
            let Some(y) = eval_term(y, ctxt) else { continue };
            if let Some(z) = ctxt.table.get(&(x, y)) {
                if *z != l { return Some(Failure); }
                else { continue }
            } else {
                ctxt.table.insert((x, y), l);
                continue 'start;
            }
        }
        break;
    }
    None
}

fn eval_term(tid: TermId, ctxt: &Ctxt) -> Option<ElemId> {
    match ctxt.terms[tid.0] {
        Term::Elem(e) => Some(e),
        Term::F(a, b) => {
            let a = eval_term(a, ctxt)?;
            let b = eval_term(b, ctxt)?;
            ctxt.table.get(&(a, b)).copied()
        },
    }
}

fn build_elem(e: ElemId, ctxt: &mut Ctxt) -> TermId {
    ctxt.terms.push(Term::Elem(e));
    TermId(ctxt.terms.len() - 1)
}

fn build_f(l: TermId, r: TermId, ctxt: &mut Ctxt) -> TermId {
    ctxt.terms.push(Term::F(l, r));
    let out = TermId(ctxt.terms.len() - 1);
    out
}

fn build_constraints(n: usize, ctxt: &mut Ctxt) {
    for x_id in 0..n {
        for y_id in 0..n {
            let x = build_elem(x_id, ctxt);
            let y = build_elem(y_id, ctxt);
            let yx = build_f(y, x, ctxt);

            let t = build_f(yx, y, ctxt);
            let t = build_f(x, t, ctxt);
            let t = build_f(y, t, ctxt);
            ctxt.constraints.push(Constraint(x_id, t));

            let t = build_f(y, yx, ctxt);
            let t = build_f(t, y, ctxt);
            let t = build_f(yx, t, ctxt);
            ctxt.constraints.push(Constraint(x_id, t));
        }
    }
}
