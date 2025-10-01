use crate::*;

// TODO things to add:
// - multi-threading
// - somehow improve rebuilding (the "usages" datastructure thing seems suboptimal)

mod api;
pub use api::*;

mod init;
pub use init::*;

use std::sync::Mutex;
use std::collections::HashSet;

lazy_static::lazy_static! {
    static ref DB: Mutex<HashSet<MatrixMagma>> = Mutex::new(HashSet::new());
}

type Map<T, K> = fxhash::FxHashMap<T, K>;

// e-class Id.
// ids less than ctxt.n correspond to ElemIds aswell.
type Id = usize;

#[derive(Clone, PartialEq, Eq)]
enum Mode {
    Forward,
    Backtrack,
    Done,
}

#[derive(Clone)]
enum TrailEvent {
    DecisionPoint(Id, /*remaining options*/ Vec<Id>),
    RmXYZ(Id, Id, Id),
    AddXYZ(Id, Id, Id),
    Equate(/*x*/ Id, /*y*/ Id), // equates x <- y.
    Defresh(Id),
}

#[derive(Clone)]
struct Ctxt {
    // Note: f(x, y) = z
    xyz: Map<(Id, Id), Id>,
    xzy: Map<(Id, Id), Id>,

    usages: Vec<Vec<(Id, Id, Id)>>,

    unionfind: Vec<Id>, // indexed by Id.
    n: usize,
    dirty_stack: Vec<(Id, Id)>,

    fresh: Vec<bool>,

    trail: Vec<TrailEvent>,
    mode: Mode,
}

fn choose_branch_id(ctxt: &Ctxt) -> Option<(Id, Id)> {
    let mut best: Option<((Id, Id), usize)> = None;
    for x in 0..ctxt.n {
        for y in 0..ctxt.n {
            let z = ctxt.xyz[&(x, y)];
            if z < ctxt.n { continue; }
            let score = ctxt.usages[z].len(); // Is this a good heuristic?
            if best.map(|(_, score2) | score2 < score).unwrap_or(true) {
                best = Some(((x, y), score));
            }
        }
    }
    Some(best?.0)
}

fn print_model(ctxt: &Ctxt) {
    let magma = MatrixMagma::by_fn(ctxt.n, |x, y| ctxt.xyz[&(x, y)]);
    let magma = magma.canonicalize();

    let mut handle = DB.lock().unwrap();
    if handle.contains(&magma) { return; }

    handle.insert(magma.clone());
    drop(handle);

    println!("Model found:");
    magma.dump();

    assert!(magma.is677());
    assert!(magma.is255());
}

fn infeasible_decision((x, y): (Id, Id), z: Id, ctxt: &Ctxt) -> bool {
    ctxt.xzy[&(x, z)] < ctxt.n
}

fn get_options((x, y): (Id, Id), ctxt: &Ctxt) -> Vec<Id> {
    let mut found_fresh = false;
    let mut options = Vec::new();
    for e in 0..ctxt.n {
        if infeasible_decision((x, y), e, &ctxt) { continue; }
        if ctxt.fresh[e] {
            if found_fresh { continue; }
            else { found_fresh = true; }
        }
        options.push(e);
    }
    options
}

fn backtrack(ctxt: &mut Ctxt) {
    loop {
        if ctxt.trail.is_empty() { ctxt.mode = Mode::Done; return; }
        match ctxt.trail.pop().unwrap() {
            TrailEvent::DecisionPoint(z, options) => {
                activate_option(z, options, ctxt);
                return;
            },
            TrailEvent::RmXYZ(x, y, z) => { raw_add_triple((x, y, z), ctxt); },
            TrailEvent::AddXYZ(x, y, z) => { raw_rm_triple((x, y, z), ctxt); },
            TrailEvent::Equate(x, y) => { ctxt.unionfind[y] = y; },
            TrailEvent::Defresh(x) => { ctxt.fresh[x] = true; },
        }
    }
}

fn forward(ctxt: &mut Ctxt) {
    let Some((x, y)) = choose_branch_id(ctxt) else {
        print_model(&ctxt);
        ctxt.mode = Mode::Backtrack;
        return;
    };

    if ctxt.fresh[x] {
        ctxt.trail.push(TrailEvent::Defresh(x));
        ctxt.fresh[x] = false;
    }
    if ctxt.fresh[y] {
        ctxt.trail.push(TrailEvent::Defresh(y));
        ctxt.fresh[y] = false;
    }

    let options = get_options((x, y), ctxt);
    let z = ctxt.xyz[&(x, y)];
    activate_option(z, options, ctxt);
}

fn mainloop(mut ctxt: Ctxt) {
    loop {
        match ctxt.mode {
            Mode::Forward => forward(&mut ctxt),
            Mode::Backtrack => backtrack(&mut ctxt),
            Mode::Done => return,
        }
    }
}

fn activate_option(z: Id, mut options: Vec<Id>, ctxt: &mut Ctxt) {
    let Some(e) = options.pop() else {
        ctxt.mode = Mode::Backtrack;
        return;
    };

    ctxt.mode = Mode::Forward;

    if ctxt.fresh[e] {
        ctxt.trail.push(TrailEvent::Defresh(e));
        ctxt.fresh[e] = false;
    }

    ctxt.trail.push(TrailEvent::DecisionPoint(z, options));

    union(z, e, ctxt);
    rebuild(ctxt);
}

pub fn sym_run(n: usize) {
    let models = new_ctxts(n);
    for ctxt in models {
        mainloop(ctxt);
    }
}
