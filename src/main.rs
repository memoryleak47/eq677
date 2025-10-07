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

fn main() {
    for i in 0..12 {
        println!("Looking for model size {i}:");
        c_run(i);
    }
}
