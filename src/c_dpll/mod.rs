use crate::*;
use smallvec::{SmallVec, smallvec};

mod init;
use init::*;

mod c;
use c::*;

mod split;
use split::*;

mod heap;
use heap::*;

mod run;
pub use run::*;

// identifies an element.
type E = u8;

#[derive(Clone)]
struct ClassXY {
    value: E, // is E::MAX when undecided.
    cs: SmallVec<[CXY; 7]>, // the constraints that currently wait on us.
    heap_index: HeapIdx,
    score: i32,
}

#[derive(Clone)]
struct ClassXZ {
    value: E,
    cs: SmallVec<[CXZ; 7]>,
}

#[derive(Clone)]
struct Ctxt {
    trail: Vec<TrailEvent>,
    classes_xy: Box<[ClassXY]>,
    classes_xz: Box<[ClassXZ]>, // indexed by `idx(x,z)`
    n: E,
    fresh: Box<[bool]>,
    propagate_queue: Vec<(E, E, E)>,
    heap: Vec<XYIdx>,
}

#[derive(Clone)]
enum TrailEvent {
    Decision(E, E, Vec<E>),
    DefineClass(E, E),
    Defresh(E),
    PushCXY(E, E),
    PushCXZ(E, E),
}

fn idx(x: E, y: E, n: E) -> XYIdx {
    (x as usize) + (n as usize) * (y as usize)
}
