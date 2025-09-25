use crate::sym_dpll::*;

pub fn add(x: Id, y: Id, ctxt: &mut Ctxt) -> Id {
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
pub fn union(x: Id, y: Id, ctxt: &mut Ctxt) {
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

pub fn find(x: Id, ctxt: &mut Ctxt) -> Id {
    let y = ctxt.unionfind[x];
    if x == y { return y; }

    let z = find(y, ctxt);
    if z != y {
        ctxt.unionfind[x] = z;
    }
    z
}

pub fn rebuild(ctxt: &mut Ctxt) {
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

