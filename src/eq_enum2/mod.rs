use crate::*;
use rayon::prelude::*;

// TODO things to still implement:
// - trail (instead of cloning)

mod init;
pub use init::*;

mod dump;
pub use dump::*;

type Map<K, V> = indexmap::IndexMap<K, V>;

type ElemId = usize;
type PosId = (usize, usize);

enum TrailEvent {
    DecisionPoint(PosId, Vec<ElemId>), // We set the PosId to something. If this fails, take the next thing from the vector and try that.
    TableStore(PosId), // ctxt.table.insert(pos, _);
    PosTermsPush(PosId), // ctxt.pos_terms[pos].push(_);
    ValueSet(TermId), // ctxt.classes[i].value = Some(_);
    Defresh(ElemId), // ctxt.fresh[i] = false;
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct TermId(usize);

type Res = Result<(), ()>;

#[derive(PartialEq, Eq)]
enum Node {
    Elem(ElemId),
    F(TermId, TermId),
    AssertEq(ElemId, TermId),
}

struct Class {
    node: Node,
    parents: Vec<TermId>,
    value: Option<ElemId>,
}

#[derive(Default)]
enum Mode {
    #[default] Forward,
    Backtracking,
    Done,
}

#[derive(Default)]
struct Ctxt {
    trail: Vec<TrailEvent>,
    classes: Vec<Class>, // indexed by TermId
    table: Map<PosId, ElemId>,

    // maps each PosId to a set of terms that currently evaluate to this PosId (if you eval its children).
    pos_terms: Map<PosId, Vec<TermId>>,

    n: usize,
    mode: Mode,

    fresh: Vec<bool>, // whether an ElemId is still "fresh".
}

pub fn eq_run2(n: usize) {
    let ctxt = &mut build_ctxt(n);

    loop {
        match ctxt.mode {
            Mode::Forward => forward_step(ctxt),
            Mode::Backtracking => backtrack_step(ctxt),
            Mode::Done => break,
        }
    }
}

fn forward_step(ctxt: &mut Ctxt) {
    let Some((pos, options)) = next_options(ctxt) else {
        print_model(ctxt);
        ctxt.mode = Mode::Backtracking;
        return;
    };

    activate_option(pos, options, ctxt);
}

fn backtrack_step(ctxt: &mut Ctxt) {
    if ctxt.trail.is_empty() {
        ctxt.mode = Mode::Done;
        return;
    }
    match ctxt.trail.pop().unwrap() {
        TrailEvent::DecisionPoint(pos, options) => {
            activate_option(pos, options, ctxt);
            return;
        },
        TrailEvent::TableStore(pos) => { ctxt.table.remove(&pos); },
        TrailEvent::PosTermsPush(pos) => { ctxt.pos_terms[&pos].pop(); },
        TrailEvent::ValueSet(tid) => { ctxt.classes[tid.0].value = None; },
        TrailEvent::Defresh(e) => { ctxt.fresh[e] = true; },
    }
}

fn print_model(ctxt: &Ctxt) {
    let magma = MatrixMagma::by_fn(ctxt.n, |x, y| *ctxt.table.get(&(x, y)).unwrap());
    println!("Model found:");
    magma.dump();
    // ctxt.dump();

    assert!(magma.is677());
    assert!(magma.is255());
}

fn next_options(ctxt: &mut Ctxt) -> Option<(PosId, Vec<ElemId>)> {
    let all_pos = (0..ctxt.n).map(|x| (0..ctxt.n).map(move |y| (x, y))).flatten();
    let mut free_pos = all_pos.filter(|xy| ctxt.table.get(xy).is_none());
    let pos = free_pos.max_by_key(|pos| ctxt.pos_terms[pos].len())?;

    let mut found_fresh = false;

    if ctxt.fresh[pos.0] {
        ctxt.fresh[pos.0] = false;
        ctxt.trail.push(TrailEvent::Defresh(pos.0));
    }
    if ctxt.fresh[pos.1] {
        ctxt.fresh[pos.1] = false;
        ctxt.trail.push(TrailEvent::Defresh(pos.1));
    }

    let mut valids = Vec::new();
    for e in 0..ctxt.n {
        if ctxt.fresh[e] {
            // If we already used a "fresh" ElemIdx, no reason to do the same operation for another fresh one!
            if found_fresh { continue }
            else { found_fresh = true; }
        }

        if (0..ctxt.n).any(|z| ctxt.table.get(&(pos.0, z)) == Some(&e)) { continue }

        valids.push(e);
    }

    Some((pos, valids))
}

fn activate_option(pos: PosId, mut options: Vec<ElemId>, ctxt: &mut Ctxt) {
    let Some(e) = options.pop() else {
        ctxt.mode = Mode::Backtracking;
        return;
    };

    if ctxt.fresh[e] {
        ctxt.fresh[e] = false;
        ctxt.trail.push(TrailEvent::Defresh(e));
    }

    ctxt.mode = Mode::Forward;
    ctxt.trail.push(TrailEvent::DecisionPoint(pos, options));
    if let Err(()) = propagate(pos, e, ctxt) {
        ctxt.mode = Mode::Backtracking;
    }
}

fn propagate(pos: PosId, e: ElemId, ctxt: &mut Ctxt) -> Res {
    let mut decisions = vec![(pos, e)];
    while let Some((pos, e)) = decisions.pop() {
        // eprintln!("prop ({}, {}) -> {}", pos.0, pos.1, e);
        if let Some(z) = ctxt.table.get(&pos) {
            if *z != e { return Err(()); }
            else { continue; }
        }

        if (0..ctxt.n).any(|z| ctxt.table.get(&(pos.0, z)) == Some(&e)) { return Err(()); }

        assert!(!ctxt.table.contains_key(&pos));
        ctxt.table.insert(pos, e);
        ctxt.trail.push(TrailEvent::TableStore(pos));
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
    ctxt.trail.push(TrailEvent::ValueSet(t));
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
                    ctxt.trail.push(TrailEvent::PosTermsPush((x, y)));
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
