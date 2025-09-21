use crate::*;
use rayon::prelude::*;

mod init;
pub use init::*;

mod dump;
pub use dump::*;

type Map<K, V> = indexmap::IndexMap<K, V>;

type ElemId = usize;
type PosId = (usize, usize);

#[derive(Clone, Copy, PartialEq, Eq)]
struct TermId(usize);

type Res = Result<(), ()>;

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
    // ctxt.dump();
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
        // println!("decide ({}, {}) -> {}", pos.0, pos.1, e);

        let mut c = ctxt.clone();

        if let Ok(()) = propagate(pos, e, &mut c) {
            step(c);
        }
    }
}

struct Failure;

fn propagate(pos: PosId, e: ElemId, ctxt: &mut Ctxt) -> Res {
    let mut decisions = vec![(pos, e)];
    while let Some((pos, e)) = decisions.pop() {
        // eprintln!("prop ({}, {}) -> {}", pos.0, pos.1, e);
        if let Some(z) = ctxt.table.get(&pos) {
            if *z != e { return Err(()); }
            else { continue; }
        }

        if (0..ctxt.n).any(|z| ctxt.table.get(&(pos.0, z)) == Some(&e)) { return Err(()); }

        ctxt.table.insert(pos, e);
        let terms = ctxt.pos_terms[&pos].clone();

        for tid in terms {
            set_class(tid, e, ctxt, &mut decisions)?;
        }
    }
    Ok(())
}

fn set_class(t: TermId, v: ElemId, ctxt: &mut Ctxt, decisions: &mut Vec<(PosId, ElemId)>) -> Res {
    if ctxt.classes[t.0].value == Some(v) { return Ok(()); }
    assert!(ctxt.classes[t.0].value.is_none(), "Class set to different things?");

    ctxt.classes[t.0].value = Some(v);
    for parent in ctxt.classes[t.0].parents.clone() {
        visit_parent(parent, ctxt, decisions)?;
    }
    Ok(())
}

// Called when we've computed one of the children of "t".
fn visit_parent(t: TermId, ctxt: &mut Ctxt, decisions: &mut Vec<(PosId, ElemId)>) -> Res {
    match ctxt.classes[t.0].node {
        Node::F(x, y) => {
            let Some(x) = ctxt.classes[x.0].value else { return Ok(()) };
            let Some(y) = ctxt.classes[y.0].value else { return Ok(()) };
            if let Some(z) = ctxt.table.get(&(x, y)) {
                set_class(t, *z, ctxt, decisions)?;
            } else {
                for p in ctxt.classes[t.0].parents.clone() {
                    if let Node::AssertEq(v, _) = ctxt.classes[p.0].node {
                        decisions.push(((x, y), v));
                    }
                }

                if !ctxt.pos_terms[&(x, y)].contains(&t) { // TODO why is this check not always false?
                    ctxt.pos_terms[&(x, y)].push(t);
                }
            }
        },
        Node::AssertEq(l, r) => {
            let r = ctxt.classes[r.0].value.unwrap();
            if l != r {
                return Err(());
            }
        }
        Node::Elem(_) => unreachable!(),
    }
    Ok(())
}
