use crate::*;
use smallvec::{SmallVec, smallvec};

mod init;
use init::*;

mod c;
use c::*;

mod split;
use split::*;

mod run;
pub use run::*;

// identifies an element.
type E = u8;

#[derive(Clone)]
struct Class {
    value: E, // is E::MAX when undecided.
    cs: SmallVec<[C; 7]>, // the constraints that currently wait on us.
}

#[derive(Clone)]
struct Ctxt {
    trail: Vec<TrailEvent>,
    classes: Box<[Class]>,
    xzy: Box<[E]>, // indexed by `idx(x,z)`
    n: E,
    fresh: Box<[bool]>,
    propagate_queue: Vec<(E, E, E)>,
}

#[derive(Clone)]
enum TrailEvent {
    Decision(E, E, Vec<E>),
    DefineClass(E, E),
    Defresh(E),
    PushC(E, E),
}

fn idx(x: E, y: E, n: E) -> usize {
    (x as usize) + (n as usize) * (y as usize)
}
