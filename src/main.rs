mod magma;
pub use magma::*;

mod matrix;
pub use matrix::*;

mod eq_dpll;
pub use eq_dpll::*;

mod db;
pub use db::*;

fn main() {
    for i in 1..10 {
        println!("Looking for model size {i}:");
        eq_run(i);
    }
}
