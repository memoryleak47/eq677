type Map<T, K> = fxhash::FxHashMap<T, K>;

// e-class Id.
// ids less than ctxt.n correspond to ElemIds aswell.
type Id = usize;

struct Ctxt {
    // Note: f(x, y) = z
    xyz: Map<(Id, Id), Id>,
    xzy: Map<(Id, Id), Id>,
    unionfind: Vec<Id>, // indexed by Id.
    n: usize,
    dirty: bool,
}

fn new_ctxt(n: usize) -> Ctxt {
    Ctxt {
        xyz: Map::default(),
        xzy: Map::default(),
        unionfind: Vec::new(),
        n,
        dirty: false,
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
            if let Some(&y2) = ctxt.xyz.get(&(x, z)) {
                union(y, y2, ctxt);
            }
            ctxt.xzy.insert((x, z), y);
        }
    }
}

fn sym_run(n: usize) {
    let ctxt = new_ctxt(n);
}
