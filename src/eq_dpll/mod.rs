use crate::*;
use rayon::prelude::*;
use smallvec::SmallVec;

fn threading_depth(n: usize) -> usize { n+1 }

mod init;
pub use init::*;

mod dump;
pub use dump::*;

type ElemId = usize;
type PosId = (usize, usize);

pub fn idx((x, y): PosId, n: usize) -> usize {
    x + n * y
}

#[derive(Clone)]
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

#[derive(PartialEq, Eq, Clone)]
enum Node {
    Elem(ElemId),
    F(TermId, TermId),
    AssertEq(ElemId, TermId),
}

#[derive(Clone)]
struct Class {
    node: Node,
    parents: SmallVec<[TermId; 2]>,
    value: Option<ElemId>,
}

#[derive(Default, Clone)]
enum Mode {
    #[default] Forward,
    Backtracking,
    Done,
}

#[derive(Clone)]
enum PropagationTask {
    SetClass(TermId, ElemId),
    Decision(PosId, ElemId),
    VisitParent(TermId),
}

#[derive(Default, Clone)]
struct Ctxt {
    trail: Vec<TrailEvent>,
    classes: Vec<Class>, // indexed by TermId
    table: Vec<ElemId>, // maps "PosId" (via idx encoding) to Option<ElemId>, where ElemId::MAX means None.

    // maps each PosId to a set of terms that currently evaluate to this PosId (if you eval its children).
    // Note: PosId use idx encoding.
    pos_terms: Vec<Vec<TermId>>,

    n: usize,
    mode: Mode,

    fresh: Vec<bool>, // whether an ElemId is still "fresh".
    depth: usize,

    propagate_queue: Vec<PropagationTask>,
}

pub fn eq_run(n: usize) {
    mainloop(build_ctxt(n));
}

fn mainloop(mut ctxt: Ctxt) {
    loop {
        match ctxt.mode {
            Mode::Forward => forward_step(&mut ctxt),
            Mode::Backtracking => backtrack_step(&mut ctxt),
            Mode::Done => break,
        }
    }
}

fn threaded_forward_step(ctxt: &mut Ctxt) {
    let Some((pos, options)) = next_options(ctxt) else {
        print_model(ctxt);
        ctxt.mode = Mode::Backtracking;
        return;
    };

    options.par_iter().for_each(|e| {
        // NOTE: this is one clone too many.
        let mut ctxt = ctxt.clone();
        ctxt.depth += 1;
        activate_option(pos, vec![*e], &mut ctxt);
        mainloop(ctxt);
    });
    ctxt.mode = Mode::Done;
}

fn forward_step(ctxt: &mut Ctxt) {
    if ctxt.depth < threading_depth(ctxt.n) {
        threaded_forward_step(ctxt);
        return;
    }

    let Some((pos, options)) = next_options(ctxt) else {
        print_model(ctxt);
        ctxt.mode = Mode::Backtracking;
        return;
    };

    activate_option(pos, options, ctxt);
}

fn backtrack_step(ctxt: &mut Ctxt) {
    loop {
        match ctxt.trail.pop() {
            Some(TrailEvent::DecisionPoint(pos, options)) => {
                activate_option(pos, options, ctxt);
                return;
            },
            Some(TrailEvent::TableStore(pos)) => { ctxt.table[idx(pos, ctxt.n)] = ElemId::MAX; },
            Some(TrailEvent::PosTermsPush(pos)) => { ctxt.pos_terms[idx(pos, ctxt.n)].pop(); },
            Some(TrailEvent::ValueSet(tid)) => { ctxt.classes[tid.0].value = None; },
            Some(TrailEvent::Defresh(e)) => { ctxt.fresh[e] = true; },
            None => { ctxt.mode = Mode::Done; break; }
        }
    }
}

fn print_model(ctxt: &Ctxt) {
    let magma = MatrixMagma::by_fn(ctxt.n, |x, y| ctxt.table[idx((x, y), ctxt.n)]);
    println!("Model found:");
    magma.dump();
    // ctxt.dump();

    assert!(magma.is677());
    assert!(magma.is255());
}

fn best_score(ctxt: &Ctxt) -> Option<PosId> {
    let mut best = None;
    // NOTE: flipping these loops doubles the runtime. So this is crucially important.
    for x in 0..ctxt.n {
        for y in 0..ctxt.n {
            let i = idx((x, y), ctxt.n);
            if ctxt.table[i] != ElemId::MAX { continue; }
            let score = ctxt.pos_terms[i].len();
            let cond = match best {
                None => true,
                Some((_, score2)) => score > score2,
            };
            if cond {
                best = Some(((x, y), score));
            }
        }
    }
    Some(best?.0)
}

