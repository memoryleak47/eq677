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

fn modpow(mut b: usize, mut e: usize, p: usize) -> usize {
    let mut res = 1;
    while (e > 0) {
        if (e & 1 == 1) {
            res = res*b % p;
        }
        b = b * b % p;
        e >>= 1;
    }
    res
}

pub fn linsearch() {
    for p in 0..30 {
        if !is_prime(p) { continue }
        for b in 1..p {
            let b4 = (b + b*b*b)%p;
            let a = modpow(b4, p-2, p); // by Fermats little theorem.
            if (a + a*a*b*b + b*b*b)%p != 0 { continue }

            println!("p={p}");
            present_model(p, |x, y| (x*a + y*b)%p);
        }
    }
}
