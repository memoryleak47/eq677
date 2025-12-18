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

mod uf;
pub use uf::*;

fn main() {
    setup_panic_hook();
    let _timer = Timer::new();

    for (name, m) in db() {
        println!("{name}:");
        let pairs: Vec<(usize, usize)> = itertools::iproduct!(0..m.n, 0..m.n).collect();
        let uf_255 = uf(&m, Equ::E255);
        let uf_677 = uf(&m, Equ::E677);
        for p1 in &pairs {
            for p2 in &pairs {
                let eq_255 = uf_255.iter().any(|x| x.contains(&p1) && x.contains(&p2));
                let eq_677 = uf_677.iter().any(|x| x.contains(&p1) && x.contains(&p2));
                let implies = |x: bool, y: bool| (!x || y);
                assert!(implies(eq_255, eq_677));
            }
        }
    }
}
