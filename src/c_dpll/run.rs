use crate::c_dpll::*;

pub fn c_run(n: usize) {
    let ctxt = build_ctxt(n);
    mainloop(ctxt);
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
    (0..ctxt.n).collect()
}

fn submit_model(ctxt: &Ctxt) {
    present_model(ctxt.n as usize, |x, y| match ctxt.classes[idx(x as E, y as E, ctxt.n)] {
        Class::Defined(e) => e as usize,
        Class::Pending(_) => unreachable!(),
    });
}

fn mainloop(mut ctxt: Ctxt) {
    let Some((x, y)) = select_p(&ctxt) else {
        submit_model(&ctxt);
        return;
    };

    for e in get_options(x, y, &ctxt) {
        let mut c = ctxt.clone();
        if propagate(x, y, e, &mut c).is_ok() {
            mainloop(c);
        }
    }
}

pub fn propagate(x: E, y: E, e: E, ctxt: &mut Ctxt) -> Result<(), ()> {
    let class = &mut ctxt.classes[idx(x, y, ctxt.n)];
    let cs = match class {
        Class::Defined(e2) => return if e == *e2 { Ok(()) } else { Err(()) },
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
        visit_c11(a, b, ba, ctxt)?;
    }

    for c in cs {
        progress_c(c, x, y, e, ctxt)?;
    }
    Ok(())
}
