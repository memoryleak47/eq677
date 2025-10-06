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

mod cst_dpll;
pub use cst_dpll::*;

mod db;
pub use db::*;

mod parallel;
pub use parallel::*;

fn main() {
    for i in 0..12 {
        println!("Looking for model size {i}:");
        cst_run(i);
    }
}
