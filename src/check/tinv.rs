use crate::*;
use std::fmt::Write;

pub fn tinv_chk(m: &MatrixMagma) -> Option<Vec<usize>> {
    let n = m.n;
    let g = m.autom_group();
    let p: Vec<usize> = g.iter().find_map(|g| {
        let mut cycles = bij_to_cycles(0, m.n, |i| g[i]);
        if cycles.len() != 1 { return None }

        cycles.pop()
    })?;

    let m2 = m.permute(p);

    let h: Vec<_> = (0..n).map(|i| m2.f(0, i)).collect();
    let m3 = MatrixMagma::by_fn(n, |x, y| (x + h[(y+n-x)%n])%n);

    assert!(m3 == m2);
    assert!(m3.canonicalize2() == m.canonicalize2());
    Some(h)
}
