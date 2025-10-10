use crate::c_dpll::*;

pub fn build_ctxt(n: usize) -> Ctxt {
    let class = Class {
        value: E::MAX,
        cs: SmallVec::new(),
    };
    let mut ctxt = Ctxt {
        trail: Vec::new(),
        n: n as E,
        xzy: std::iter::repeat(E::MAX)
            .take(n*n)
            .collect(),
        classes: std::iter::repeat(class)
            .take(n*n)
            .collect(),
        fresh: std::iter::repeat(true)
            .take(n)
            .collect(),
        propagate_queue: Vec::new(),
    };
    ctxt
}
