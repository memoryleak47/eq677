use crate::*;

// identifies an element.
type E = usize;

// identifies an (x,y)-position of Es.
type P = usize;

pub fn cst_run(n: usize) {
    todo!()
}

struct Class {
    c1: C1,
    c2: C2,
    value: Option<E>,

    // The constraints that await our decision.
    listening_c1: Vec<P>,
    listening_c2: Vec<P>,
}

enum TrailEvent {
    ChangeC1(P, C1), // C1 is the old state.
    ChangeC2(P, C2), // C2 is the old state.
    Decide(P, Vec<E>) // Vec<E> are the other options I should try.
    // TODO add more
}

struct Ctxt {
    trail: Vec<TrailEvent>,

    // indexed via `idx`.
    classes: Vec<Class>,
}

enum C1 {
    M0,          // x = y*(x*((y*x)*y))
    M1(/*a*/ E), // x = y*(x*(a*y))
    M2(/*a*/ E), // x = y*(x*a)
    M3(/*a*/ E), // x = y*a
}

enum C2 {
    M0,                    // x = (y*x) * ((y*(y*x)) *y)
    M1(/*yx*/ E),          // x = yx * ((y*yx) *y)
    M2(/*yx*/ E, /*a*/ E), // x = yx * (a * y)
    M3(/*yx*/ E, /*a*/ E), // x = yx * a
}
