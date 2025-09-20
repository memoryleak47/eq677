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
    table: Map<PosId, ElemId>,

    // maps each PosId to a set of terms that currently evaluate to this PosId (if you eval its children).
    pos_terms: Map<PosId, Vec<TermId>>,
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

        if propagate(pos, e, &mut c).is_none() {
            step(c);
        }
    }
}

struct Failure;

fn propagate(pos: PosId, e: ElemId, ctxt: &mut Ctxt) -> Option<Failure> {
    let mut decisions = vec![(pos, e)];
    while let Some((pos, e)) = decisions.pop() {
        ctxt.table.insert(pos, e);
        let terms = ctxt.pos_terms[&pos].clone();

        for tid in terms {
            for &parent in &ctxt.classes[tid].parents {
                // option 1: parent now evaluates to f(A, B).
                //    Then we should check whether it's part of AssertEq(Z, f(A, B)), and add a corresponding decision; otherwise
                //    we should just add it to pos_terms[(A, B)].
                // option 2: parent now evaluates to f(A, f(...)). Then we don't care?
                // Option 3: parent now evaluates to AssertEq(A, B); then check A=B. but is this even possible?
                check_parent(parent, ctxt, &mut decisions);
            }
        }
    }
    None
}

// Checks whether a parent is ready for compute
fn check_parent(t: TermId, ctxt: &Ctxt, decisions: &mut Vec<(PosId, ElemId)>) {
}
