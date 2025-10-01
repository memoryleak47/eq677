use crate::sym_dpll::*;

pub(in crate::sym_dpll) fn add(x: Id, y: Id, ctxt: &mut Ctxt) -> Id {
    if let Some(z) = ctxt.xyz.get(&(x, y)) {
        return *z;
    }

    let z = ctxt.unionfind.len();
    ctxt.unionfind.push(z);
    ctxt.usages.push(Vec::new());

    ctxt.xyz.insert((x, y), z);
    ctxt.xzy.insert((x, z), y);
    ctxt.usages[x].push((x, y, z));
    if x != y {
        ctxt.usages[y].push((x, y, z));
    }
    if x != z && y != z {
        ctxt.usages[z].push((x, y, z));
    }

    z
}

// you need to manually call rebuild() after this!
pub(in crate::sym_dpll) fn union(x: Id, y: Id, ctxt: &mut Ctxt) {
    let x = find(x, ctxt);
    let y = find(y, ctxt);
    if x == y { return; }

    ctxt.dirty_stack.push((x, y)); // unordered x = y.

    if x < ctxt.n && y < ctxt.n {
        ctxt.mode = Mode::Backtrack;
    }
}

// TODO re-add path compression
pub(in crate::sym_dpll) fn find(mut x: Id, ctxt: &Ctxt) -> Id {
    loop {
        let y = ctxt.unionfind[x];
        if x == y { return x; }
        x = y;
    }
}

pub(in crate::sym_dpll) fn add_triple(t@(x, y, z): (Id, Id, Id), ctxt: &mut Ctxt) {
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

pub(in crate::sym_dpll) fn raw_add_triple(t@(x, y, z): (Id, Id, Id), ctxt: &mut Ctxt) {
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

pub(in crate::sym_dpll) fn rm_triple(t@(x, y, z): (Id, Id, Id), ctxt: &mut Ctxt) {
    ctxt.trail.push(TrailEvent::RmXYZ(x, y, z));
    raw_rm_triple(t, ctxt);
}

pub(in crate::sym_dpll) fn raw_rm_triple(t@(x, y, z): (Id, Id, Id), ctxt: &mut Ctxt) {
    assert_eq!(ctxt.xyz.get(&(x, y)), Some(&z));

    ctxt.usages[x].retain(|t2| *t2 != t);
    ctxt.usages[y].retain(|t2| *t2 != t);
    ctxt.usages[z].retain(|t2| *t2 != t);
    assert_eq!(ctxt.xyz.remove(&(x, y)), Some(z));
    assert_eq!(ctxt.xzy.remove(&(x, z)), Some(y));
}

pub(in crate::sym_dpll) fn rebuild(ctxt: &mut Ctxt) {
    if ctxt.mode == Mode::Backtrack {
        ctxt.dirty_stack.clear();
        return;
    }
    while let Some((a, b)) = ctxt.dirty_stack.pop() {
        let a = find(a, ctxt);
        let b = find(b, ctxt);
        if a == b { continue; }
        let (a, b) = if a < b { (a, b) } else { (b, a) };

        if a < ctxt.n && b < ctxt.n {
            ctxt.mode = Mode::Backtrack;
            ctxt.dirty_stack.clear();
            return;
        }

        ctxt.trail.push(TrailEvent::Equate(a, b));
        ctxt.unionfind[b] = a;

        for (x, y, z) in ctxt.usages[b].clone() {
            rm_triple((x, y, z), ctxt);

            let x = if x == b { a } else { x };
            let y = if y == b { a } else { y };
            let z = if z == b { a } else { z };

            add_triple((x, y, z), ctxt);

            if ctxt.mode == Mode::Backtrack {
                ctxt.dirty_stack.clear();
                return;
            }
        }
    }
}

#[allow(unused)]
pub(in crate::sym_dpll) fn check(ctxt: &Ctxt) {
    let mut a: Vec<(Id, Id, Id)> = ctxt.xyz.iter().map(|((x, y), z)| (*x, *y, *z)).collect();
    let mut b: Vec<(Id, Id, Id)> = ctxt.xzy.iter().map(|((x, z), y)| (*x, *y, *z)).collect();

    a.sort();
    b.sort();

    assert_eq!(a, b);
}
