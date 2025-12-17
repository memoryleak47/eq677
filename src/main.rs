#![feature(explicit_tail_calls)]

#![allow(unused)] // heh
#![allow(private_interfaces)]
#![allow(irrefutable_let_patterns)]

mod magma;
pub use magma::*;

mod matrix;
pub use matrix::*;

mod eq_dpll;
pub use eq_dpll::*;

mod sym_dpll;
pub use sym_dpll::*;

mod tinv_dpll;
pub use tinv_dpll::*;

mod semitinv_dpll;
pub use semitinv_dpll::*;

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

mod search;
pub use search::*;

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

fn good(m: &MatrixMagma) -> bool {
    if !m.is_idempotent() { return false }
    for x in 0..m.n {
        for y in 0..m.n {
            if x == y { continue }
            let mut eq = false;
            let f = |x, y| {
                if x == y { eq = true; }
                m.f(x, y)
            };
            assert!(x == m.f(y, m.f(x, m.f(m.f(y, x), y))));
            if eq { return false }
        }
    }
    true
}

fn main() {
    setup_panic_hook();
    let _timer = Timer::new();

    combine_search();
    // funny_extend2();
}
