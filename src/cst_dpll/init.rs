use crate::cst_dpll::*;

pub fn build_ctxt(n: usize) -> Ctxt {
    let mut ctxt = Ctxt {
        trail: Vec::new(),
        n,
        classes: std::iter::repeat(Class::Pending(smallvec![C::C1M1(C1M1), C::C2M1(C2M1)]))
            .take(n*n)
            .collect(),
    };
    ctxt
}
