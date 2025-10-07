use crate::cst_dpll::*;

// Constraints

#[derive(Clone)]
pub enum C {
    C1M0(/*x*/ E, /*y*/ E),                     // x = y*(x*((y*x)*y))
    C1M1(/*x*/ E, /*y*/ E, /*a*/ E),            // x = y*(x*(  a  *y))
    C1M2(/*x*/ E, /*y*/ E, /*a*/ E),            // x = y*(x*a)

    C2M0(/*x*/ E, /*y*/ E),                     // x = (y*x) * ((y*(y*x)) * y)
    C2M1(/*x*/ E, /*y*/ E, /*yx*/ E),           // x = yx    * ((y*  yx)  * y)
    C2M2(/*x*/ E, /*y*/ E, /*yx*/ E, /*a*/ E),  // x = yx    * (a  * y)
}

