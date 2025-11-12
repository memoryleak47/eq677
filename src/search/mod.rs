use crate::*;

mod linear;
pub use linear::*;

mod linmat;
pub use linmat::*;

mod affine;
pub use affine::*;

mod affmat;
pub use affmat::*;

mod poly;
pub use poly::*;

mod bij;
pub use bij::*;

mod tinv;
pub use tinv::*;

mod fakelin;
pub use fakelin::*;

mod complex;
pub use complex::*;

pub fn all() {
    let mut handles = Vec::new();
    for s in [linear_search, linmat_search, affine_search, affmat_search, poly_search, bij_plus_search, bij_mul_search, c_search, db_search, tinv_search, complex_search] {
        handles.push(std::thread::spawn(s));
    }
    for h in handles {
        h.join().unwrap();
    }
}
