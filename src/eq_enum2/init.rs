use crate::eq_enum2::*;

pub fn build_ctxt(n: usize) -> Ctxt {
    let mut ctxt = Ctxt::default();
    ctxt.n = n;
    add_constraints(&mut ctxt);
    ctxt
}

fn add_constraints(ctxt: &mut Ctxt) {
    let n = ctxt.n;
    let mut xs = Vec::new();
    for x_id in 0..n {
        xs.push(build_elem(x_id, ctxt));
    }
    for x_id in 0..n {
        for y_id in 0..n {
            let x = xs[x_id];
            let y = xs[y_id];
            let yx = build_f(y, x, ctxt);

            let t = build_f(yx, y, ctxt);
            let t = build_f(x, t, ctxt);
            let t = build_f(y, t, ctxt);
            build_assert(x_id, t, ctxt);

            let t = build_f(y, yx, ctxt);
            let t = build_f(t, y, ctxt);
            let t = build_f(yx, t, ctxt);
            build_assert(x_id, t, ctxt);
        }
    }
}

fn build_elem(e: ElemId, ctxt: &mut Ctxt) -> TermId {
    ctxt.classes.push(Class {
        node: Node::Elem(e),
        parents: Vec::new(),
        value: Some(e),
    });
    ctxt.classes.len() - 1
}

fn build_f(l: TermId, r: TermId, ctxt: &mut Ctxt) -> TermId {
    ctxt.classes.push(Class {
        node: Node::F(l, r),
        parents: Vec::new(),
        value: None,
    });
    let out = ctxt.classes.len() - 1;
    ctxt.classes[l].parents.push(out);
    ctxt.classes[r].parents.push(out);

    if let (Node::Elem(l), Node::Elem(r)) = (&ctxt.classes[l].node, &ctxt.classes[r].node) {
        ctxt.pos_terms.entry((*l, *r)).or_default().push(out);
    }

    out
}

fn build_assert(l: ElemId, r: TermId, ctxt: &mut Ctxt) {
    ctxt.classes.push(Class {
        node: Node::AssertEq(l, r),
        parents: Vec::new(),
        value: None,
    });
    let out = ctxt.classes.len() - 1;
    ctxt.classes[r].parents.push(out);
}
