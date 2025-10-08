use crate::c_dpll::*;

pub fn build_ctxt(n: usize) -> Ctxt {
    let mut ctxt = Ctxt {
        trail: Vec::new(),
        n: n as E,
        classes: std::iter::repeat(Class::Pending(SmallVec::new()))
            .take(n*n)
            .collect(),
        propagate_queue: Vec::new(),
    };
    ctxt
}
