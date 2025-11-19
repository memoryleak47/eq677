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

#[derive(Clone, Copy)]
pub enum CH {
    C11(/*x*/ E, /*y*/ E, /*a1*/ E),
    C12(/*x*/ E, /*y*/ E, /*a2*/ E),
}

pub fn progress_c(c: CH, ctxt: &mut Ctxt) -> Result<(), ()> {
    match c {
        CH::C11(x, y, a1) => visit_c11(x, y, a1, ctxt),
        CH::C12(x, y, a2) => visit_c12(x, y, a2, ctxt),
    }
}

fn progress_c11(x: E, y: E, a1: E, a2: E, ctxt: &mut Ctxt) -> Result<(), ()> { visit_c12(x, y, a2, ctxt) }
fn progress_c12(x: E, y: E, a2: E, a3: E, ctxt: &mut Ctxt) -> Result<(), ()> { prove_triple(y, a3, x, ctxt) }

// C1
pub fn visit_c11(x: E, y: E, a1: E, ctxt: &mut Ctxt) -> Result<(), ()> {
    match try_f(a1, y, ctxt) {
        Err(i) => {
            add_c(CH::C11(x, y, a1), i, ctxt);
            Ok(())
        },
        Ok(a2) => progress_c11(x, y, a1, a2, ctxt),
    }
}

fn visit_c12(x: E, y: E, a2: E, ctxt: &mut Ctxt) -> Result<(), ()> {
    match try_f(x, a2, ctxt) {
        Err(i) => {
            add_c(CH::C12(x, y, a2), i, ctxt);
            Ok(())
        },
        Ok(a3) => progress_c12(x, y, a2, a3, ctxt),
    }
}

// f(r, r) = r
// f(i, r) = a+i
// f(r, j) = b+j
// f(i, j) = i + h(j-i), where i+r = r.

// Ok(i) means: f(x, y) = i.
// Err(i) means: you are blocked on the computation of h(i).
pub fn try_f(x: E, y: E, ctxt: &Ctxt) -> Result<E, E> {
    let r = ctxt.r;
    match (x == r, y == r) {
        (true, true) => return Ok(r),
        (false, true) => return Ok((x + ctxt.a)%r),
        (true, false) => return Ok((y + ctxt.b)%r),
        (false, false) => {},
    }
    let id = (y+r-x)%r;
    let v = ctxt.classes_h[id as usize].value;
    if v == E::MAX { Err(id) }
    else if v == r { Ok(r) }
    else { Ok((x+v)%r) }
}


fn prove_triple(x: E, y: E, z: E, ctxt: &mut Ctxt) -> Result<(), ()> {
    let r = ctxt.r;
    match try_f(x, y, ctxt) {
        Ok(z2) => assert(z == z2),

        // note: if x=r or y=r, then we are guaranteed to be in the Ok-case above.
        // Thus we are in case four:
        // f(x, y) = x + h(y-x)
        // <-> h(y-x) = f(x, y) - x
        Err(i) => {
            let v = if z == r { r } else { (z+r-x)%r };
            prove_pair(i, v, ctxt)
        },
    }
}

// SPAWNING:

// called when f(x, y) = z gets proven.
pub fn spawn_cs(x: E, y: E, z: E, ctxt: &mut Ctxt) -> Result<(), ()> {
    if f(x, y, ctxt) != z {
        dbg!(x);
        dbg!(y);
        dbg!(z);
        dbg!(f(x, y, ctxt));
        dbg!(ctxt.classes_h.iter().map(|x| x.value).collect::<Vec<_>>());
        panic!("spawn_cs called on untrue triple.");
    }

    spawn_c11(x, y, z, ctxt)?; // argument order always x, y, z.
    Ok(())
}

// f(y, x) = a1.
fn spawn_c11(y: E, x: E, a1: E, ctxt: &mut Ctxt) -> Result<(), ()> { visit_c11(x, y, a1, ctxt) }


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
