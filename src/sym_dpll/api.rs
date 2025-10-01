use crate::sym_dpll::*;

pub(in crate::sym_dpll) fn add(x: Id, y: Id, ctxt: &mut Ctxt) -> Id {
    if let Some(z) = ctxt.xyz.get(&(x, y)) {
        return *z;
    }

    let z = ctxt.classes.len();

    ctxt.classes.push(Class {
        uf: z,
        uf_rev: Vec::new(),
        usages: vec![(x, y, z)],
    });

    ctxt.xyz.insert((x, y), z);
    ctxt.xzy.insert((x, z), y);

    ctxt.classes[x].usages.push((x, y, z));
    if x != y {
        ctxt.classes[y].usages.push((x, y, z));
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
        let y = ctxt.classes[x].uf;
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
        ctxt.classes[b].uf = a;
        ctxt.classes[a].uf_rev.push(b);
    }
}

// we currently equate b into a.
// bb is some follower of b.
fn rebuild_usages(a: Id, b: Id, bb: Id, ctxt: &mut Ctxt) {
    if ctxt.mode == Mode::Backtrack {
        ctxt.dirty_stack.clear();
        return;
    }

    for bbb in ctxt.classes[bb].uf_rev.clone() {
        rebuild_usages(a, b, bbb, ctxt);
    }

    for (xo, yo, zo) in ctxt.classes[bb].usages.clone() {
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
            use std::collections::hash_map::Entry;

            ctxt.trail.push(TrailEvent::RmXYZ(x, y, z));
            assert_eq!(ctxt.xyz.remove(&(x, y)), Some(z));
            assert_eq!(ctxt.xzy.remove(&(x, z)), Some(y));

            let e1 = ctxt.xyz.entry((x2, y2));
            let e2 = ctxt.xzy.entry((x2, z2));

            let e1 = match e1 {
                Entry::Vacant(e1) => e1,
                Entry::Occupied(e1) => {
                    let z3 = e1.get();
                    union(z2, *z3, ctxt);
                    continue;
                },
            };

            let e2 = match e2 {
                Entry::Vacant(e2) => e2,
                Entry::Occupied(e2) => {
                    let y3 = e2.get();
                    union(y2, *y3, ctxt);
                    continue;
                },
            };

            ctxt.trail.push(TrailEvent::AddXYZ(x2, y2, z2));
            e1.insert(z2);
            e2.insert(y2);
        }
    }
}
