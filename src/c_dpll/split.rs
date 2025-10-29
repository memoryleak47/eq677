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

pub fn split_models_via_row(ctxt: Ctxt) -> Vec<Ctxt> {
    if ctxt.n <= 1 { return vec![ctxt] }

    let mut out = Vec::new();

    // We choose 0 to maximize |C(0,0)|, thus if 0*0 = 0, then x*x = x holds generally.
    // Further, in this case we can still assume freshness.
    {
        let mut ctxt = ctxt.clone();
        for i in 0..ctxt.n {
            assert!(prove_triple(i, i, i, &mut ctxt).is_ok());
        }
        assert!(propagate(&mut ctxt).is_ok());
        out.push(ctxt);
    }

    for zero_orbit_size in 2..=ctxt.n {
        let mut ctxt = ctxt.clone();

        ctxt.nonfresh = ctxt.n;
        for i in 0..zero_orbit_size {
            assert!(prove_triple(0, i, (i+1)%zero_orbit_size, &mut ctxt).is_ok());
        }
        split_rest(1, zero_orbit_size, ctxt, &mut out);
    }

    out
}


// We can not build smaller cycles than last_size. last_size is monotonically growing. But not strictly so.
// next_idx is the next E that is undefined so far.
fn split_rest(last_size: E, next_idx: E, ctxt: Ctxt, out: &mut Vec<Ctxt>) {
    assert!(next_idx <= ctxt.n);

    if next_idx == ctxt.n {
        out.push(ctxt);
        return
    }

    let remaining = ctxt.n - next_idx;
    'outer: for next_cycle in last_size..=remaining {
        let mut ctxt = ctxt.clone();
        for i in 0..next_cycle {
            let ii = next_idx + i;
            let ii_1 = next_idx + (i+1)%next_cycle;
            if prove_triple(0, ii, ii_1, &mut ctxt).is_err() { continue 'outer; }
        }
        split_rest(next_cycle, next_idx + next_cycle, ctxt, out)
    }
}
