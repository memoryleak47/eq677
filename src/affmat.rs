use crate::*;

// f(x, y) = M1*x + M2*y + c

// x = f(y, f(x, f(f(y, x), y)))
// x = f(y, f(x, f(M1*y + M2*x + c, y)))
// x = f(y, f(x, M1*M1*y + M1*M2*x + M1*c + M2*y + c))
// x = f(y, M1*x + M2*M1*M1*y + M2*M1*M2*x + M2*M1*c + M2*M2*y + M2*c + c)
// x = M1*y + M2*M1*x + M2*M2*M1*M1*y + M2*M2*M1*M2*x + M2*M2*M1*c + M2*M2*M2*y + M2*M2*c + M2*c + c

// set x=0 and y=0:
// 0 = (M2*M2*M1 + M2*M2 + M2 + 1)*c

// set x=0 and simplify using the equation from above:
// 0 = (M1 + M2*M2*M1*M1 + M2*M2*M2)*y

// x = (M2*M1 + M2*M2*M1*M2)*x

type Mat = [[usize; 2]; 2];
type V = [usize; 2];

pub fn affmat_search() {
    for p in 0..10 {
        for m1 in p_mats(p) {
            for m2 in p_mats(p) {
                for c0 in 0..p {
                    for c1 in 0..p {
                        let m12 = mm(m1, m1, p);
                        let m22 = mm(m2, m2, p);

                        // 0 = M1 + M2^2 M1^2 + M2^3
                        let m22_12 = mm(m22, m12, p);
                        let m2_3 = mm(m2, m22, p);
                        let o = mplus(m1, mplus(m22_12, m2_3, p), p);
                        if o != [[0, 0], [0, 0]] { continue }

                        // => I = M2 M1 + M2^2 M1 M2
                        let m2m1 = mm(m2, m1, p);
                        let m22_1_2 = mm(m22, mm(m1, m2, p), p);
                        let i = mplus(m2m1, m22_1_2, p);
                        if i != [[1, 0], [0, 1]] { continue }

                        // 0 = (M2^2*M1 + M2^2 + M2 + 1)*c
                        let m22_1 = mm(m22, m1, p);
                        let id = [[1, 0], [0, 1]];
                        let a = mplus(m22_1, m22, p);
                        let a = mplus(a, m2, p);
                        let a = mplus(a, id, p);
                        if a != [[0, 0], [0, 0]] { continue }

                        println!("p={p}, p^2={}", p*p);
                        present_model(p*p, |x, y| {
                            let x = [x%p, x/p];
                            let y = [y%p, y/p];
                            let v = vplus(mv(m1, x, p), mv(m2, y, p), p);
                            let v = vplus(v, [c0, c1], p);
                            v[0] + p*v[1]
                        });

                    }
                }
            }
        }
    }
}


fn p_mats(p: usize) -> Vec<Mat> {
    let mut out = Vec::new();
    for a1 in 0..p {
        for a2 in 0..p {
            for a3 in 0..p {
                for a4 in 0..p {
                    out.push([[a1, a2], [a3, a4]]);
                }
            }
        }
    }
    out
}

fn mm(a: Mat, b: Mat, p: usize) -> Mat {
    let mut c = [[0; 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            c[i][j] = (a[i][0] * b[0][j] + a[i][1] * b[1][j])%p;
        }
    }
    c
}

fn mv(a: Mat, v: V, p: usize) -> V {
    let mut r = [0; 2];
    for i in 0..2 {
        r[i] = (a[i][0] * v[0] + a[i][1] * v[1])%p;
    }
    r
}

fn mplus([[l1, l2], [l3, l4]]: Mat, [[r1, r2], [r3, r4]]: Mat, p: usize) -> Mat {
    [[(l1 + r1)%p, (l2 + r2)%p], [(l3 + r3)%p, (l4 + r4)%p]]
}

fn vplus([l1, l2]: V, [r1, r2]: V, p: usize) -> V {
    [(l1 + r1)%p, (l2 + r2)%p]
}
