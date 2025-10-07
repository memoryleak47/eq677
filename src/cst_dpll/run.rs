use crate::cst_dpll::*;

pub fn cst_run(n: usize) {
    let ctxt = build_ctxt(n);
    mainloop(ctxt);
}

// returns None if we are done.
fn select_p(ctxt: &Ctxt) -> Option<P> {
    let mut best = None;
    for x in 0..(ctxt.n as E) {
        for y in 0..(ctxt.n as E) {
            let p = mk_p(x, y, ctxt.n);
            let score = match &ctxt.classes[p as usize] {
                Class::Defined(_) => continue,
                Class::Pending(cs) => cs.len(),
            };
            let cond = match best {
                None => true,
                Some((_, score2)) => score > score2,
            };
            if cond {
                best = Some((p, score));
            }
        }
    }
    Some(best?.0)
}

// TODO filter infeasible options
fn get_options(p: P, ctxt: &Ctxt) -> Vec<E> {
    (0..(ctxt.n as u8)).collect()
}

fn submit_model(ctxt: &Ctxt) {
    present_model(ctxt.n, |x, y| match ctxt.classes[mk_p(x as E, y as E, ctxt.n) as usize] {
        Class::Defined(e) => e as usize,
        Class::Pending(_) => unreachable!(),
    });
}

fn mainloop(mut ctxt: Ctxt) {
    let Some(p) = select_p(&ctxt) else {
        submit_model(&ctxt);
        return;
    };

    for e in get_options(p, &ctxt) {
        let mut c = ctxt.clone();
        if propagate(p, e, &mut c).is_ok() {
            mainloop(c);
        }
    }
}

pub fn propagate(p: P, e: E, ctxt: &mut Ctxt) -> Result<(), ()> {
    if let Class::Defined(e2) = ctxt.classes[p as usize] && e == e2 { return Ok(()); }

    let Class::Pending(cs) = std::mem::replace(&mut ctxt.classes[p as usize], Class::Defined(e)) else {
        return Err(());
    };

    let x = px(p, ctxt.n);
    let y = py(p, ctxt.n);

    // spawn constraints!
    {
        // x*y = e.
        let (a, b, ba) = (y, x, e);
        visit_c11(a, b, ba, ctxt)?;
    }

    for c in cs {
        progress_c(c, x, y, e, ctxt)?;
    }
    Ok(())
}
