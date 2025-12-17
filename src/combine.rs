use crate::*;

pub fn combine_search() {
    let mut goodies = Vec::new();
    for (name, m) in db() {
        if good(&m) {
            print!("{name}, ");
            goodies.push(m);
        }
    }

    for (_, a) in db() {
        if a.n < 2 { continue }
        if a.n > 20 { continue }
        for (_, b) in db() {
            if a.n != b.n { continue }

            for g in &goodies {
                if g.n < 2 { continue }
                if g.n > 20 { continue }
                let a = a.shuffle();
                let b = b.shuffle();
                let g = g.shuffle();
                let m = GenericMagma {
                    elems: itertools::iproduct!(0..g.n, 0..a.n).collect(),
                    f_def: |x: (usize, usize), y: (usize, usize)| {
                        let z0 = g.f(x.0, y.0);
                        let z1 = 
                            if x.0 == y.0 { a.f(x.1, y.1) }
                            else { b.f(x.1, y.1) };
                        (z0, z1)
                    },
                }.to_matrix();
                if m.is677() {
                    dbg!(a.n);
                    dbg!(g.n);
                    present_model(m.n, "k", |x, y| m.f(x, y));
                }
            }
        }
    }
}
