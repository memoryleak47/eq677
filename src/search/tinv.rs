use crate::*;

/*
    Translation-invariant models of the form:
    f(x, y) = x + h(y - x)
*/


fn all_perms(n: usize) -> Vec<Vec<usize>> {
    if n == 0 { return vec![Vec::new()]; }
    let mut outs = Vec::new();

    // we decide 'out[0] = a'.
    for p in all_perms(n-1) {
        for a in 0..n {
            let mut out = Vec::new();
            out.push(a);
            out.extend(p.iter().copied().map(|x| x + (x >= a) as usize));
            outs.push(out);
        }
    }
    outs
}

pub fn tinv_search() {
    for p in 0.. {
        for h in all_perms(p) {
            let f_def = |x, y| (x + h[(y + p - x)%p])%p;
            let f = FnMagma {
                n: p,
                f_def,
            };
            if f.is677() {
                present_model(p, "tinv", f_def);
            }
        }
    }
}
