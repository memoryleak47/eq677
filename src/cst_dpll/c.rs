use crate::cst_dpll::*;

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

// note: progress_c is called when we know the value that a constraint is waiting on.
//       visit_c* is called when we want to check whether we can call `progress_c`,
//       or rather store the constraint in some class.

pub fn progress_c(c: C, p: P, e: E, ctxt: &mut Ctxt) -> Result<(), ()> {
    match c {
        C::C11(a) => progress_c11(a, p, e, ctxt),
        _ => todo!(),
    }
}

fn progress_c11(a: E, p: P, e: E, ctxt: &mut Ctxt) -> Result<(), ()> {
    visit_c12(todo!(), todo!(), ctxt)
}

fn visit_c12(_: (), p: P, ctxt: &mut Ctxt) -> Result<(), ()> {
    todo!()
}
