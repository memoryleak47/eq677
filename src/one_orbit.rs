use crate::*;

#[derive(Clone)]
struct Ctxt {
    m: MatrixMagma,
    n: usize,
    nonfresh: usize,
}

pub fn one_orbit_run(n: usize) {
    let mut ctxt = Ctxt {
        m: MatrixMagma::undefined(n),
        n,
        nonfresh: 0,
    };

    run(&mut ctxt);
}

fn run(ctxt: &mut Ctxt) {
    let Some((x, y)) = heuristic(&ctxt) else {
        present_model(ctxt.n, "one_orbit", |x, y| ctxt.m.f(x, y));
        return;
    };

    for z in 0..=ctxt.nonfresh {
        let mut ctxt = ctxt.clone();
        ctxt.m.set_f(x, y, z);
        if propagate(&mut ctxt).is_ok() {
            run(&mut ctxt);
        }
    }
}

// What question to ask next?
fn heuristic(ctxt: &Ctxt) -> Option<(usize, usize)> {
    for x in 0..ctxt.n {
        for y in 0..ctxt.n {
            if ctxt.m.f(x, y) == usize::MAX {
                return Some((x, y));
            }
        }
    }
    None
}

fn propagate(ctxt: &mut Ctxt) -> Result<(), ()> {
    todo!()
}
