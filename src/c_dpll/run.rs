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
            let cond = match best {
                None => true,
                Some((_, score2)) => score > score2,
            };
            if cond {
                best = Some(((x, y), score));
            }
        }
    }
    Some(best?.0)
}

// TODO filter infeasible options
fn get_options(x: E, y: E, ctxt: &Ctxt) -> Vec<E> {
    let mut options = Vec::new();
    let mut found_fresh = false;
    for i in 0..ctxt.n {
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
    if ctxt.fresh[e as usize] {
        ctxt.fresh[e as usize] = false;
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
        let class = &mut ctxt.classes[idx(x, y, ctxt.n)];
        let cs = match class {
            Class::Defined(e2) => {
                if e == *e2 { continue }
                else {
                    ctxt.propagate_queue.clear();
                    become backtrack(ctxt)
                }
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
        }

        for &c in &cs {
            progress_c(c, x, y, e, ctxt);
        }

        // NOTE: The corresponding `*class = Class::Defined(e);` is a bit away, but I think it should be sound.
        ctxt.trail.push(TrailEvent::DefineClass(x, y, cs));
    }

    become branch(ctxt);
}
