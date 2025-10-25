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

pub fn all_search() {
    let searches = vec![linear_search, linmat_search, affine_search, affmat_search, poly_search];
    into_par_for_each(searches, |x| x());
}
