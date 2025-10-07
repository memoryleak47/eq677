use crate::cst_dpll::*;

pub fn cst_run(n: usize) {
    let ctxt = build_ctxt(n);
    mainloop(ctxt);
}

// returns None if we are done.
fn select_p(ctxt: &Ctxt) -> Option<P> {
    todo!()
}

fn get_options(p: P, ctxt: &Ctxt) -> Vec<E> {
    todo!()
}

fn mainloop(mut ctxt: Ctxt) {
    let Some(p) = select_p(&ctxt) else {
        todo!("print model");
    };

    for e in get_options(p, &ctxt) {
        let mut c = ctxt.clone();
        if propagate(p, e, &mut c).is_ok() {
            mainloop(c);
        }
    }
}

fn propagate(p: P, e: E, ctxt: &mut Ctxt) -> Result<(), ()> {
    todo!()
}
