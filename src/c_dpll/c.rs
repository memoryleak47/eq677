use crate::c_dpll::*;

// Constraints

#[derive(Clone, Copy)]
pub enum C {
    C11(/*a*/ E),           // a = b*(a*(ba*b))
                            //            x*y
    C12(/*b*/ E),           // a = b*(a*bab)
                            //        x*y

    C21(/*a*/ E),           // a = ba * ((b*ba) * b)
                            //            x*y
    C22(/*a*/ E, /*ba*/ E), // a = ba * (bba * b)
                            //            x  * y
}

pub fn progress_c(c: C, x: E, y: E, e: E, ctxt: &mut Ctxt) {
    match c {
        C::C11(a) => {
            let _ba = x;
            let b = y;
            let bab = e;
            visit_c12(a, b, bab, ctxt)
        }
        C::C12(b) => {
            let a = x;
            let bab = y;
            let abab = e;
            ctxt.propagate_queue.push((b, abab, a));
        },
        C::C21(a) => {
            let b = x;
            let ba = y;
            let bba = e;
            visit_c22(a, b, ba, bba, ctxt);
        },
        C::C22(a, ba) => {
            let bba = x;
            let b = y;
            let bbab = e;
            ctxt.propagate_queue.push((ba, bbab, a));
        },
    }
}

// C1
pub fn visit_c11(a: E, b: E, ba: E, ctxt: &mut Ctxt) {
    let class = &mut ctxt.classes[idx(ba, b, ctxt.n)];
    if class.value == E::MAX {
        ctxt.trail.push(TrailEvent::PushC(ba, b));
        class.cs.push(C::C11(a));
    } else {
        let bab = class.value;
        visit_c12(a, b, bab, ctxt);
    }
}

fn visit_c12(a: E, b: E, bab: E, ctxt: &mut Ctxt) {
    let i = idx(a, bab, ctxt.n);
    let class = &ctxt.classes[i];
    if class.value == E::MAX {
        for z in 0..ctxt.n {
            if ctxt.classes[idx(b, z, ctxt.n)].value == a {
                ctxt.propagate_queue.push((a, bab, z));
                return;
            }
        }

        ctxt.trail.push(TrailEvent::PushC(a, bab));
        ctxt.classes[i].cs.push(C::C12(b));
    } else {
        let abab = class.value;
        ctxt.propagate_queue.push((b, abab, a));
    }
}

// C2
pub fn visit_c21(a: E, b: E, ba: E, ctxt: &mut Ctxt) {
    let class = &mut ctxt.classes[idx(b, ba, ctxt.n)];
    if class.value == E::MAX {
        ctxt.trail.push(TrailEvent::PushC(b, ba));
        class.cs.push(C::C21(a));
    } else {
        let bba = class.value;
        visit_c22(a, b, ba, bba, ctxt);
    }
}

fn visit_c22(a: E, b: E, ba: E, bba: E, ctxt: &mut Ctxt) {
    let i = idx(bba, b, ctxt.n);
    if ctxt.classes[i].value == E::MAX {
        // a = ba * (bba * b)
        for z in 0..ctxt.n {
            if ctxt.classes[idx(ba, z, ctxt.n)].value == a {
                ctxt.propagate_queue.push((bba, b, z));
                return;
            }
        }

        ctxt.trail.push(TrailEvent::PushC(bba, b));
        ctxt.classes[i].cs.push(C::C22(a, ba));
    } else {
        let bbab = ctxt.classes[i].value;
        ctxt.propagate_queue.push((ba, bbab, a));
    }
}
