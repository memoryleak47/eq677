use crate::*;
use nauty_pet::prelude::*;
use nauty_pet::canon::*;
use nauty_pet::autom::{AutomStats, TryIntoAutomStatsTraces, AutomGroup, TryIntoAutomGroupNautyDense};
use petgraph::visit::EdgeRef;
use std::collections::{HashMap, HashSet};

type Graph = petgraph::graph::UnGraph<NodeType, EdgeType>;
type Group = Vec<Perm>;
type Perm = Vec<usize>;

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
enum NodeType {
    Elem, XYZ
}

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
enum EdgeType {
    X, Y, Z
}

impl MatrixMagma {
    pub fn canonicalize2(&self) -> MatrixMagma {
        let g = graphify(self);
        let g = g.into_canon_traces();
        let m = de_graphify(&g);
        m
    }

    pub fn autom_stats(&self) -> AutomStats {
        graphify(self).try_into_autom_stats_traces().unwrap()
    }

    pub fn autom_group(&self) -> Group {
        let mut a = graphify(self).try_into_autom_group_nauty_dense().unwrap().0;
        for x in &mut a {
            x.truncate(self.n);
        }
        a
    }

    pub fn autom_group_mini(&self) -> Group {
        minimize(self.autom_group())
    }
}

// produces perm^k
fn compose_rep(perm: &[usize], k: usize) -> Perm {
    let n = perm.len();
    (0..n).map(|mut x| {
        for _ in 0..k {
            x = perm[x];
        }
        x
    }).collect()
}

pub fn minimize(mut group: Group) -> Group {
    let n = group[0].len();
    let mut set: HashSet<Perm> = group.iter().cloned().collect();
    for a in group.iter() {
        if set.contains(a) {
            for k in 2..n {
                let rep = compose_rep(&a, k);
                if &rep == a { break }

                set.remove(&rep);
            }
        }
    }
    set.into_iter().collect()
}

pub fn orbits(autom: &[Vec<usize>]) -> Vec<usize> {
    let n = autom[0].len();
    let mut orbits: Vec<usize> = (0..n).collect();
    for aut in autom {
        for i in 0..n {
            let j = aut[i];
            if j < orbits[i] {
                orbits[i] = j;
            }
        }
    }
    orbits
}

fn graphify(m: &MatrixMagma) -> Graph {
    let mut g = Graph::new_undirected();
    let mut nodes = Vec::new();
    for x in 0..m.n {
        nodes.push(g.add_node(NodeType::Elem));
    }
    for x in 0..m.n {
        for y in 0..m.n {
            let z = m.f(x, y);
            let xyz = g.add_node(NodeType::XYZ);
            g.add_edge(xyz, nodes[x], EdgeType::X);
            g.add_edge(xyz, nodes[y], EdgeType::Y);
            g.add_edge(xyz, nodes[z], EdgeType::Z);
        }
    }
    g
}

fn de_graphify(g: &Graph) -> MatrixMagma {
    let mut nodes = HashMap::new();
    for idx in g.node_indices() {
        if g[idx] == NodeType::Elem {
            nodes.insert(idx, nodes.len());
        }
    }

    let mut m = MatrixMagma::zeros(nodes.len());

    for idx in g.node_indices() {
        if g[idx] == NodeType::XYZ {
            let mut x = None;
            let mut y = None;
            let mut z = None;
            for e in g.edges(idx) {
                match e.weight() {
                    EdgeType::X => { x = Some(e.target()); }
                    EdgeType::Y => { y = Some(e.target()); }
                    EdgeType::Z => { z = Some(e.target()); }
                }
            }
            let (x, y, z) = (x.unwrap(), y.unwrap(), z.unwrap());
            m.set_f(nodes[&x], nodes[&y], nodes[&z]);
        }
    }
   
    m
}
