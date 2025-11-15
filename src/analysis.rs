use crate::*;

use egg::*;

type EGraph = egg::EGraph<MagmaLang, ()>;

define_language! {
    enum MagmaLang {
        "f" = F([Id; 2]),
        E(usize),
        "X" = X,
        "Y" = Y,
    }
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
    for c in eg.classes() {
        if !c.nodes.contains(&MagmaLang::X) { continue }
        println!("{}", ex.find_best(c.id).1);
        let mut terms = Vec::new();
        for n in &c.nodes {
            let e = n.join_recexprs(|x| ex.find_best(x).1);
            terms.push(e);
        }
        terms.sort_by_cached_key(|x| MySize.cost_rec(x));
        for e in terms {
            println!("= {e}");
        }
        println!();
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
