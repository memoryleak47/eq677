use crate::c_dpll::*;

pub fn mk_p(x: E, y: E, n: usize) -> P {
    let x = x as P;
    let y = y as P;
    let n = n as P;

    x + n * y
}

pub fn px(p: P, n: usize) -> E {
    let n = n as P;
    (p%n) as E
}

pub fn py(p: P, n: usize) -> E {
    let n = n as P;
    (p/n) as E
}

#[test]
fn ptest() {
    let n = 17;
    let p = mk_p(14, 13, n);
    assert_eq!(px(p, n), 14);
    assert_eq!(py(p, n), 13);
}
