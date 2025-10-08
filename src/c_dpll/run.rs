use crate::c_dpll::*;

pub fn c_run(n: usize) {
    let mut ctxt = build_ctxt(n);
    branch(&mut ctxt);
}

// returns None if we are done.
fn select_p(ctxt: &Ctxt) -> Option<(E, E)> {
    let mut best = None;
    for x in 0..ctxt.n {
        for y in 0..ctxt.n {
            let score = match &ctxt.classes[idx(x, y, ctxt.n)] {
                Class::Defined(_) => continue,
                Class::Pending(cs) => cs.len(),
            };
            if best.map(|(_, score2)| score > score2).unwrap_or(true) {
                best = Some(((x, y), score));
            }
        }
    }
    Some(best?.0)
}

fn infeasible_decision(x: E, y: E, e: E, ctxt: &Ctxt) -> bool {
    for z in 0..ctxt.n {
        if z == y { continue; }
        if let Class::Defined(e2) = ctxt.classes[idx(x, z, ctxt.n)] && e2 == e { return true; }
    }
    false
}

fn get_options(x: E, y: E, ctxt: &Ctxt) -> Vec<E> {
    let mut options = Vec::new();
    let mut found_fresh = false;
    for i in 0..ctxt.n {
        if infeasible_decision(x, y, i, ctxt) { continue }

        if ctxt.fresh[i as usize] {
            if found_fresh { continue }
            found_fresh = true;
        }
        options.push(i);
    }
    options
}

fn submit_model(ctxt: &Ctxt) {
    present_model(ctxt.n as usize, |x, y| match ctxt.classes[idx(x as E, y as E, ctxt.n)] {
        Class::Defined(e) => e as usize,
        Class::Pending(_) => unreachable!(),
    });
}

fn defresh(e: E, ctxt: &mut Ctxt) {
    let f = &mut ctxt.fresh[e as usize];
    if *f {
        *f = false;
        ctxt.trail.push(TrailEvent::Defresh(e));
    }
}

fn branch(ctxt: &mut Ctxt) {
    let Some((x, y)) = select_p(ctxt) else {
        submit_model(ctxt);
        become backtrack(ctxt);
    };

    defresh(x, ctxt);
    defresh(y, ctxt);

    let options = get_options(x, y, ctxt);
    if branch_options(x, y, options, ctxt).is_ok() { become propagate(ctxt); }
    else { become backtrack(ctxt); }
}

fn branch_options(x: E, y: E, mut options: Vec<E>, ctxt: &mut Ctxt) -> Result<(), ()> {
    if let Some(e) = options.pop() {
        defresh(e, ctxt);
        ctxt.trail.push(TrailEvent::Decision(x, y, options));
        ctxt.propagate_queue.push((x, y, e));
        Ok(())
    } else {
        Err(())
    }
}

fn backtrack(ctxt: &mut Ctxt) {
    ctxt.propagate_queue.clear();

    loop {
        let Some(event) = ctxt.trail.pop() else { return };
        match event {
            TrailEvent::Decision(x, y, mut options) => {
                if branch_options(x, y, options, ctxt).is_ok() { become propagate(ctxt); }
            },
            TrailEvent::DefineClass(x, y, cs) => {
                ctxt.classes[idx(x, y, ctxt.n)] = Class::Pending(cs);
            },
            TrailEvent::PushC(x, y) => {
                let Class::Pending(cs) = &mut ctxt.classes[idx(x, y, ctxt.n)] else { panic!() };
                cs.pop().unwrap();
            }
            TrailEvent::Defresh(x) => {
                ctxt.fresh[x as usize] = true;
            }
        }
    }
}

pub fn propagate(ctxt: &mut Ctxt) {
    while let Some((x, y, e)) = ctxt.propagate_queue.pop() {
        if infeasible_decision(x, y, e, ctxt) { become backtrack(ctxt);}

        let class = &mut ctxt.classes[idx(x, y, ctxt.n)];
        let cs = match class {
            Class::Defined(e2) => {
                if e == *e2 { continue }
                else { become backtrack(ctxt) }
            },
            Class::Pending(cs) => {
                let cs = std::mem::take(cs);
                *class = Class::Defined(e);
                cs
            },
        };

        // spawn constraints!
        {
            // x*y = e.
            let (a, b, ba) = (y, x, e);
            visit_c11(a, b, ba, ctxt);
            visit_c21(a, b, ba, ctxt);
        }

        for &c in &cs {
            progress_c(c, x, y, e, ctxt);
        }

        // The corresponding `*class = Class::Defined(e);` is a bit away, but I think it should be sound.
        // We need the `cs` for the above code though.
        // Invariant: visit_c11 and progress_c are not allowed to backtrack.
        //            They can just push stuff into the propagate_queue;
        //            and create TrailEvent::PushC.
        ctxt.trail.push(TrailEvent::DefineClass(x, y, cs));
    }

    become branch(ctxt);
}
