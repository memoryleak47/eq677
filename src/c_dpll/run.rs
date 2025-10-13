use crate::c_dpll::*;

fn threading_depth(n: E) -> E { n + 1 }

pub fn c_run(n: usize) {
    let models = split_models(build_ctxt(n));
    into_par_for_each(models, |mut ctxt| {
        prerun(0, &mut ctxt);
    });
}

fn prerun(depth: E, ctxt: &mut Ctxt) {
    // No need to have a trail, we won't backtrack in the prerun.
    ctxt.trail.clear();

    if depth >= threading_depth(ctxt.n) {
        main_branch(ctxt);
        return;
    }

    let Some((x, y)) = select_p(ctxt) else {
        submit_model(ctxt);
        return;
    };

    // We don't use "defresh(x, ctxt)" as the prerun doesn't need to put stuff on the trail.
    ctxt.fresh[x as usize] = false;
    ctxt.fresh[y as usize] = false;

    let options = get_options(x, y, ctxt);
    into_par_for_each(options, |e| {
        let c = &mut ctxt.clone();

        if prove_triple(x, y, e, c).is_err() { return }
        if propagate(c).is_err() { return }

        c.fresh[e as usize] = false;
        prerun(depth+1, c);
    });
}

// NOTE:
// let score = class.cs.iter().map(|c| score_c(*c)).sum::<i32>() + (x == 0) as i32;
fn score_c(c: CXY) -> i32 {
    match c {
        CXY::C11(..) => 2,
        CXY::C12(..) => 3,
        CXY::C21(..) => 2,
        CXY::C22(..) => 3,
    }
}

// returns None if we are done.
fn select_p(ctxt: &Ctxt) -> Option<(E, E)> {
    ctxt.heap.get(0).copied()
}

fn get_options(x: E, y: E, ctxt: &Ctxt) -> Vec<E> {
    let mut found_fresh = false;
    (0..ctxt.n).filter(|&i| {
        if ctxt.classes_xz[idx(x, i, ctxt.n)].value != E::MAX { return false; }
        if ctxt.fresh[i as usize] {
            if found_fresh { return false; }
            found_fresh = true;
        }
        true
    }).collect()
}

fn submit_model(ctxt: &Ctxt) {
    present_model(ctxt.n as usize, |x, y| {
        let i = idx(x as E, y as E, ctxt.n);
        ctxt.classes_xy[i].value as usize
    });
}

fn defresh(e: E, ctxt: &mut Ctxt) {
    let f = &mut ctxt.fresh[e as usize];
    if *f {
        *f = false;
        ctxt.trail.push(TrailEvent::Defresh(e));
    }
}

fn main_branch(ctxt: &mut Ctxt) {
    let Some((x, y)) = select_p(ctxt) else {
        submit_model(ctxt);
        become main_backtrack(ctxt);
    };

    defresh(x, ctxt);
    defresh(y, ctxt);

    let options = get_options(x, y, ctxt);
    if branch_options(x, y, options, ctxt).is_ok() { become main_propagate(ctxt); }
    else { become main_backtrack(ctxt); }
}

fn branch_options(x: E, y: E, mut options: Vec<E>, ctxt: &mut Ctxt) -> Result<(), ()> {
    if let Some(e) = options.pop() {
        ctxt.trail.push(TrailEvent::Decision(x, y, options));
        prove_triple(x, y, e, ctxt)?;

        // note: This defresh has to be after the decision point!
        // Otherwise it won't get re-freshed on backtracking.
        defresh(e, ctxt);

        Ok(())
    } else {
        Err(())
    }
}

fn main_backtrack(ctxt: &mut Ctxt) {
    ctxt.propagate_queue.clear();

    loop {
        let Some(event) = ctxt.trail.pop() else { return };
        match event {
            TrailEvent::Decision(x, y, mut options) => {
                if branch_options(x, y, options, ctxt).is_ok() { become main_propagate(ctxt); }
            },
            TrailEvent::DefineClass(x, y) => {
                let z = std::mem::replace(&mut ctxt.classes_xy[idx(x, y, ctxt.n)].value, E::MAX);
                ctxt.classes_xz[idx(x, z, ctxt.n)].value = E::MAX;
                heap_push(x, y, ctxt);
            },
            TrailEvent::PushCXY(x, y) => {
                ctxt.classes_xy[idx(x, y, ctxt.n)].cs.pop().unwrap();
            }
            TrailEvent::PushCXZ(x, z) => {
                ctxt.classes_xz[idx(x, z, ctxt.n)].cs.pop().unwrap();
            }
            TrailEvent::Defresh(x) => {
                ctxt.fresh[x as usize] = true;
            }
        }
    }
}

pub fn main_propagate(ctxt: &mut Ctxt) {
    match propagate(ctxt) {
        Ok(()) => become main_branch(ctxt),
        Err(()) => become main_backtrack(ctxt),
    }
}

pub fn prove_triple(x: E, y: E, z: E, ctxt: &mut Ctxt) -> Result<(), ()> {
    let xy_ref = &mut ctxt.classes_xy[idx(x, y, ctxt.n)].value;
    let xy = *xy_ref;
    if xy == z { return Ok(()) }
    if xy != E::MAX { return Err(()) }

    let xz_ref = &mut ctxt.classes_xz[idx(x, z, ctxt.n)].value;
    if *xz_ref != E::MAX { return Err(()); }

    *xy_ref = z;
    *xz_ref = y;
    ctxt.trail.push(TrailEvent::DefineClass(x, y));
    ctxt.propagate_queue.push((x, y, z));
    heap_remove(x, y, ctxt);
    Ok(())
}

pub fn propagate(ctxt: &mut Ctxt) -> Result<(), ()> {
    while let Some((x, y, z)) = ctxt.propagate_queue.pop() {
        // spawn constraints!
        let (a, b, ba) = (y, x, z);
        visit_c11(a, b, ba, ctxt)?;
        visit_c21(a, b, ba, ctxt)?;

        let i = idx(x, y, ctxt.n);
        let len = ctxt.classes_xy[i].cs.len();
        for j in 0..len {
            let c = ctxt.classes_xy[i].cs[j];
            progress_c(c, x, y, z, ctxt)?;
        }

        let i = idx(x, z, ctxt.n);
        let len = ctxt.classes_xz[i].cs.len();
        for j in 0..len {
            let CXZ(a,b) = ctxt.classes_xz[i].cs[j];
            // z = x*(a*b).
            prove_triple(a, b, y, ctxt)?;
        }
    }

    Ok(())
}
