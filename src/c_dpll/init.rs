use crate::c_dpll::*;

pub fn build_ctxt(n: usize) -> Ctxt {
    let class_xy = ClassXY {
        value: E::MAX,
        cs: SmallVec::new(),
        score: -1,
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
        nonfresh: 0,
        propagate_queue: Vec::new(),
    };
    for x in 0..ctxt.n {
        for y in 0..ctxt.n {
            ctxt.classes_xy[idx(x, y, ctxt.n)].score = compute_score(x, y, &ctxt);
        }
    }
    ctxt
}
