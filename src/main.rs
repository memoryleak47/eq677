#![feature(explicit_tail_calls)]

#![allow(unused)] // heh
#![allow(private_interfaces)]
#![allow(irrefutable_let_patterns)]

mod magma;
pub use magma::*;

mod search;
pub use search::*;

mod matrix;
pub use matrix::*;

mod sym_dpll;
pub use sym_dpll::*;

mod c_dpll;
pub use c_dpll::*;

mod composite;
pub use composite::*;

mod db;
pub use db::*;

mod conj;
pub use conj::*;

mod parallel;
pub use parallel::*;

mod present;
pub use present::*;

mod kb;
pub use kb::*;

mod twee;
pub use twee::*;

mod twee_sys;
pub use twee_sys::*;

mod timer;
pub use timer::*;

mod fo;
pub use fo::*;

mod analysis;
pub use analysis::*;

mod one_orbit;
pub use one_orbit::*;

mod one_orbit2;
pub use one_orbit2::*;

mod autom_search;
pub use autom_search::*;

mod load;
pub use load::*;

mod check;
pub use check::*;

mod prop_combo;
pub use prop_combo::*;

mod enumerate;
pub use enumerate::*;

mod combine;
pub use combine::*;

fn main() {
    setup_panic_hook();
    let _timer = Timer::new();

    let models = split_models(build_ctxt(9, Vec::new()));
    for m in &models {
        dbg!(tree_search(m));
    }
}
