use crate::c_dpll::*;

pub fn build_ctxt(n: usize) -> Ctxt {
    let class_xy = ClassXY {
        value: E::MAX,
        cs: SmallVec::new(),
        score: -1,
        heap_index: 0,
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
        heap: Vec::new(),
    };

    for x in 0..ctxt.n {
        for y in 0..ctxt.n {
            heap_push(x, y, 0, &mut ctxt);
        }
    }

    ctxt
}
