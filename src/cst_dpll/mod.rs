use crate::*;
use smallvec::SmallVec;

mod init;
use init::*;

mod p;
use p::*;

mod c;
use c::*;

mod run;
pub use run::*;

// identifies an element.
type E = u8;

// identifies an (x,y)-position of Es.
type P = u16;

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
