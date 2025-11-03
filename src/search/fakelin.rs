use crate::*;

/*
    f(x, y) = (x*c0)%c1 + y*c2 + c3
*/

pub fn fakelin_search() {
    for p in 0.. {
        dbg!(p);
        for (c0, c1, c2, c3) in itertools::iproduct!(0..p, 1..p, 0..p, 0..p) {
            let f_def = |x, y| ((x*c0)%c1 + y*c2 + c3)%p;
            let f = FnMagma {
                n: p,
                f_def,
            };
            if f.is677() {
                present_model(p, "fakelin", f_def);
            }
        }
    }
}

/*
    f(x, y) = x*c0 + y*c1 + c2 + (x == c3) * c4
*/

pub fn fakelin_search2() {
    for p in 0.. {
        dbg!(p);
        for (c0, c1, c2, c3, c4) in itertools::iproduct!(0..p, 0..p, 0..p, 0..p, 1..p) {
            let f_def = |x, y| (x*c0 + y*c1 + c2 + ((x == c3) as usize) * c4)%p;
            let f = FnMagma {
                n: p,
                f_def,
            };
            if f.is677() {
                present_model(p, "fakelin2", f_def);
            }
        }
    }
}
