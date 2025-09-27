use crate::sym_dpll::*;

pub fn add(x: Id, y: Id, ctxt: &mut Ctxt) -> Id {
    if let Some(z) = ctxt.xyz.get(&(x, y)) {
        return *z;
    }

    let z = ctxt.unionfind.len();
    ctxt.unionfind.push(z);
    ctxt.usages.push(Vec::new());

    add_triple((x, y, z), ctxt);

    z
}

// you need to manually call rebuild() after this!
pub fn union(x: Id, y: Id, ctxt: &mut Ctxt) {
    let x = find(x, ctxt);
    let y = find(y, ctxt);
    if x == y { return; }

    let (x, y) = if x < y { (x, y) } else { (y, x) };
    ctxt.unionfind[y] = x;
    ctxt.dirty_stack.push(y);
    ctxt.trail.push(TrailEvent::Equate(x, y));

    if x < ctxt.n && y < ctxt.n {
        ctxt.mode = Mode::Backtrack;
    }
}

// TODO re-add path compression
pub fn find(mut x: Id, ctxt: &Ctxt) -> Id {
    loop {
        let y = ctxt.unionfind[x];
        if x == y { return x; }
        x = y;
    }
}

pub fn add_triple(t@(x, y, z): (Id, Id, Id), ctxt: &mut Ctxt) {
    if let Some(&z2) = ctxt.xyz.get(&(x, y)) {
        union(z, z2, ctxt);
        return;
    }
    if let Some(&y2) = ctxt.xzy.get(&(x, z)) {
        union(y, y2, ctxt);
        return;
    }

    ctxt.trail.push(TrailEvent::AddXYZ(x, y, z));

    raw_add_triple(t, ctxt);
}

pub fn raw_add_triple(t@(x, y, z): (Id, Id, Id), ctxt: &mut Ctxt) {
    assert!(!ctxt.xyz.contains_key(&(x, y)));
    ctxt.xyz.insert((x, y), z);
    ctxt.xzy.insert((x, z), y);
    ctxt.usages[x].push((x, y, z));
    if x != y {
        ctxt.usages[y].push((x, y, z));
    }
    if x != z && y != z {
        ctxt.usages[z].push((x, y, z));
    }
}

pub fn rm_triple(t@(x, y, z): (Id, Id, Id), ctxt: &mut Ctxt) {
    ctxt.trail.push(TrailEvent::RmXYZ(x, y, z));
    raw_rm_triple(t, ctxt);
}

pub fn raw_rm_triple(t@(x, y, z): (Id, Id, Id), ctxt: &mut Ctxt) {
    assert_eq!(ctxt.xyz.get(&(x, y)), Some(&z));

    ctxt.usages[x].retain(|t2| *t2 != t);
    ctxt.usages[y].retain(|t2| *t2 != t);
    ctxt.usages[z].retain(|t2| *t2 != t);
    assert_eq!(ctxt.xyz.remove(&(x, y)), Some(z));
    assert_eq!(ctxt.xzy.remove(&(x, z)), Some(y));
}

pub fn rebuild(ctxt: &mut Ctxt) {
    while let Some(a) = ctxt.dirty_stack.pop() {
        for t@(xo, yo, zo) in ctxt.usages[a].clone() {
            rm_triple(t, ctxt);

            let x = find(xo, ctxt);
            let y = find(yo, ctxt);
            let z = find(zo, ctxt);

            add_triple((x, y, z), ctxt);

            if ctxt.mode == Mode::Backtrack {
                ctxt.dirty_stack.clear();
                return;
            }
        }
    }
}

pub fn check(ctxt: &Ctxt) {
    let mut a: Vec<(Id, Id, Id)> = ctxt.xyz.iter().map(|((x, y), z)| (*x, *y, *z)).collect();
    let mut b: Vec<(Id, Id, Id)> = ctxt.xzy.iter().map(|((x, z), y)| (*x, *y, *z)).collect();

    a.sort();
    b.sort();

    assert_eq!(a, b);
}
