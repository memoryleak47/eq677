use crate::sym_dpll::*;

pub(in crate::sym_dpll) fn add(x: Id, y: Id, ctxt: &mut Ctxt) -> Id {
    if let Some(z) = ctxt.xyz.get(&(x, y)) {
        return *z;
    }

    let z = ctxt.unionfind.len();

    ctxt.unionfind.push(z);
    ctxt.unionfind_rev.push(Vec::new());

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

pub(in crate::sym_dpll) fn find(mut x: Id, ctxt: &Ctxt) -> Id {
    loop {
        let y = ctxt.unionfind[x];
        if x == y { return x; }
        x = y;
    }
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

        rebuild_usages(a, b, b, ctxt);

        ctxt.trail.push(TrailEvent::Equate(a, b));
        ctxt.unionfind[b] = a;
        ctxt.unionfind_rev[a].push(b);
    }
}

// we currently equate b into a.
// bb is some follower of b.
fn rebuild_usages(a: Id, b: Id, bb: Id, ctxt: &mut Ctxt) {
    if ctxt.mode == Mode::Backtrack {
        ctxt.dirty_stack.clear();
        return;
    }

    for bbb in ctxt.unionfind_rev[bb].clone() {
        rebuild_usages(a, b, bbb, ctxt);
    }

    for (xo, yo, zo) in ctxt.usages[bb].clone() {
        if ctxt.mode == Mode::Backtrack {
            ctxt.dirty_stack.clear();
            return;
        }

        let x = find(xo, ctxt);
        let y = find(yo, ctxt);
        let z = find(zo, ctxt);

        let x2 = if x == b { a } else { x };
        let y2 = if y == b { a } else { y };
        let z2 = if z == b { a } else { z };

        if ctxt.xyz.get(&(x, y)) == Some(&z) {
            ctxt.trail.push(TrailEvent::RmXYZ(x, y, z));
            assert_eq!(ctxt.xyz.remove(&(x, y)), Some(z));
            assert_eq!(ctxt.xzy.remove(&(x, z)), Some(y));

            if let Some(z3) = ctxt.xyz.get(&(x2, y2)) {
                union(z2, *z3, ctxt);
                continue;
            }
            if let Some(y3) = ctxt.xzy.get(&(x2, z2)) {
                union(y2, *y3, ctxt);
                continue;
            }

            ctxt.trail.push(TrailEvent::AddXYZ(x2, y2, z2));
            ctxt.xyz.insert((x2, y2), z2);
            ctxt.xzy.insert((x2, z2), y2);
        }
    }
}
