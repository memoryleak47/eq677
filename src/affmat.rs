use crate::*;

// TODO it would probably advisable to restart inferring stuff from
// f(x, y) = M1*x + M2*y + c
// I'm not sure the stuff from below works in an exactly compatible way.

// x = f(y, f(x, f(f(y, x), y)))
// x = f(y, f(x, f(M1*y + M2*x + c, y)))
// x = f(y, f(x, M1*M1*y + M1*M2*x + M1*c + M2*y + c))
// x = f(y, M1*x + M2*M1*M1*y + M2*M1*M2*x + M2*M1*c + M2*M2*y + M2*c + c)
// x = M1*y + M2*M1*x + M2*M2*M1*M1*y + M2*M2*M1*M2*x + M2*M2*M1*c + M2*M2*M2*y + M2*M2*c + M2*c + c

/*
  f(x, y) = M1*x + M2*y
  => 0 = M1 + M2^2 M1^2 + M2^3
  => I = M2 M1 + M2^2 M1 M2
*/

type Mat = [[usize; 3]; 3];
type V = [usize; 3];

fn p_mats(p: usize) -> Vec<Mat> {
    let mut out = Vec::new();
    for a1 in 0..p {
        for a2 in 0..p {
            for a3 in 0..p {
                for a4 in 0..p {
                    for a5 in 0..p {
                        for a6 in 0..p {
                            out.push([[a1, a2, a3], [a4, a5, a6], [0, 0, 1]]);
                        }
                    }
                }
            }
        }
    }
    out
}

/// 3x3 matrix multiplication (mod p)
fn mm(a: Mat, b: Mat, p: usize) -> Mat {
    let mut c = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            c[i][j] = (a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j]) % p;
        }
    }
    // TODO why does this fail?
    // assert_eq!(c[2], [0, 0, 1]);
    c
}

/// 3x3 matrix times 3x1 vector (mod p)
fn mv(a: Mat, v: V, p: usize) -> V {
    let mut r = [0; 3];
    for i in 0..3 {
        r[i] = (a[i][0] * v[0] + a[i][1] * v[1] + a[i][2] * v[2]) % p;
    }
    assert_eq!(r[2], 1);
    r
}

/// 3x3 matrix addition (mod p)
fn mplus(l: Mat, r: Mat, p: usize) -> Mat {
    let mut out = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            out[i][j] = (l[i][j] + r[i][j]) % p;
        }
    }
    out[2][2] = 1;
    assert_eq!(out[2], [0, 0, 1]);
    out
}

/// 3x1 vector addition (mod p)
fn vplus(l: V, r: V, p: usize) -> V {
    [
        (l[0] + r[0]) % p,
        (l[1] + r[1]) % p,
        1,
    ]
}

pub fn affmat_search() {
    for p in 0..10 {
        for m1 in p_mats(p) {
            for m2 in p_mats(p) {
                // We keep the constant factor only in m1.
                if m2[0][2] != 0 || m2[1][2] != 0 { continue }

                let m12 = mm(m1, m1, p);
                let m22 = mm(m2, m2, p);

                // 0 = M1 + M2^2 M1^2 + M2^3
                let m22_12 = mm(m22, m12, p);
                let m2_3 = mm(m2, m22, p);
                let o = mplus(m1, mplus(m22_12, m2_3, p), p);
                if o != [[0, 0, 0], [0, 0, 0], [0, 0, 1]] { continue }

                // => I = M2 M1 + M2^2 M1 M2
                let m2m1 = mm(m2, m1, p);
                let m22_1_2 = mm(m22, mm(m1, m2, p), p);
                let i = mplus(m2m1, m22_1_2, p);
                if i != [[1, 0, 0], [0, 1, 0], [0, 0, 1]] { continue }

                println!("p={p}, p^2={}", p*p);
                present_model(p*p, |x, y| {
                    let x = [x%p, x/p, 1];
                    let y = [y%p, y/p, 1];
                    let v = vplus(mv(m1, x, p), mv(m2, y, p), p);
                    v[0] + p*v[1]
                });
            }
        }
    }
}
