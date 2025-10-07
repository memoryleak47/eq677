use crate::cst_dpll::*;

pub fn build_ctxt(n: usize) -> Ctxt {
    let mut ctxt = Ctxt {
        trail: Vec::new(),
        n,
        classes: std::iter::repeat(Class::Pending(SmallVec::new()))
            .take(n*n)
            .collect(),
    };
    ctxt
}
