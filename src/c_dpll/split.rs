use crate::c_dpll::*;

pub fn split_models(ctxt: Ctxt) -> Vec<Ctxt> {
    if ctxt.n <= 1 { return vec![ctxt] }

    let mut out = Vec::new();

    {
        let mut ctxt = ctxt.clone();
        ctxt.nonfresh = 2;
        assert!(prove_triple(0, 0, 1, &mut ctxt).is_ok());
        assert!(propagate(&mut ctxt).is_ok());
        out.push(ctxt);
    }

    {
        let mut ctxt = ctxt.clone();
        for i in 0..ctxt.n {
            assert!(prove_triple(i, i, i, &mut ctxt).is_ok());
        }
        assert!(propagate(&mut ctxt).is_ok());
        out.push(ctxt);
    }

    out
}

