use crate::*;
use rayon::prelude::*;

// This flag allows me to turn off multi-thrading globally.
// Useful for flamegraphs and deterministic debugging.
const PAR: bool = false;

pub fn par_for_each<T>(x: &[T], f: impl Fn(&T) + Send + Sync) where T: Send + Sync + 'static {
    if PAR {
        x.par_iter().for_each(f);
    } else {
        x.iter().for_each(f);
    }
}

pub fn into_par_for_each<T>(x: Vec<T>, f: impl Fn(T) + Send + Sync) where T: Send {
    if PAR {
        x.into_par_iter().for_each(f);
    } else {
        x.into_iter().for_each(f);
    }
}
