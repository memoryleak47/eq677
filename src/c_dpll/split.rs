use crate::c_dpll::*;

use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug)]
pub enum BranchTree {
    Heuristic,
    Branch(/*x*/ E, /*y*/ E, BTreeMap</*z*/ E, BranchTree>),
}

pub fn bmap<const N: usize>(x: E, y: E, list: [(E, BranchTree); N]) -> BranchTree {
    let map = list.into_iter().collect();
    BranchTree::Branch(x, y, map)
}

pub fn heur() -> BranchTree { BranchTree::Heuristic }

pub fn draw(tree: &BranchTree) -> String {
    match tree {
        BranchTree::Heuristic => String::from("heur()"),
        BranchTree::Branch(x, y, map) => {
            let mut mapelems = Vec::new();
            for (z, mm) in map {
                mapelems.push(format!("({z}, {})", draw(mm)));
            }
            let mapstr = mapelems.join(", ");
            format!("bmap({x}, {y}, [{mapstr}])")
        },
    }
}

// This excludes "paradox" leaves.
pub fn iter_leaves<'a>(ctxt: &Ctxt, tree: &'a mut BranchTree) -> Vec<(Ctxt, &'a mut BranchTree)> {
    assert_eq!(ctxt.cost_counter, 0);

    match tree {
        BranchTree::Heuristic => vec![(ctxt.clone(), tree)],
        BranchTree::Branch(x, y, map) => {
            let count = (ctxt.nonfresh+1).min(ctxt.n);
            let set1: BTreeSet<_> = (0..count).collect();
            let set2: BTreeSet<_> = map.keys().copied().collect();
            assert_eq!(set1, set2);

            let mut out = Vec::new();

            for (z, subtree) in map.iter_mut() {
                let mut ctxt = ctxt.clone();
                defresh(*x, &mut ctxt);
                defresh(*y, &mut ctxt);
                defresh(*z, &mut ctxt);
                if prove_triple(*x, *y, *z, &mut ctxt).is_ok() && propagate(&mut ctxt).is_ok() {
                    out.extend(iter_leaves(&ctxt, subtree));
                }
            }
            out
        },
    }
}

fn grow(ctxt: &Ctxt, tree: &BranchTree) -> Vec<BranchTree> {
    assert_eq!(ctxt.cost_counter, 0);

    let Some((i, (child_ctxt, _))) = iter_leaves(ctxt, &mut tree.clone()).into_iter().enumerate().max_by_key(|(_, (ct, _))| run_ctxt(&mut ct.clone())) else { return Vec::new() };

    let mut out = Vec::new();

    let count = (child_ctxt.nonfresh+1).min(ctxt.n);
    for x in 0..count {
        for y in 0..count {
            let mut child_ctxt = child_ctxt.clone();
            defresh(x, &mut child_ctxt);
            defresh(y, &mut child_ctxt);

            let count2 = (child_ctxt.nonfresh+1).min(ctxt.n);
            let map = (0..count2).map(|a| (a, BranchTree::Heuristic)).collect();

            let mut tree2 = tree.clone();
            let (_, leaf) = &mut iter_leaves(ctxt, &mut tree2)[i];
            **leaf = BranchTree::Branch(x, y, map);

            out.push(tree2);
        }
    }

    out
}

// tries to find a good branching tree for this scenario.
pub fn tree_search(ctxt: &Ctxt) -> BranchTree {
    assert_eq!(ctxt.cost_counter, 0);

    let mut trees = vec![BranchTree::Heuristic];

    for _ in 0..2 {
        for mut tree in std::mem::take(&mut trees) {
            trees.push(tree.clone());
            trees.extend(grow(ctxt, &tree));
        }
    }

    trees.into_iter()
         .min_by_key(|tree| combined_cost(ctxt, tree))
         .unwrap()
}

pub fn combined_cost(ctxt: &Ctxt, tree: &BranchTree) -> usize {
    assert_eq!(ctxt.cost_counter, 0);

    let mut out = 0;
    for (mut ct, _) in iter_leaves(ctxt, &mut tree.clone()) {
        out += run_ctxt(&mut ct);
    }
    out
}

pub fn split_models(ctxt: Ctxt) -> Vec<Ctxt> {
    assert_eq!(ctxt.cost_counter, 0);

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
