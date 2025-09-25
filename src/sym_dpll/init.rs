use crate::sym_dpll::*;

pub fn new_ctxts(n: usize) -> Vec<Ctxt> {
    let mut ctxt = Ctxt {
        xyz: Map::default(),
        xzy: Map::default(),
        usages: (0..n).map(|_| Vec::new()).collect(),
        unionfind: (0..n).collect(), // setup the initial 0..n ElemId classes.
        n,
        dirty_stack: Vec::new(),
        paradox: false,
        fresh: vec![true; n],
    };
    setup_constraints(&mut ctxt);
    split_models(ctxt)
}

pub fn setup_constraints(ctxt: &mut Ctxt) {
    for x in 0..ctxt.n {
        for y in 0..ctxt.n {
            // x = f(y, f(x, f(f(y, x), y)))
            let yx = add(y, x, ctxt);
            let yxy = add(yx, y, ctxt);
            let xyxy = add(x, yxy, ctxt);
            let yxyxy = add(y, xyxy, ctxt);
            union(x, yxyxy, ctxt);
        }
    }

    for x in 0..ctxt.n {
        for y in 0..ctxt.n {
            // x = f(yx, f(f(y, yx), y))
            let yx = add(y, x, ctxt);
            let yyx = add(y, yx, ctxt);
            let yyxy = add(yyx, y, ctxt);
            let yxyyxy = add(yx, yyxy, ctxt);
            union(x, yxyyxy, ctxt);
        }
    }
    rebuild(ctxt);
}

fn split_models(ctxt: Ctxt) -> Vec<Ctxt> {
    if ctxt.n == 1 { return vec![ctxt] }

    let mut out = Vec::new();

    {
        let mut ctxt = ctxt.clone();
        ctxt.fresh[0] = false;
        ctxt.fresh[1] = false;
        union(ctxt.xyz[&(0, 0)], 1, &mut ctxt);
        rebuild(&mut ctxt);
        assert!(!ctxt.paradox);
        out.push(ctxt);
    }

    {
        let mut ctxt = ctxt.clone();
        for i in 0..ctxt.n {
            union(ctxt.xyz[&(i, i)], i, &mut ctxt);
        }
        assert!(!ctxt.paradox);
        out.push(ctxt);
    }

    out
}
