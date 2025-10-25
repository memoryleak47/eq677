use crate::*;

use std::collections::BTreeSet;

mod kbo;

pub type E = u8;
pub type V = usize;

// typically oriented lhs -> rhs.
pub type Rule = (Term, Term, /*oriented*/);

struct ER(BTreeSet<Rule>);

#[derive(PartialEq, Eq)]
pub enum Term {
    V(V),
    E(E),
    F(Box<[Term; 2]>),
}

pub fn completion(er: &mut ER) {
    loop {
        simplify(er);
        if !compute_cps(er) { break }
    }
}

// Returns false, if no new CPs were added.
fn compute_cps(er: &mut ER) -> bool {
    todo!()
}

fn simplify(er: &mut ER) {
    todo!()
}
