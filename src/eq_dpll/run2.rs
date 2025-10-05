use crate::eq_dpll::*;

pub fn eq_run2(n: usize) {
    let mut candidates = vec![build_ctxt(n)];

    for _ in 0..10 {
        for mut candidate in std::mem::take(&mut candidates) {
            let Some((pos, options)) = next_options2(&mut candidate) else {
                print_model(&candidate);
                continue;
            };

            for o in options {
                let mut ctxt = candidate.clone();
                activate_option(pos, vec![o], &mut ctxt);
                if ctxt.mode != Mode::Backtracking {
                    candidates.push(ctxt);
                }
            }
        }
    }

    println!("deduplication! from {} ...", candidates.len());
    use std::collections::HashSet;

    let mut seen = HashSet::new();
    for candidate in std::mem::take(&mut candidates) {
        let mut m = get_partial_magma(&candidate);
        m.canonicalize();
        if seen.insert(m) {
            candidates.push(candidate);
        }
    }
    println!("... to {}", candidates.len());

    into_par_for_each(candidates, |mut ctxt| {
        ctxt.depth = 200;
        mainloop(ctxt);
    });
}

fn get_partial_magma(ctxt: &Ctxt) -> MatrixMagma {
    MatrixMagma::by_fn(ctxt.n, |x, y| ctxt.table[idx((x,y), ctxt.n)])
}

// TODO improve choice heuristic.
fn next_options2(ctxt: &mut Ctxt) -> Option<(PosId, Vec<ElemId>)> {
    let pos = best_score(ctxt)?;

    let mut found_fresh = false;

    if ctxt.fresh[pos.0] {
        ctxt.fresh[pos.0] = false;
        ctxt.trail.push(TrailEvent::Defresh(pos.0));
    }
    if ctxt.fresh[pos.1] {
        ctxt.fresh[pos.1] = false;
        ctxt.trail.push(TrailEvent::Defresh(pos.1));
    }

    let mut valids = Vec::new();
    for e in 0..ctxt.n {
        if ctxt.fresh[e] {
            // If we already used a "fresh" ElemId, no reason to do the same operation for another fresh one!
            if found_fresh { continue }
            else { found_fresh = true; }
        }

        if infeasible_decision(pos, e, ctxt) { continue }

        valids.push(e);
    }

    Some((pos, valids))
}

