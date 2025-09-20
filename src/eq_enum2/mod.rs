use crate::*;
use rayon::prelude::*;

mod init;
pub use init::*;

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

#[derive(Clone)]
struct Class {
    node: Node,
    parents: Vec<TermId>,
}

#[derive(Clone, Default)]
struct Ctxt {
    classes: Vec<Class>, // indexed by TermId
    constraints: Vec<TermId>,
    table: Map<PosId, ElemId>,
    n: usize,
}

pub fn eq_run(n: usize) {
    step(build_ctxt(n));
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
            let Node::AssertEq(l, tid) = ctxt.classes[c].node else { panic!() };
            let Node::F(x, y) = ctxt.classes[tid].node else { panic!() };
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
    match ctxt.classes[tid].node {
        Node::Elem(e) => Some(e),
        Node::F(a, b) => {
            let a = eval_term(a, ctxt)?;
            let b = eval_term(b, ctxt)?;
            ctxt.table.get(&(a, b)).copied()
        },
        Node::AssertEq(_, _) => panic!(),
    }
}
