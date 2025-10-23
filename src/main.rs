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

mod c_dpll;
pub use c_dpll::*;

mod db;
pub use db::*;

mod parallel;
pub use parallel::*;

mod present;
pub use present::*;

mod linear;
pub use linear::*;

mod linmat;
pub use linmat::*;

mod affine;
pub use affine::*;

mod affmat;
pub use affmat::*;

fn main() {
    affmat_search();
}
