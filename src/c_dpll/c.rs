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
            let a = a;
            let b = y;
            let bab = e;
            visit_c12(a, b, bab, ctxt)
        }
        C::C12(b) => {
            let a = x;
            let bab = y;
            let abab = e;
            propagate(b, abab, a, ctxt)
        },
        _ => todo!(),
    }
}

pub fn visit_c11(a: E, b: E, ba: E, ctxt: &mut Ctxt) -> Result<(), ()> {
    match &mut ctxt.classes[idx(ba, b, ctxt.n)] {
        Class::Defined(bab) => return visit_c12(a, b, *bab, ctxt),
        Class::Pending(cs) => cs.push(C::C11(a)),
    }
    Ok(())
}

fn visit_c12(a: E, b: E, bab: E, ctxt: &mut Ctxt) -> Result<(), ()> {
    match &mut ctxt.classes[idx(a, bab, ctxt.n)] {
        Class::Defined(abab) => return propagate(b, *abab, a, ctxt),
        Class::Pending(cs) => cs.push(C::C12(b)),
    }
    Ok(())
}
