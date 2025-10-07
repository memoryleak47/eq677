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
