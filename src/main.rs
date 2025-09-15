mod magma;
pub use magma::*;

mod matrix;
pub use matrix::*;

fn main() {
    let m = MatrixMagma::by_fn(5, |x, y| (2*x + 4*y) % 5);
    dbg!(m.is667());
    dbg!(m.is225());
    
}
