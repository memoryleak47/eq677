use crate::*;

pub fn funny_extend() {
    for q in 1.. {
        dbg!(q);

        for (l0, l1, l2, l3, l4, a, b) in itertools::iproduct!(0..q, 0..q, 0..q, 0..q, 0..q, 0..q, 0..q) {
            let l = [l0, l1, l2, l3, l4];
            let f = |x: EE, y: EE| -> EE {
                let o = (y.0 + 5 - x.0)%5;
                ((2*x.0+4*y.0)%5, (a*x.1 + b*y.1 + l[o])%q)
            };
            let m = GenericMagma {
                elems: itertools::iproduct!(0..5, 0..q).collect(),
                f_def: f,
            }.to_matrix();
            if m.is677() {
                let s = format!("{l:?}, a={a}, b={b}");
                present_model(m.n, &s, |x, y| m.f(x, y));
            }
        }
    }
}

pub fn funny_extend2() {
    for q in 1.. {
        dbg!(q);

        for (a, b) in itertools::iproduct!(0..q, 0..q) {
            let f = |x: EE, y: EE| -> EE {
                let o = (y.0 + 5 - x.0)%5;
                if o == 0 {
                    ((2*x.0+4*y.0)%5, (a*x.1 + b*y.1)%q)
                } else {
                    ((2*x.0+4*y.0)%5, (a*x.1 + b*y.1 + 1)%q)
                }
            };
            let m = GenericMagma {
                elems: itertools::iproduct!(0..5, 0..q).collect(),
                f_def: f,
            }.to_matrix();
            if m.is677() {
                let s = format!("extend(x, y) = {a}*x + {b}*y + (base(x)=base(y))");
                present_model(m.n, &s, |x, y| m.f(x, y));
            }
        }
    }
}
