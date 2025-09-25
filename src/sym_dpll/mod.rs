use crate::*;

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
    unionfind: Vec<Id>, // indexed by Id.
    n: usize,
    dirty: bool,
    paradox: bool,
}

fn new_ctxt(n: usize) -> Ctxt {
    Ctxt {
        xyz: Map::default(),
        xzy: Map::default(),
        unionfind: (0..n).collect(), // setup the initial 0..n ElemId classes.
        n,
        dirty: false,
        paradox: false,
    }
}

fn add(x: Id, y: Id, ctxt: &mut Ctxt) -> Id {
    if let Some(z) = ctxt.xyz.get(&(x, y)) {
        return *z;
    }

    let z = ctxt.unionfind.len();
    ctxt.xyz.insert((x, y), z);
    ctxt.xzy.insert((x, z), y);
    ctxt.unionfind.push(z);

    z
}

// you need to manually call rebuild() after this!
fn union(x: Id, y: Id, ctxt: &mut Ctxt) {
    let x = find(x, ctxt);
    let y = find(y, ctxt);
    if x == y { return; }

    let (x, y) = if x < y { (x, y) } else { (y, x) };
    ctxt.unionfind[y] = x;
    ctxt.dirty = true;

    if x < ctxt.n && y < ctxt.n {
        ctxt.paradox = true;
    }
}

fn find(x: Id, ctxt: &mut Ctxt) -> Id {
    let y = ctxt.unionfind[x];
    if x == y { return y; }

    let z = find(y, ctxt);
    if z != y {
        ctxt.unionfind[x] = z;
    }
    z
}

fn rebuild(ctxt: &mut Ctxt) {
    while ctxt.dirty {
        ctxt.dirty = false;
        for ((x, y), z) in ctxt.xyz.clone() {
            let x = find(x, ctxt);
            let y = find(y, ctxt);
            let z = find(z, ctxt);
            if let Some(&z2) = ctxt.xyz.get(&(x, y)) {
                union(z, z2, ctxt);
            }
            ctxt.xyz.insert((x, y), z);
            if let Some(&y2) = ctxt.xzy.get(&(x, z)) {
                union(y, y2, ctxt);
            }
            ctxt.xzy.insert((x, z), y);
        }
    }
}

fn setup_constraints(ctxt: &mut Ctxt) {
    for x in 0..ctxt.n {
        for y in 0..ctxt.n {
            // x = f(y, f(x, f(f(y, x), y)))
            let yx = add(y, x, ctxt);
            let yxy = add(yx, y, ctxt);
            let xyxy = add(x, yxy, ctxt);
            let yxyxy = add(y, xyxy, ctxt);
            union(x, yxyxy, ctxt);
        }
    }

    for x in 0..ctxt.n {
        for y in 0..ctxt.n {
            // x = f(yx, f(f(y, yx), y))
            let yx = add(y, x, ctxt);
            let yyx = add(y, yx, ctxt);
            let yyxy = add(yyx, y, ctxt);
            let yxyyxy = add(yx, yyxy, ctxt);
            union(x, yxyyxy, ctxt);
        }
    }
    rebuild(ctxt);
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
    let mut ctxt = new_ctxt(n);
    setup_constraints(&mut ctxt);
    mainloop(ctxt);
}
