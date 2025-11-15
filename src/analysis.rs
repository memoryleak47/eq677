use crate::*;

use egg::*;

type EGraph = egg::EGraph<MagmaLang, ()>;
type RecExpr = egg::RecExpr<MagmaLang>;

const THRESHOLD: usize = 17;

define_language! {
    enum MagmaLang {
        "f" = F([Id; 2]),
        E(usize),
        "X" = X,
        "Y" = Y,
    }
}

fn flip_re(t: &mut RecExpr) {
    for i in 0..t.len() {
        let i = i.into();

        match t[i] {
            MagmaLang::X => t[i] = MagmaLang::Y,
            MagmaLang::Y => t[i] = MagmaLang::X,
            _ => {},
        }
    }
}

fn normalize_equation(lhs: &RecExpr, rhs: &RecExpr) -> (RecExpr, RecExpr) {
    let mut lhs = lhs.clone();
    let mut rhs = rhs.clone();

    let s = format!("{lhs} {rhs}");
    let should_flip = match (s.find("X"), s.find("Y")) {
        (Some(i), Some(j)) if i > j => true,
        (_, Some(_)) => true,
        _ => false,
    };
    if should_flip {
        flip_re(&mut lhs);
        flip_re(&mut rhs);
    }
    (lhs, rhs)
}

pub fn analyze(m: &MatrixMagma) {
    if m.n < 2 { return }

    let mut eg = eggify(m);
    let mut out = None;

    let xv = eg.add(MagmaLang::X);
    let yv = eg.add(MagmaLang::Y);

    for x in 0..m.n {
        for y in 0..m.n {
            let mut eg = eg.clone();
            let xl = eg.lookup(MagmaLang::E(x)).unwrap();
            let yl = eg.lookup(MagmaLang::E(y)).unwrap();
            eg.union(xv, xl);
            eg.union(yv, yl);
            match out {
                None => out = Some(eg),
                Some(o) => out = Some(eg.egraph_intersect(&o, ())),
            }
        }
    }

    let eg = out.unwrap();
    let ex = Extractor::new(&eg, MySize);

    let mut equations: Vec<(RecExpr, RecExpr)> = Vec::new();
    for c in eg.classes() {
        let lhs = ex.find_best(c.id).1;
        for n in &c.nodes {
            let rhs = n.join_recexprs(|x| ex.find_best(x).1);
            if lhs.to_string() == rhs.to_string() { continue }
            let (lhs, rhs) = normalize_equation(&lhs, &rhs);
            equations.push((lhs, rhs));
        }
    }
    equations.sort_by_cached_key(|(i, j)| MySize.cost_rec(i) + MySize.cost_rec(j));
    equations.dedup();
    for (lhs, rhs) in equations {
        if MySize.cost_rec(&lhs) + MySize.cost_rec(&rhs) > THRESHOLD { break }
        println!("{lhs} = {rhs}");
    }
}

fn eggify(m: &MatrixMagma) -> EGraph {
    let mut eg = EGraph::new(());
    for x in 0..m.n {
        eg.add(MagmaLang::E(x));
    }
    for x in 0..m.n {
        for y in 0..m.n {
            let x_ = eg.lookup(MagmaLang::E(x)).unwrap();
            let y_ = eg.lookup(MagmaLang::E(y)).unwrap();
            let fxy_ = eg.add(MagmaLang::F([x_, y_]));
            let z_ = eg.lookup(MagmaLang::E(m.f(x, y))).unwrap();
            eg.union(z_, fxy_);
        }
    }
    eg.rebuild();
    assert_eq!(eg.classes().len(), m.n);

    eg
}

struct MySize;

impl CostFunction<MagmaLang> for MySize {
    type Cost = usize;

    fn cost<C>(&mut self, enode: &MagmaLang, mut costs: C) -> usize where C: FnMut(Id) -> usize {
        match *enode {
            MagmaLang::F([a, b]) => costs(a) + costs(b) + 1,
            MagmaLang::E(_) => 5,
            MagmaLang::X => 1,
            MagmaLang::Y => 1,
        }
    }
}
