use crate::*;
use rayon::prelude::*;

type Map<K, V> = indexmap::IndexMap<K, V>;

type ElemId = usize;
type PosId = (usize, usize);
type TermId = usize;

#[derive(Clone, PartialEq, Eq)]
enum Node {
    Elem(ElemId),
    F(TermId, TermId),
    AssertEq(ElemId, TermId),
}

type Table = Map<PosId, ElemId>;

#[derive(Clone, Default)]
struct Ctxt {
    constraints: Vec<TermId>,
    terms: Vec<Node>, // indexed by TermId.
    parents: Vec<Vec<TermId>>, // indexed by TermId.
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
        for &c in ctxt.constraints.iter() {
            let Node::AssertEq(l, tid) = ctxt.terms[c] else { panic!() };
            let Node::F(x, y) = ctxt.terms[tid] else { panic!() };
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
    match ctxt.terms[tid] {
        Node::Elem(e) => Some(e),
        Node::F(a, b) => {
            let a = eval_term(a, ctxt)?;
            let b = eval_term(b, ctxt)?;
            ctxt.table.get(&(a, b)).copied()
        },
        Node::AssertEq(_, _) => panic!(),
    }
}

fn build_elem(e: ElemId, ctxt: &mut Ctxt) -> TermId {
    ctxt.terms.push(Node::Elem(e));
    ctxt.parents.push(Vec::new());
    ctxt.terms.len() - 1
}

fn build_f(l: TermId, r: TermId, ctxt: &mut Ctxt) -> TermId {
    ctxt.terms.push(Node::F(l, r));
    ctxt.parents.push(Vec::new());
    let out = ctxt.terms.len() - 1;
    ctxt.parents[l].push(out);
    ctxt.parents[r].push(out);
    out
}

fn build_assert(l: ElemId, r: TermId, ctxt: &mut Ctxt) {
    ctxt.terms.push(Node::AssertEq(l, r));
    ctxt.parents.push(Vec::new());
    let out = ctxt.terms.len() - 1;
    ctxt.parents[r].push(out);
    ctxt.constraints.push(out);
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
            build_assert(x_id, t, ctxt);

            let t = build_f(y, yx, ctxt);
            let t = build_f(t, y, ctxt);
            let t = build_f(yx, t, ctxt);
            build_assert(x_id, t, ctxt);
        }
    }
}
