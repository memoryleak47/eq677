type Map<T, K> = fxhash::FxHashMap<T, K>;

type Id = usize; // e-class Id.

struct Ctxt {
    // Note: f(x, y) = z
    xyz: Map<(Id, Id), Id>,
    xzy: Map<(Id, Id), Id>,
    unionfind: Vec<Id>, // indexed by Id.
    n: usize,
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

fn sym_run(n: usize) {
    let ctxt = Ctxt {
        xyz: Map::default(),
        xzy: Map::default(),
        unionfind: Vec::new(),
        n,
    };
}
