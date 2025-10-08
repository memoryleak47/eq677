use crate::c_dpll::*;

pub fn split_models(ctxt: Ctxt) -> Vec<Ctxt> {
    if ctxt.n <= 1 { return vec![ctxt] }

    let mut out = Vec::new();

    {
        let mut ctxt = ctxt.clone();
        ctxt.fresh[0] = false;
        ctxt.fresh[1] = false;
        ctxt.propagate_queue.push((0, 0, 1));
        out.push(ctxt);
    }

    {
        let mut ctxt = ctxt.clone();
        for i in 0..ctxt.n {
            ctxt.propagate_queue.push((i, i, i));
        }
        out.push(ctxt);
    }

    out
}

