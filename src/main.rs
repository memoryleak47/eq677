mod magma;
pub use magma::*;

mod matrix;
pub use matrix::*;

mod eq_dpll;
pub use eq_dpll::*;

mod db;

fn main() {
    for i in 1..11 {
        println!("Looking for model size {i}:");
        eq_run(i);
    }
}
