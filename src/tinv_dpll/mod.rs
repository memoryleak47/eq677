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
        if ctxt.h_inv[v as usize] != E::MAX { continue }

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

    propagate(ctxt)
}

fn propagate(ctxt: &mut Ctxt) -> Result<(), ()> {
    // 0 = f(y, f(0, f(f(y, 0), y)))
    for y in 0..ctxt.n {
        let a = f(y, 0, ctxt); if a == E::MAX { continue }
        let a = f(a, y, ctxt); if a == E::MAX { continue }
        let a = f(0, a, ctxt); if a == E::MAX { continue }
        let b = f(y, a, ctxt); if b == E::MAX {
            // f(y, a) == 0
            // <-> y + h(a-y) == 0
            // <-> h(a-y) == -y
            let n = ctxt.n;
            return set((a+n-y)%n, (n-y)%n, ctxt);
        }
        if b != 0 { return Err(()) }
    }

    // 0 = f(f(y, 0), f(f(y, f(y, 0)), y))
    // 0 = f(a, f(f(y, a), y))
    // 0 = f(a, f(b, y))
    // 0 = f(a, b)

    for y in 0..ctxt.n {
        let a = f(y, 0, ctxt); if a == E::MAX { continue }
        let b = f(y, a, ctxt); if b == E::MAX { continue }
        let b = f(b, y, ctxt); if b == E::MAX { continue }
        let c = f(a, b, ctxt); if c == E::MAX {
            // f(a, b) == 0
            // <-> a + h(b-a) == 0
            // <-> h(b-a) == -a
            let n = ctxt.n;
            return set((b+n-a)%n, (n-a)%n, ctxt);
        }
        if c != 0 { return Err(()) }
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
