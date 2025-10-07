use crate::*;
use smallvec::{SmallVec, smallvec};

mod init;
use init::*;

mod c;
use c::*;

mod run;
pub use run::*;

// identifies an element.
type E = u8;

#[derive(Clone)]
enum Class {
    Defined(E),
    Pending(SmallVec<[C; 7]>), // the constraints that currently wait on us.
}

#[derive(Clone)]
struct Ctxt {
    trail: Vec<TrailEvent>,
    classes: Box<[Class]>,
    n: E,
}

#[derive(Clone)]
enum TrailEvent {}

fn idx(x: E, y: E, n: E) -> usize {
    (x as usize) + (n as usize) * (y as usize)
}
