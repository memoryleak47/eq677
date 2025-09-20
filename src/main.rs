mod magma;
pub use magma::*;

mod matrix;
pub use matrix::*;

mod eq_enum;
// pub use eq_enum::*;

mod eq_enum2;
pub use eq_enum2::*;

fn main() {
    for i in 1..9 {
        println!("Looking for model size {i}:");
        eq_run(i);
    }
}
