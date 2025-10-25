use crate::*;

pub type E = u8;
pub type V = usize;

pub enum Term {
    Var(V),
    Elem(E),
    F(Box<[Term; 2]>),
}

// returns whether we can prove s > t:
fn kbo(s: &Term, t: &Term) -> bool {
    todo!()
}

fn weight(t: &Term) -> u32 {
    match t {
        Term::Var(_) => 1,
        Term::Elem(_) => 1,
        Term::F(b) => weight(&b[0]) + weight(&b[1]) + 1,
    }
}
