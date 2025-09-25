mod magma;
pub use magma::*;

mod matrix;
pub use matrix::*;

mod eq_dpll;
pub use eq_dpll::*;

mod sym_dpll;
pub use sym_dpll::*;

mod db;
pub use db::*;

fn main() {
    for i in 1..8 {
        println!("Looking for model size {i}:");
        sym_run(i);
    }
}
