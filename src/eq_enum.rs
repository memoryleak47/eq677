use crate::*;

type Map<K, V> = indexmap::IndexMap<K, V>;

type ElemIdx = usize;
type PosIdx = (usize, usize);

#[derive(Clone)]
struct Constraint(ElemIdx, Term);

#[derive(Clone, PartialEq, Eq)]
enum Term {
    Elem(ElemIdx),
    F(Box<[Term; 2]>),
}

type Table = Map<PosIdx, ElemIdx>;

#[derive(Clone)]
struct Ctxt {
    constraints: Vec<Constraint>,
    table: Table,
    n: usize,
    fresh: Vec<bool>, // whether an ElemIdx is still "fresh".
}

pub fn eq_run(n: usize) {
    step(Ctxt {
        constraints: build_constraints(n),
        table: Map::new(),
        n,
        fresh: vec![true; n],
    });
}


fn step(ctxt: Ctxt) {
    let all_pos = (0..ctxt.n).map(|x| (0..ctxt.n).map(move |y| (x, y))).flatten();
    let free_pos = all_pos.filter(|xy| ctxt.table.get(xy).is_none());
    let Some(pos) = free_pos.max_by_key(|pos| score(*pos, &ctxt)) else {
        let magma = MatrixMagma::by_fn(ctxt.n, |x, y| *ctxt.table.get(&(x, y)).unwrap());
        println!("Model found:");
        magma.dump();

        assert!(magma.is677());
        assert!(magma.is255());

        return; // We are done!
    };
    let mut found_fresh = false;
    for e in 0..ctxt.n {
        if ctxt.fresh[e] {
            // If we already used a "fresh" ElemIdx, no reason to do the same operation for another fresh one!
            if found_fresh { continue }
            else { found_fresh = true; }
        }

        if (0..ctxt.n).any(|z| ctxt.table.get(&(pos.0, z)) == Some(&e)) { continue }

        let mut c = ctxt.clone();
        c.table.insert(pos, e);

        c.fresh[pos.0] = false;
        c.fresh[pos.1] = false;
        c.fresh[e] = false;

        if simplify(&mut c).is_none() {
            step(c);
        }
    }
}

struct Failure;

fn simplify(ctxt: &mut Ctxt) -> Option<Failure> {
    'outer: loop {
        for Constraint(l, t) in ctxt.constraints.iter_mut() {
            simplify_term(t, &ctxt.table);
            match t {
                Term::F(xy) => if let [Term::Elem(x), Term::Elem(y)] = &**xy {
                    ctxt.table.insert((*x, *y), *l);
                    continue 'outer;
                },
                Term::Elem(r) => if l != r {
                    return Some(Failure)
                },
            }
        }
        return None;
    }
}

fn simplify_term(t: &mut Term, tab: &Table) {
    match t {
        Term::Elem(_) => {},
        Term::F(ab) => {
            let [a, b] = &mut **ab;
            simplify_term(a, tab);
            simplify_term(b, tab);
            if let [Term::Elem(a), Term::Elem(b)] = [a, b] && let Some(new) = tab.get(&(*a, *b)) {
                *t = Term::Elem(*new);
            }
        },
    }
}

fn score(pos: PosIdx, ctxt: &Ctxt) -> usize {
    let mut s = 0;
    for Constraint(_, t) in &ctxt.constraints {
        let f = match termsize(t) {
            0 => 0,
            2 => 100,
            3 => 30,
            4 => 6,
            5 => 3,
            x => panic!("how? {x}"),
        };
        s += count_pos(pos, t) * f;
    }
    s
}

fn count_pos(pos: PosIdx, term: &Term) -> usize {
    match term {
        Term::Elem(_) => 0,
        Term::F(ab) => {
            if ab[0] == Term::Elem(pos.0) && ab[1] == Term::Elem(pos.1) { 1 }
            else { count_pos(pos, &ab[0]) + count_pos(pos, &ab[1]) }
        }
    }
}

// counts the "F" terms.
fn termsize(term: &Term) -> usize {
    match term {
        Term::Elem(_) => 0,
        Term::F(ab) => termsize(&ab[0]) + termsize(&ab[1]) + 1,
    }
}

fn build_constraints(n: usize) -> Vec<Constraint> {
    let mut constraints = Vec::new();
    let f = |a: &Term, b: &Term| Term::F(Box::new([a.clone(), b.clone()]));
    for x in 0..n {
        for y in 0..n {
            let t = {
                let x = &Term::Elem(x);
                let y = &Term::Elem(y);

                f(y, &f(x, &f(&f(y, x), y)))
            };
            constraints.push(Constraint(x, t));
        }
    }
    constraints
}
