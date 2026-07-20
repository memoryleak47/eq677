use crate::c_dpll::*;

use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub enum BranchTree {
    Heuristic,
    Branch(/*x*/ E, /*y*/ E, BTreeMap</*z*/ E, BranchTree>),
}

// This excludes "paradox" leaves.
fn iter_leaves<'a>(ctxt: &Ctxt, tree: &'a mut BranchTree) -> Vec<(Ctxt, &'a mut BranchTree)> {
    match tree {
        BranchTree::Heuristic => vec![(ctxt.clone(), tree)],
        BranchTree::Branch(x, y, map) => {
            let mut out = Vec::new();
            for (z, subtree) in map.iter_mut() {
                let mut ctxt = ctxt.clone();
                if prove_triple(*x, *y, *z, &mut ctxt).is_ok() && propagate(&mut ctxt).is_ok() {
                    out.extend(iter_leaves(&ctxt, subtree));
                }
            }
            out
        },
    }
}

// tries to find a good branching tree for this scenario.
pub fn tree_search(ctxt: &Ctxt) -> BranchTree {
    let mut trees = vec![BranchTree::Heuristic];

    for _ in 0..2 {
        for mut tree in std::mem::take(&mut trees) {
            trees.push(tree.clone());

            let Some((i, (ct, _))) = iter_leaves(ctxt, &mut tree).into_iter().enumerate().max_by_key(|(_, (ct, _))| run_ctxt(&mut ct.clone())) else { continue };
            let ct = ct.clone();

            let count = (ct.nonfresh+1).min(ct.n);
            for x in 0..count {
                for y in 0..count {
                    let new_nonfresh = ctxt.nonfresh + (x == ctxt.nonfresh || y == ctxt.nonfresh) as E;
                    let count2 = new_nonfresh.min(ct.n);
                    let mut map = BTreeMap::new();
                    for z in 0..count2 {
                        map.insert(z, BranchTree::Heuristic);
                    }
                    let mut tree2 = tree.clone();
                    let (_, leaf) = &mut iter_leaves(ctxt, &mut tree2)[i];
                    **leaf = BranchTree::Branch(x, y, map);

                    trees.push(tree2);
                }
            }
        }
    }

    trees.into_iter()
         .min_by_key(|tree| combined_cost(ctxt, tree))
         .unwrap()
}

fn combined_cost(ctxt: &Ctxt, tree: &BranchTree) -> usize {
    let mut out = 0;
    for (mut ct, _) in iter_leaves(ctxt, &mut tree.clone()) {
        out += run_ctxt(&mut ct);
    }
    out
}

fn branch_on(x: E, y: E, mut ctxt: Ctxt) -> Vec<Ctxt> {
    let mut out = Vec::new();
    for z in 0..(ctxt.nonfresh+1).min(ctxt.n) {
        let mut ctxt = ctxt.clone();
        if z == ctxt.nonfresh { ctxt.nonfresh += 1; }
        if prove_triple(x, y, z, &mut ctxt).is_ok() && propagate(&mut ctxt).is_ok() {
            out.push(ctxt);
        }
    }
    out
}

pub fn split_models(ctxt: Ctxt) -> Vec<Ctxt> {
    if !ctxt.forced_automs.is_empty() { return vec![ctxt] }
    if ctxt.n <= 4 { return vec![ctxt] }

    let mut out = Vec::new();

    {
        let mut ctxt = ctxt.clone();
        ctxt.nonfresh = 2;
        assert!(prove_triple(0, 0, 1, &mut ctxt).is_ok());
        assert!(propagate(&mut ctxt).is_ok());
    }

    {
        let mut ctxt = ctxt.clone();
        for i in 0..ctxt.n {
            assert!(prove_triple(i, i, i, &mut ctxt).is_ok());
        }
        assert!(propagate(&mut ctxt).is_ok());
        out.push(ctxt);
    }

    out
}

pub fn split_models_via_cycle_end(ctxt: Ctxt) -> Vec<Ctxt> {
    let n = ctxt.n;

    let mut cs = vec![ctxt];
    // For each x, we choose the y, s.t. x*y = x.
    for x in 0..n {
        let t = std::mem::take(&mut cs);
        for mut ctxt in t {
            // mark x as nonfresh.
            ctxt.nonfresh = ctxt.nonfresh.max(x+1);

            let count = n.min(ctxt.nonfresh+1);
            for y in 0..count {
                let mut ctxt = ctxt.clone();
                if prove_triple(x, y, x, &mut ctxt).is_err() { continue }
                cs.push(ctxt);
            }
        }
    }

    cs
}

pub fn split_models_via_row(ctxt: Ctxt) -> Vec<Ctxt> {
    if ctxt.n <= 1 { return vec![ctxt] }

    let mut out = Vec::new();

    // We choose 0 to maximize |C(0,0)|, thus if 0*0 = 0, then x*x = x holds generally.
    // Further, in this case we can still assume freshness.
    {
        let mut ctxt = ctxt.clone();
        for i in 0..ctxt.n {
            assert!(prove_triple(i, i, i, &mut ctxt).is_ok());
        }
        assert!(propagate(&mut ctxt).is_ok());
        out.push(ctxt);
    }

    // zero_orbit_size = 1 is covered by the case above;
    // zero_orbit_size in [2, 3] can't be, as no self-producing cycles are of length 2 or 3.
    // See atp directory.
    for zero_orbit_size in 4..=ctxt.n {
        let mut ctxt = ctxt.clone();

        ctxt.nonfresh = ctxt.n;
        for i in 0..zero_orbit_size {
            assert!(prove_triple(0, i, (i+1)%zero_orbit_size, &mut ctxt).is_ok());
        }
        split_rest(1, zero_orbit_size, ctxt, &mut out);
    }

    out
}


// We can not build smaller cycles than last_size. last_size is monotonically growing. But not strictly so.
// next_idx is the next E that is undefined so far.
fn split_rest(last_size: E, next_idx: E, ctxt: Ctxt, out: &mut Vec<Ctxt>) {
    assert!(next_idx <= ctxt.n);

    if next_idx == ctxt.n {
        out.push(ctxt);
        return
    }

    let remaining = ctxt.n - next_idx;
    'outer: for next_cycle in last_size..=remaining {
        let mut ctxt = ctxt.clone();
        for i in 0..next_cycle {
            let ii = next_idx + i;
            let ii_1 = next_idx + (i+1)%next_cycle;
            if prove_triple(0, ii, ii_1, &mut ctxt).is_err() { continue 'outer; }
        }
        split_rest(next_cycle, next_idx + next_cycle, ctxt, out)
    }
}
