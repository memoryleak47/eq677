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

pub fn progress_c(c: C, x: E, y: E, e: E, ctxt: &mut Ctxt) -> Result<(), ()> {
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
            prove_triple(b, abab, a, ctxt)
        },
        C::C21(a) => {
            let b = x;
            let ba = y;
            let bba = e;
            visit_c22(a, b, ba, bba, ctxt)
        },
        C::C22(a, ba) => {
            let bba = x;
            let b = y;
            let bbab = e;
            prove_triple(ba, bbab, a, ctxt)
        },
    }
}

// C1
pub fn visit_c11(a: E, b: E, ba: E, ctxt: &mut Ctxt) -> Result<(), ()> {
    let class = &mut ctxt.classes[idx(ba, b, ctxt.n)];
    let v = class.value;
    if v == E::MAX {
        ctxt.trail.push(TrailEvent::PushC(ba, b));
        class.cs.push(C::C11(a));
        Ok(())
    } else {
        let bab = v;
        visit_c12(a, b, bab, ctxt)
    }
}

fn visit_c12(a: E, b: E, bab: E, ctxt: &mut Ctxt) -> Result<(), ()> {
    let i = idx(a, bab, ctxt.n);
    let class = &ctxt.classes[i];
    let v = class.value;
    if v == E::MAX {
        // a = b*(a*bab)
        let z = ctxt.xzy[idx(b, a, ctxt.n)];
        if z != E::MAX {
            return prove_triple(a, bab, z, ctxt);
        }

        ctxt.trail.push(TrailEvent::PushC(a, bab));
        ctxt.classes[i].cs.push(C::C12(b));
        Ok(())
    } else {
        let abab = v;
        prove_triple(b, abab, a, ctxt)
    }
}

// C2
pub fn visit_c21(a: E, b: E, ba: E, ctxt: &mut Ctxt) -> Result<(), ()> {
    let class = &mut ctxt.classes[idx(b, ba, ctxt.n)];
    let v = class.value;
    if v == E::MAX {
        ctxt.trail.push(TrailEvent::PushC(b, ba));
        class.cs.push(C::C21(a));
        Ok(())
    } else {
        let bba = v;
        visit_c22(a, b, ba, bba, ctxt)
    }
}

fn visit_c22(a: E, b: E, ba: E, bba: E, ctxt: &mut Ctxt) -> Result<(), ()> {
    let i = idx(bba, b, ctxt.n);
    let v = ctxt.classes[i].value;
    if v == E::MAX {
        // a = ba * (bba * b)
        let z = ctxt.xzy[idx(ba, a, ctxt.n)];
        if z != E::MAX {
            return prove_triple(bba, b, z, ctxt);
        }

        ctxt.trail.push(TrailEvent::PushC(bba, b));
        ctxt.classes[i].cs.push(C::C22(a, ba));
        Ok(())
    } else {
        let bbab = v;
        prove_triple(ba, bbab, a, ctxt)
    }
}
