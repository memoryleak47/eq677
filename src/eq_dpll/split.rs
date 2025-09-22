use crate::eq_dpll::*;

pub fn split_models(ctxt: Ctxt) -> Vec<Ctxt> {
    if ctxt.n == 1 { return vec![ctxt] }

    let mut out = Vec::new();

    {
        let mut ctxt = ctxt.clone();
        ctxt.fresh[0] = false;
        ctxt.fresh[1] = false;
        if propagate((0, 0), 1, &mut ctxt).is_ok() {
            out.push(ctxt);
        }
    }

    {
        let mut ctxt = ctxt.clone();
        let mut ok = true;
        for i in 0..ctxt.n {
            if propagate((i, i), i, &mut ctxt).is_err() {
                ok = false;
                break;
            }
        }
        if ok {
            out.push(ctxt);
        }
    }

    out
}

