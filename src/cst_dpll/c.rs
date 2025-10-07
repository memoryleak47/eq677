use crate::cst_dpll::*;

// Constraints

#[derive(Clone, Copy)]
pub enum C {
    C1M1(C1M1),
    C1M2(C1M2),
    C1M3(C1M3),

    C2M1(C2M1),
    C2M2(C2M2),
    C2M3(C2M3),
}

#[derive(Clone, Copy)]
pub struct C1M1;                    // a = b*(a*((b*a)*b))
                                    //            x*y
#[derive(Clone, Copy)]
pub struct C1M2(/*a*/ E);           // a = b*(a*(ba*b))
                                    //            x*y
#[derive(Clone, Copy)]
pub struct C1M3(/*b*/ E);           // a = b*(a*bab)
                                    //        x*y

#[derive(Clone, Copy)]
pub struct C2M1;                    // a = (b*a) * ((b*(b*a)) * b)
                                    //                  x*y
#[derive(Clone, Copy)]
pub struct C2M2(/*a*/ E);           // a = ba * ((b*ba) * b)
                                    //            x*y
#[derive(Clone, Copy)]
pub struct C2M3(/*a*/ E, /*ba*/ E); // a = ba * (bba * b)
                                    //            x  * y

// note: progress_c is called when we know the value that a constraint is waiting on.
//       visit_c*m* is called when we want to check whether we can call `progress_c`,
//       or rather store the constraint in some class.

pub fn progress_c(c: C, p: P, e: E, ctxt: &mut Ctxt) -> Result<(), ()> {
    match c {
        C::C1M1(c) => progress_c1m1(c, p, e, ctxt),
        _ => todo!(),
    }
}

fn progress_c1m1(_: C1M1, p: P, e: E, ctxt: &mut Ctxt) -> Result<(), ()> {
    let c = C1M2(py(p, ctxt.n));
    visit_c1m2(c, todo!(), ctxt)
}

fn visit_c1m2(c: C1M2, p: P, ctxt: &mut Ctxt) -> Result<(), ()> {
    todo!()
}
