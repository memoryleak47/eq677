use crate::*;

// Unclear so far: Is this stronger than linsearch, or subsumed by it?

/*
  x*y = (ax + by + c)%p

  x = y*(x*((y*x)*y))
    = y*(x*((ay + bx + c)*y))
    = y*(x*(aay + bax + ac + by + c))
    = y*(ax + baay + bbax + bac + bby + bc + c)
    = ay + bax + bbaay + bbbax + bbac + bbby + bbc + bc + c

  => 0 = (a + bbaa + bbb)y + (bba + bb + b + 1)c
    => 0 = (bba + bb + b + 1)c
    => 0 = a + bbaa + bbb
  => 1 = bbba + ba
*/

pub fn affinesearch() {
    for p in 0..30 {
        for a in 0..p {
            for b in 0..p {
                let b4 = (b*b*b + b)%p;
                for c in 0..p {
                    if (a*b4)%p != 1 { continue }
                    if (a + a*a*b*b + b*b*b)%p != 0 { continue }
                    if ((b*b*a + b*b + b + 1)*c)%p != 0 { continue }

                    println!("p={p}");
                    present_model(p, |x, y| (x*a + y*b + c)%p);
                }
            }
        }
    }
}
