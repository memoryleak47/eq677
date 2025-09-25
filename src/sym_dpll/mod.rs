use crate::*;

use rayon::prelude::*;

// TODO things to add:
// - trail
// - somehow improve rebuilding

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

#[derive(Clone)]
struct Ctxt {
    // Note: f(x, y) = z
    xyz: Map<(Id, Id), Id>,
    xzy: Map<(Id, Id), Id>,

    usages: Vec<Vec<(Id, Id, Id)>>,

    unionfind: Vec<Id>, // indexed by Id.
    n: usize,
    dirty_stack: Vec<Id>,
    paradox: bool,

    fresh: Vec<bool>,
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
    // ctxt.dump();

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

fn mainloop(mut ctxt: Ctxt) {
    let Some((x, y)) = choose_branch_id(&ctxt) else {
        print_model(&ctxt);
        return;
    };

    ctxt.fresh[x] = false;
    ctxt.fresh[y] = false;

    let options = get_options((x, y), &ctxt);
    options.into_par_iter().for_each(|e| {
        let z = ctxt.xyz[&(x, y)];
        let mut c = ctxt.clone();
        c.fresh[e] = false;
        union(e, z, &mut c);
        rebuild(&mut c);
        if !c.paradox {
            mainloop(c);
        }
    });
}

pub fn sym_run(n: usize) {
    new_ctxts(n).into_par_iter().for_each(|ctxt| {
        mainloop(ctxt);
    });
}
