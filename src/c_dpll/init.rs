use crate::c_dpll::*;

pub fn build_ctxt(n: usize) -> Ctxt {
    let class_xy = ClassXY {
        value: E::MAX,
        cs: SmallVec::new(),
    };
    let class_xz = ClassXZ {
        value: E::MAX,
        cs: SmallVec::new(),
    };
    let mut ctxt = Ctxt {
        trail: Vec::new(),
        n: n as E,
        classes_xz: std::iter::repeat(class_xz)
            .take(n*n)
            .collect(),
        classes_xy: std::iter::repeat(class_xy)
            .take(n*n)
            .collect(),
        fresh: std::iter::repeat(true)
            .take(n)
            .collect(),
        propagate_queue: Vec::new(),
    };
    ctxt
}
