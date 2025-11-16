use crate::*;
use smallvec::{SmallVec, smallvec};

mod init;
use init::*;

mod c;
use c::*;

mod run;
use run::*;
pub use run::{tinv_run, tinv_search};

// identifies an element.
type E = u8;

#[derive(Clone)]
struct ClassXY {
    value: E, // is E::MAX when undecided.
    cs: SmallVec<[CXY; 7]>, // the constraints that currently wait on us.
    score: i32, // a cache for the base_score.
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
    propagate_queue: Vec<(E, E, E)>,
}

#[derive(Clone)]
enum TrailEvent {
    Decision(E, E, E),
    DefineClass(E, E),
    PushCXY(E, E),
    PushCXZ(E, E),
}

fn idx(x: E, y: E, n: E) -> usize {
    (x as usize) + (n as usize) * (y as usize)
}

impl Ctxt {
    pub fn matrix(&self) -> MatrixMagma {
        MatrixMagma::by_fn(self.n as usize, |x, y| {
             let i = idx(x as E, y as E, self.n);
             let v = self.classes_xy[i].value;
             if v == E::MAX { usize::MAX } else { v as _ }
        })
    }

    pub fn dump(&self) { self.matrix().dump() }
    pub fn cycle_dump(&self) { self.matrix().cycle_dump() }
}
