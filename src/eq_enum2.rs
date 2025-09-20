use crate::*;
use rayon::prelude::*;

type Map<K, V> = indexmap::IndexMap<K, V>;

type ElemId = usize;
type PosId = (usize, usize);
type TermId = usize;

#[derive(Clone)]
struct Constraint(ElemId, TermId);

#[derive(Clone, PartialEq, Eq)]
enum Term {
    Elem(ElemId),
    F(TermId, TermId),
}

type Table = Map<PosId, ElemId>;

#[derive(Clone)]
struct Ctxt {
    constraints: Vec<Constraint>,
    terms: Vec<Term>, // indexed by TermId.
    table: Table,
    n: usize,
}

pub fn eq_run(n: usize) {
    let mut terms = Vec::new();
    step(Ctxt {
        constraints: build_constraints(n, &mut terms),
        terms,
        table: Map::new(),
        n,
    });
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

        propagate(&mut c);
        step(c);
    }
}

struct Failure;

fn propagate(ctxt: &mut Ctxt) -> Option<Failure> {
    'start: loop {
        for &Constraint(l, tid) in ctxt.constraints.iter() {
            let Term::F(x, y) = ctxt.terms[tid] else { panic!() };
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
    }
    None
}

fn eval_term(tid: TermId, ctxt: &Ctxt) -> Option<ElemId> {
    match ctxt.terms[tid] {
        Term::Elem(e) => Some(e),
        Term::F(a, b) => {
            let a = eval_term(a, ctxt)?;
            let b = eval_term(b, ctxt)?;
            ctxt.table.get(&(a, b)).copied()
        },
    }
}

fn build_elem(e: ElemId, terms: &mut Vec<Term>) -> TermId {
    terms.push(Term::Elem(e));
    terms.len() - 1
}

fn build_f(l: TermId, r: TermId, terms: &mut Vec<Term>) -> TermId {
    terms.push(Term::F(l, r));
    terms.len() - 1
}

fn build_constraints(n: usize, terms: &mut Vec<Term>) -> Vec<Constraint> {
    let mut constraints = Vec::new();
    for x in 0..n {
        for y in 0..n {
            let x = build_elem(x, terms);
            let y = build_elem(y, terms);
            let mut f = |a, b| build_f(a, b, terms);
            let yx = f(y, x);

            let t = f(yx, y);
            let t = f(x, t);
            let t = f(y, t);
            constraints.push(Constraint(x, t));

            let t = f(y, yx);
            let t = f(t, y);
            let t = f(yx, t);
            constraints.push(Constraint(x, t));
        }
    }
    constraints
}
