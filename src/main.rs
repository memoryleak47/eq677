mod magma;
pub use magma::*;

mod matrix;
pub use matrix::*;

mod eq_enum;
pub use eq_enum::*;

fn main() {
    for i in 1..8 {
        println!("Looking for model size {i}:");
        eq_run(i);
    }
}
