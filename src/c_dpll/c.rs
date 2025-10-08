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
    match &mut ctxt.classes[idx(ba, b, ctxt.n)] {
        Class::Defined(bab) => visit_c12(a, b, *bab, ctxt),
        Class::Pending(cs) => {
            ctxt.trail.push(TrailEvent::PushC(ba, b));
            cs.push(C::C11(a));
        },
    }
}

fn visit_c12(a: E, b: E, bab: E, ctxt: &mut Ctxt) {
    match &mut ctxt.classes[idx(a, bab, ctxt.n)] {
        Class::Defined(abab) => ctxt.propagate_queue.push((b, *abab, a)),
        Class::Pending(cs) => {
            ctxt.trail.push(TrailEvent::PushC(a, bab));
            cs.push(C::C12(b));
        },
    }
}

// C2
pub fn visit_c21(a: E, b: E, ba: E, ctxt: &mut Ctxt) {
    match &mut ctxt.classes[idx(b, ba, ctxt.n)] {
        Class::Defined(bba) => visit_c22(a, b, ba, *bba, ctxt),
        Class::Pending(cs) => {
            ctxt.trail.push(TrailEvent::PushC(b, ba));
            cs.push(C::C21(a));
        },
    }
}

fn visit_c22(a: E, b: E, ba: E, bba: E, ctxt: &mut Ctxt) {
    match &mut ctxt.classes[idx(bba, b, ctxt.n)] {
        Class::Defined(bbab) => ctxt.propagate_queue.push((ba, *bbab, a)),
        Class::Pending(cs) => {
            ctxt.trail.push(TrailEvent::PushC(bba, b));
            cs.push(C::C22(a, ba));
        },
    }
}
