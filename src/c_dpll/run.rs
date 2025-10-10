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

        c.propagate_queue.push((x, y, e));
        c.fresh[e as usize] = false;

        if propagate(c).is_ok() {
            prerun(depth+1, c);
        }
    });
}

fn score_c(c: C) -> usize {
    match c {
        C::C11(..) => 2,
        C::C12(..) => 3,
        C::C21(..) => 2,
        C::C22(..) => 3,
    }
}

// returns None if we are done.
fn select_p(ctxt: &Ctxt) -> Option<(E, E)> {
    let mut best = None;
    for x in 0..ctxt.n {
        for y in 0..ctxt.n {
            let class = &ctxt.classes[idx(x, y, ctxt.n)];
            if class.value != E::MAX { continue }

            let score = class.cs.iter().map(|c| score_c(*c)).sum::<usize>() + (x == 0) as usize;
            if best.map(|(_, score2)| score > score2).unwrap_or(true) {
                best = Some(((x, y), score));
            }
        }
    }
    Some(best?.0)
}

fn infeasible_decision(x: E, y: E, e: E, ctxt: &Ctxt) -> bool {
    for z in 0..ctxt.n {
        if ctxt.classes[idx(x, z, ctxt.n)].value == e { return true; }
    }
    false
}

fn get_options(x: E, y: E, ctxt: &Ctxt) -> Vec<E> {
    let mut found_fresh = false;
    (0..ctxt.n).filter(|&i| {
        if infeasible_decision(x, y, i, ctxt) { return false; }
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
        ctxt.classes[i].value as usize
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
        ctxt.propagate_queue.push((x, y, e));

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
                ctxt.classes[idx(x, y, ctxt.n)].value = E::MAX;
            },
            TrailEvent::PushC(x, y) => {
                ctxt.classes[idx(x, y, ctxt.n)].cs.pop().unwrap();
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

pub fn propagate(ctxt: &mut Ctxt) -> Result<(), ()> {
    while let Some((x, y, e)) = ctxt.propagate_queue.pop() {
        let i = idx(x, y, ctxt.n);

        let v = ctxt.classes[i].value;
        if v == e { continue }
        if v != E::MAX { return Err(()) }

        if infeasible_decision(x, y, e, ctxt) { return Err(()); }

        ctxt.classes[i].value = e;
        ctxt.trail.push(TrailEvent::DefineClass(x, y));

        // spawn constraints!
        let (a, b, ba) = (y, x, e);
        visit_c11(a, b, ba, ctxt);
        visit_c21(a, b, ba, ctxt);

        let len = ctxt.classes[i].cs.len();
        for j in 0..len {
            let c = ctxt.classes[i].cs[j];
            progress_c(c, x, y, e, ctxt);
        }
    }

    Ok(())
}
