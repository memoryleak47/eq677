use crate::semitinv_dpll::*;

// Constraints

pub const C11_SCORE: i32 = 2000;
pub const C12_SCORE: i32 = 3000;
pub const C21_SCORE: i32 = 2000;
pub const C22_SCORE: i32 = 3000;

//       x = f(y, f(x, f(f(y, x), y)))
// (C11) x = f(y, f(x, f(a1, y)))
// (C12) x = f(y, f(x, a2))
//       x = f(y, a3)

// argument order for visit_cij and progress_c/progress_cij:
// - everything contained in the constraint variant CH in order
// - the query args (i.e. a, b of the query f(a, b)).
// - the answer to the query (only for progress_c/progress_cij)

#[derive(Clone, Copy)]
pub enum CH {
    C11(/*x*/ E), // query = f(a1, y) =: a2
    C12(/*y*/ E), // query = f(x, a2) =: a3

    C21(/*y*/ E),
    C22(/*y*/ E, /*a*/ E),
}

pub fn progress_c(c: CH, x: E, y: E, z: E, ctxt: &mut Ctxt) -> Result<(), ()> {
    match c {
        CH::C11(c1) => progress_c11(c1, x, y, z, ctxt),
        CH::C12(c1) => progress_c12(c1, x, y, z, ctxt),
        _ => Ok(()),
    }
}

fn progress_c11(x: E, a1: E, y: E, a2: E, ctxt: &mut Ctxt) -> Result<(), ()> { visit_c12(y, x, a2, ctxt) } 
fn progress_c12(y: E, x: E, a2: E, a3: E, ctxt: &mut Ctxt) -> Result<(), ()> { prove_triple(y, a3, x, ctxt) }

// called when f(x, y) = z gets proven.
pub fn spawn_cs(x: E, y: E, z: E, ctxt: &mut Ctxt) -> Result<(), ()> {
    spawn_c11(x, y, z, ctxt)?; // argument order always x, y, z.
    Ok(())
}

// f(y, x) = a1.
fn spawn_c11(y: E, x: E, a1: E, ctxt: &mut Ctxt) -> Result<(), ()> { visit_c11(x, a1, y, ctxt) }

// C1
pub fn visit_c11(x: E, a1: E, y: E, ctxt: &mut Ctxt) -> Result<(), ()> {
    match try_f(a1, y, ctxt) {
        Err(i) => {
            add_c(CH::C11(x), i, ctxt);
            Ok(())
        },
        Ok(a2) => progress_c11(x, a1, y, a2, ctxt),
    }
}

fn visit_c12(y: E, x: E, a2: E, ctxt: &mut Ctxt) -> Result<(), ()> {
    match try_f(x, a2, ctxt) {
        Err(i) => {
            add_c(CH::C12(y), i, ctxt);
            Ok(())
        },
        Ok(a3) => progress_c12(y, x, a2, a3, ctxt),
    }
}

// C2
// TODO
pub fn visit_c21(a: E, b: E, c: E, ctxt: &mut Ctxt) -> Result<(), ()> {
    Ok(())
}

pub fn visit_c22(y: E, a: E, neg_b: E, ctxt: &mut Ctxt) -> Result<(), ()> {
    Ok(())
}

// Ok(i) means: f(x, y) = i.
// Err(i) means: you are blocked on the computation of h(i).
fn try_f(x: E, y: E, ctxt: &Ctxt) -> Result<E, E> {
    let r = ctxt.r;
    match (x == r, y == r) {
        (true, true) => return Ok(r),
        (false, true) => return Ok((x + ctxt.a)%r),
        (true, false) => return Ok((y + ctxt.b)%r),
        (false, false) => {},
    }
    let id = (y+r-x)%r;
    let v = ctxt.classes_h[id as usize].value;
    if v == E::MAX { Err(id) } else { Ok((x+v)%r) }
}


fn prove_triple(x: E, y: E, z: E, ctxt: &mut Ctxt) -> Result<(), ()> {
    let r = ctxt.r;
    match try_f(x, y, ctxt) {
        Ok(z2) => assert(z == z2),

        // f(x, y) = x + h(y-x)
        // <-> h(y-x) = f(x, y) - x
        Err(i) => prove_pair(i, (z+r-x)%r, ctxt),
    }
}

fn assert(x: bool) -> Result<(), ()> {
    match x {
        true => Ok(()),
        false => Err(()),
    }
}

fn add_c(c: CH, i: E, ctxt: &mut Ctxt) {
    ctxt.trail.push(TrailEvent::PushCH(i));
    let class = &mut ctxt.classes_h[i as usize];
    class.cs.push(c);
    class.score += score_c(c);
}
