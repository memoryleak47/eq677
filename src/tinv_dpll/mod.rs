use crate::*;

type E = u8;

#[derive(Clone)]
struct Ctxt {
    n: E,
    h: Box<[E]>, // E::MAX means unknown.
    h_inv: Box<[E]>, // E::MAX means unknown.
}

fn f(x: E, y: E, ctxt: &Ctxt) -> E {
    assert!(x < ctxt.n);
    assert!(y < ctxt.n);

    let n = ctxt.n;
    let i = (n+y-x)%n;
    let h = ctxt.h[i as usize];
    if h == E::MAX { return E::MAX }
    (x + h)%n
}

pub fn tinv_run(n: usize) {
    let mut ctxt = build_ctxt(n);
    run(&mut ctxt);
}

// returns i, s.t. h[i] will be guessed next.
fn heuristic(ctxt: &Ctxt) -> Option<E> {
    ctxt.h.iter().enumerate()
        .filter(|(_, x)| **x == E::MAX)
        .map(|(x, _)| x as E)
        .next()
}

fn run(ctxt: &mut Ctxt) {
    let Some(x) = heuristic(ctxt) else {
        submit_model(ctxt);
        return;
    };

    assert!(ctxt.h[x as usize] == E::MAX);

    for v in 0..ctxt.n {
        let ctxt = &mut ctxt.clone();
        if set(x, v, ctxt).is_ok() {
            run(ctxt);
        }
    }
}

fn submit_model(ctxt: &Ctxt) {
    present_model(ctxt.n as usize, "tinv-search", |x, y| f(x as E, y as E, ctxt) as usize);
}

// h[x] = v
fn set(x: E, v: E, ctxt: &mut Ctxt) -> Result<(), ()> {
    ctxt.h[x as usize] = v;
    ctxt.h_inv[v as usize] = x;

    check(ctxt)
}

fn check(ctxt: &Ctxt) -> Result<(), ()> {
    // 0 = f(y, f(0, f(f(y, 0), y)))
    for y in 0..ctxt.n {
        let a = f(y, 0, ctxt); if a == E::MAX { continue }
        let a = f(a, y, ctxt); if a == E::MAX { continue }
        let a = f(0, a, ctxt); if a == E::MAX { continue }
        let a = f(y, a, ctxt); if a == E::MAX { continue }
        if a != 0 { return Err(()) }
    }
    Ok(())
}

fn build_ctxt(n: usize) -> Ctxt {
    Ctxt {
        n: n as E,
        h: vec![E::MAX; n].into_boxed_slice(),
        h_inv: vec![E::MAX; n].into_boxed_slice(),
    }
}
