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

pub fn combined_cost(ctxt: &Ctxt, tree: &BranchTree) -> usize {
    let mut out = 0;
    for (mut ct, _) in iter_leaves(ctxt, &mut tree.clone()) {
        out += run_ctxt(&mut ct);
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
        out.push(ctxt);
    }

    out
}
