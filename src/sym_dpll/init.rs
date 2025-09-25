use crate::sym_dpll::*;

pub fn new_ctxt(n: usize) -> Ctxt {
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
    ctxt
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
