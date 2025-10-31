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

mod twee;
pub use twee::*;

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
    nonfresh: E, // The number of nonfresh elems. An element e is fresh, if e >= nonfresh.
    propagate_queue: Vec<(E, E, E)>,
    chosen_per_row: Box<[E]>,
    yxx: Box<[E]>, // y := yxx[x] where y*x = x, E::MAX means undefined.
}

#[derive(Clone)]
enum TrailEvent {
    Decision(E, E, E),
    DefineClass(E, E),
    Defresh,
    PushCXY(E, E),
    PushCXZ(E, E),
}

fn idx(x: E, y: E, n: E) -> usize {
    (x as usize) + (n as usize) * (y as usize)
}

impl Ctxt {
    pub fn dump(&self) {
        let m = MatrixMagma::by_fn(self.n as usize, |x, y| {
            let i = idx(x as E, y as E, self.n);
            let v = self.classes_xy[i].value;
            if v == E::MAX { usize::MAX } else { v as _ }
        });
        m.dump();
    }
}
