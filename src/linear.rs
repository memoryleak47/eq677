use crate::*;

/*
  x*y = (ax + by)%p

  x = y*(x*((y*x)*y))
    = y*(x*((ay + bx)*y))
    = y*(x*(aay + bax + by))
    = y*(ax + baay + bbax + bby)
    = ay + bax + bbaay + bbbax + bbby
  => 0 = a + bbaa + bbb
  => 1 = ba + bbba
*/

fn is_prime(p: usize) -> bool {
    if p < 2 { return false }

    (2..p).all(|d| p%d != 0)
}

pub fn linsearch() {
    for p in 0..20 {
        if !is_prime(p) { continue }
        for b in 1..p {
            let b4 = (b + b*b*b)%p;
            let a = b4.pow((p-2) as _)%p; // by Fermats little theorem.
            if (a + a*a*b*b + b*b*b)%p != 0 { continue }

            present_model(p, |x, y| (x*a + y*b)%p);
        }
    }
}
