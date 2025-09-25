use crate::*;

// TODO things to add:
// - trail
// - multi-threading
// - symmetry breaking (via freshness)
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
}

fn choose_branch_id(ctxt: &Ctxt) -> Option<Id> {
    for x in 0..ctxt.n {
        for y in 0..ctxt.n {
            let z = ctxt.xyz[&(x, y)];
            if z >= ctxt.n {
                return Some(z);
            }
        }
    }
    None
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

fn mainloop(ctxt: Ctxt) {
    let Some(z) = choose_branch_id(&ctxt) else {
        print_model(&ctxt);
        return;
    };
    for x in 0..ctxt.n {
        let mut c = ctxt.clone();
        union(x, z, &mut c);
        rebuild(&mut c);
        if !c.paradox {
            mainloop(c);
        }
    }
}

pub fn sym_run(n: usize) {
    mainloop(new_ctxt(n));
}
