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
    value: Option<ElemId>,
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
        // eprintln!("({}, {}) -> {}", pos.0, pos.1, e);
        if let Some(z) = ctxt.table.get(&pos) {
            if *z != e { return Some(Failure); }
            else { continue; }
            // TODO we could also raise a Failure if we have a duplicate per row!
        }
        ctxt.table.insert(pos, e);
        let terms = ctxt.pos_terms[&pos].clone();

        for tid in terms {
            ctxt.classes[tid].value = Some(e);
            for parent in ctxt.classes[tid].parents.clone() {
                if visit_parent(parent, ctxt, &mut decisions).is_some() {
                    return Some(Failure);
                }
            }
        }
    }
    None
}

// Called when we've computed one of the children of "t".
fn visit_parent(t: TermId, ctxt: &mut Ctxt, decisions: &mut Vec<(PosId, ElemId)>) -> Option<Failure> {
    match ctxt.classes[t].node {
        Node::F(x, y) => {
            let Some(x) = ctxt.classes[x].value else { return None };
            let Some(y) = ctxt.classes[y].value else { return None };
            if let Some(z) = ctxt.table.get(&(x, y)) {
                ctxt.classes[t].value = Some(*z);
                for p in ctxt.classes[t].parents.clone() {
                    if visit_parent(p, ctxt, decisions).is_some() {
                        return Some(Failure);
                    }
                }
            } else {
                for p in ctxt.classes[t].parents.clone() {
                    if let Node::AssertEq(v, _) = ctxt.classes[p].node {
                        decisions.push(((x, y), v));
                    }
                }

                ctxt.pos_terms[&(x, y)].extend(ctxt.classes[t].parents.clone());
            }
            None
        },
        Node::AssertEq(l, r) => {
            let r = ctxt.classes[r].value.unwrap();
            if l != r {
                return Some(Failure);
            } else {
                None
            }
        }
        Node::Elem(_) => unreachable!(),
    }
}
