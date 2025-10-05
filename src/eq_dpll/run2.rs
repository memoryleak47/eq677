use crate::eq_dpll::*;

const GRID_SIZE: usize = 4;

pub fn eq_run2(n: usize) {
    let mut candidates = vec![build_ctxt(n)];
    // candidates[0].fresh.iter_mut().for_each(|x| { *x = false; });
    let mut out_candidates = Vec::new();

    while candidates.len() > 0 {
        for mut candidate in std::mem::take(&mut candidates) {
            let Some((pos, options)) = next_options2(&mut candidate) else {
                if get_partial_magma(&candidate).is_total() {
                    print_model(&candidate);
                } else {
                    out_candidates.push(candidate);
                }
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

    candidates = out_candidates;
    let old_size = candidates.len();

    use std::collections::HashSet;

    let mut seen = HashSet::new();
    for mut candidate in std::mem::take(&mut candidates) {
        if revisit(&mut candidate).is_err() { continue }

        let m = get_partial_magma(&candidate).canonicalize();
        if seen.insert(m) {
            candidates.push(candidate);
        }
    }
    println!("-- deduplicated from {} to {}", old_size, candidates.len());

    into_par_for_each(candidates, |mut ctxt| {
        ctxt.depth = 200;
        mainloop(ctxt);
    });
}

pub fn get_partial_magma(ctxt: &Ctxt) -> MatrixMagma {
    MatrixMagma::by_fn(ctxt.n, |x, y| ctxt.table[idx((x,y), ctxt.n)])
}

// TODO improve choice heuristic.
fn next_options2(ctxt: &mut Ctxt) -> Option<(PosId, Vec<ElemId>)> {
    let pos = best_score2(ctxt)?;

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

fn best_score2(ctxt: &Ctxt) -> Option<PosId> {
    let mut best = None;
    // NOTE: flipping these loops doubles the runtime. So this is crucially important.
    for x in 0..ctxt.n {
        for y in 0..ctxt.n {
            if x >= GRID_SIZE || y >= GRID_SIZE { continue; }

            let i = idx((x, y), ctxt.n);
            if ctxt.table[i] != ElemId::MAX { continue; }
            let score = ctxt.pos_terms[i].len();
            let cond = match best {
                None => true,
                Some((_, score2)) => score > score2,
            };
            if cond {
                best = Some(((x, y), score));
            }
        }
    }
    Some(best?.0)
}

pub fn revisit(ctxt: &mut Ctxt) -> Res{
    for (i, c) in ctxt.classes.iter().enumerate() {
        if c.value.is_some() {
            for p in c.parents.iter() {
                ctxt.propagate_queue.push(PropagationTask::VisitParent(*p));
            }
        }
    }
    propagate_loop(ctxt)
}
