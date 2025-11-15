use crate::*;

use egg::*;

type EGraph = egg::EGraph<MagmaLang, ()>;

define_language! {
    enum MagmaLang {
        "f" = F([Id; 2]),
        "var" = Var(Id),
        E(usize),
    }
}

pub fn analyze(m: &MatrixMagma) {
    if m.n < 2 { return }

    let mut eg = eggify(m);
    let mut out = None;

    let xg = eg.lookup(MagmaLang::E(0)).unwrap();
    let yg = eg.lookup(MagmaLang::E(1)).unwrap();
    let xv = eg.add(MagmaLang::Var(xg));
    let yv = eg.add(MagmaLang::Var(yg));

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
    let ex = Extractor::new(&eg, AstSize);
    for c in eg.classes() {
        println!("{}", ex.find_best(c.id).1);
        for n in &c.nodes {
            let e = n.join_recexprs(|x| ex.find_best(x).1);
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
