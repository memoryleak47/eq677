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

pub fn all_search() {
    let searches = vec![linear_search, linmat_search, affine_search, affmat_search, poly_search, bij_plus_search, bij_mul_search, c_search];
    into_par_for_each(searches, |x| x());
}
