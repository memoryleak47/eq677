use crate::cst_dpll::*;

pub fn cst_run(n: usize) {
    let ctxt = build_ctxt(n);
    mainloop(ctxt);
}

// returns None if we are done.
// TODO selection heuristic.
fn select_p(ctxt: &Ctxt) -> Option<P> {
    for x in 0..(ctxt.n as u8) {
        for y in 0..(ctxt.n as u8) {
            let p = mk_p(x, y, ctxt.n);
            if let Class::Pending(_) = ctxt.classes[p as usize] {
                return Some(p);
            }
        }
    }
    None
}

// TODO filter infeasible options
fn get_options(p: P, ctxt: &Ctxt) -> Vec<E> {
    (0..(ctxt.n as u8)).collect()
}

fn mainloop(mut ctxt: Ctxt) {
    let Some(p) = select_p(&ctxt) else {
        println!("model found!");
        return;
    };

    for e in get_options(p, &ctxt) {
        let mut c = ctxt.clone();
        if propagate(p, e, &mut c).is_ok() {
            mainloop(c);
        }
    }
}

fn propagate(p: P, e: E, ctxt: &mut Ctxt) -> Result<(), ()> {
    let Class::Pending(cs) = std::mem::replace(&mut ctxt.classes[p as usize], Class::Decided(e)) else {
        panic!("double-defined class!");
    };
    for c in cs {
        progress_c(c, p, e, ctxt)?;
    }
    Ok(())
}
