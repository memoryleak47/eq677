use crate::tinv_dpll::*;

// Constraints

pub const C11_SCORE: i32 = 2000;
pub const C12_SCORE: i32 = 3000;

//       0 = y + h(-y + h(y + h(-y) + h(-h(-y)))); i = -y
// (C11) 0 = y + h(-y + h(y + a + h(-a))); i = -a
// (C12) 0 = y + h(-y + h(y + a + b)); i = y + a + b
//       0 = y + h(-y + c)

// the visit_c** functions first get all the CH::C**(..) args, and then i.
// progress_c also, but then also gets v.

#[derive(Clone, Copy)]
pub enum CH {
    C11(/*y*/ E),
    C12(/*y*/ E),
}

pub fn progress_c(c: CH, i: E, v: E, ctxt: &mut Ctxt) -> Result<(), ()> {
    let n = ctxt.n;
    match c {
        CH::C11(y) => {
            let a = (n-i)%n;
            let yab = (y + a + v)%n;
            visit_c12(y, yab, ctxt)
        },
        CH::C12(y) => {
            // 0 = y + h(-y + v)
            // -> h(-y + v) = -y
            let n = ctxt.n;
            prove_pair((n-y+v)%n, (n-y)%n, ctxt)
        },
    }
}

// C1
pub fn visit_c11(y: E, a: E, ctxt: &mut Ctxt) -> Result<(), ()> {
    let n = ctxt.n;
    let neg_a = (n-a)%n;
    let class = &mut ctxt.classes_h[neg_a as usize];
    let v = class.value;
    if v == E::MAX {
        ctxt.trail.push(TrailEvent::PushCH(neg_a));
        class.cs.push(CH::C11(y));
        class.score += C11_SCORE;

        Ok(())
    } else {
        let b = v;
        let yab = (y + a + b)%n;
        visit_c12(y, yab, ctxt)
    }
}

fn visit_c12(y: E, yab: E, ctxt: &mut Ctxt) -> Result<(), ()> {
    let n = ctxt.n;
    let class = &mut ctxt.classes_h[yab as usize];
    let v = class.value;
    if v == E::MAX {
        ctxt.trail.push(TrailEvent::PushCH(yab));
        class.cs.push(CH::C12(y));
        class.score += C12_SCORE;

        Ok(())
    } else {
        let c = v;
        // h(-y + c) = -y
        prove_pair((n-y+c)%n, (n-y)%n, ctxt)
    }
}
