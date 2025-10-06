use crate::cst_dpll::*;

pub fn build_ctxt(n: usize) -> Ctxt {
    let mut ctxt = Ctxt {
        trail: Vec::new(),
        n,
        classes: std::iter::repeat(Class::Pending(SmallVec::new()))
            .take(n*n)
            .collect(),
    };
    add_constraints(n, &mut ctxt);
    ctxt
}

fn add_constraints(n: usize, ctxt: &mut Ctxt) {
    for x in 0..(n as u8) {
        for y in 0..(n as u8) {
            let p = mk_p(x, y, n);
            let mut v = SmallVec::new();
            v.push(C::C1M0(y, x)); // TODO right way around?
            v.push(C::C2M0(y, x));
            ctxt.classes[p as usize] = Class::Pending(v);
        }
    }
}
