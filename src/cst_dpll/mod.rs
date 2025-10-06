use crate::*;
use smallvec::SmallVec;

mod init;
use init::*;

mod p;
use p::*;

mod run;
pub use run::*;

// identifies an element.
type E = u8;

// identifies an (x,y)-position of Es.
type P = u16;

// Constraint
#[derive(Clone)]
enum C {
    C1M0(/*x*/ E, /*y*/ E),                     // x = y*(x*((y*x)*y))
    C1M1(/*x*/ E, /*y*/ E, /*a*/ E),            // x = y*(x*(  a  *y))
    C1M2(/*x*/ E, /*y*/ E, /*a*/ E),            // x = y*(x*a)

    C2M0(/*x*/ E, /*y*/ E),                     // x = (y*x) * ((y*(y*x)) * y)
    C2M1(/*x*/ E, /*y*/ E, /*yx*/ E),           // x = yx    * ((y*  yx)  * y)
    C2M2(/*x*/ E, /*y*/ E, /*yx*/ E, /*a*/ E),  // x = yx    * (a  * y)
}

#[derive(Clone)]
enum Class {
    Evaluated(E),
    Pending(SmallVec<[C; 4]>), // the constraints that currently wait on us.
}

struct Ctxt {
    trail: Vec<TrailEvent>,
    classes: Box<[Class]>,
    n: usize,
}

enum TrailEvent {
    TickC(/*old pos*/ P, /*new pos*/ P, /*old c*/ C),
    Decide(P, Vec<E>) // Vec<E> are the other options I should try.
    // TODO add more
}