fn next_options(ctxt: &mut Ctxt) -> Option<(PosId, Vec<ElemId>)> {
    let pos = best_score(ctxt)?;

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
            // If we already used a "fresh" ElemId, no reason to do the same operation for another fresh one!
            if found_fresh { continue }
            else { found_fresh = true; }
        }

        if infeasible_decision(pos, e, ctxt) { continue }

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
    assert!(ctxt.propagate_queue.is_empty());
    ctxt.propagate_queue.push(PropagationTask::Decision(pos, e));

    while let Some(task) = ctxt.propagate_queue.pop() {
        let output = match task {
            PropagationTask::Decision(pos, e) => handle_decision(pos, e, ctxt),
            PropagationTask::SetClass(t, e) => handle_set_class(t, e, ctxt),
            PropagationTask::VisitParent(p) => handle_visit_parent(p, ctxt),
        };
        if let Err(()) = output {
            ctxt.propagate_queue.clear();
            return Err(());
        }
    }
    Ok(())
}

fn infeasible_decision((x, y): PosId, e: ElemId, ctxt: &Ctxt) -> bool {
    for z in 0..ctxt.n {
        if ctxt.table[x + z*ctxt.n] == e { return true; }
    }
    false
}

fn handle_decision(pos: PosId, e: ElemId, ctxt: &mut Ctxt) -> Res {
    // eprintln!("prop ({}, {}) -> {}", pos.0, pos.1, e);
    if let z = ctxt.table[idx(pos, ctxt.n)] && z != ElemId::MAX {
        if z != e { return Err(()); }
        else { return Ok(()); }
    }

    if infeasible_decision(pos, e, ctxt) { return Err(()); }


    assert_eq!(ctxt.table[idx(pos, ctxt.n)], ElemId::MAX);
    ctxt.table[idx(pos, ctxt.n)] = e;
    ctxt.trail.push(TrailEvent::TableStore(pos));
    let terms = &ctxt.pos_terms[idx(pos, ctxt.n)];

    for &tid in terms {
        ctxt.propagate_queue.push(PropagationTask::SetClass(tid, e));
    }
    Ok(())
}

fn handle_set_class(t: TermId, v: ElemId, ctxt: &mut Ctxt) -> Res {
    if ctxt.classes[t.0].value == Some(v) { return Ok(()); }
    assert!(ctxt.classes[t.0].value.is_none(), "Class set to different things?");

    ctxt.classes[t.0].value = Some(v);
    ctxt.trail.push(TrailEvent::ValueSet(t));
    for &parent in &ctxt.classes[t.0].parents {
        ctxt.propagate_queue.push(PropagationTask::VisitParent(parent));
    }
    Ok(())
}

// Called when we've computed one of the children of "t".
fn handle_visit_parent(t: TermId, ctxt: &mut Ctxt) -> Res {
    match ctxt.classes[t.0].node {
        Node::F(x, y) => {
            let Some(x) = ctxt.classes[x.0].value else { return Ok(()) };
            let Some(y) = ctxt.classes[y.0].value else { return Ok(()) };
            if let z = ctxt.table[idx((x, y), ctxt.n)] && z != ElemId::MAX {
                ctxt.propagate_queue.push(PropagationTask::SetClass(t, z));
            } else {
                for &p in &ctxt.classes[t.0].parents {
                    if let Node::AssertEq(v, _) = ctxt.classes[p.0].node {
                        ctxt.propagate_queue.push(PropagationTask::Decision((x, y), v));
                    }

                    if let Node::F(l, r) = ctxt.classes[p.0].node {
                        let Some(l) = ctxt.classes[l.0].value else { continue };
                        assert!(t == r);
                        for &pp in &ctxt.classes[p.0].parents {
                            if let Node::AssertEq(v, _) = ctxt.classes[pp.0].node {
                                for a in 0..ctxt.n {
                                    if ctxt.table[idx((l, a), ctxt.n)] == v {
                                        // if f(l, a) = v /\ f(l, f(x, y)) = v, then
                                        // f(x, y) = a.
                                        ctxt.propagate_queue.push(PropagationTask::Decision((x, y), a));
                                    }
                                }
                            }
                        }
                    }
                }

                let a = &mut ctxt.pos_terms[idx((x, y), ctxt.n)];
                // invariant: assert!(!a.contains(&t));
                a.push(t);
                ctxt.trail.push(TrailEvent::PosTermsPush((x, y)));
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
