use crate::c_dpll::*;

pub fn heap_push(x: E, y: E, score: i32, ctxt: &mut Ctxt) {
    let i = ctxt.heap.len();
    ctxt.heap.push((x, y));
    ctxt.classes_xy[idx(x, y, ctxt.n)].heap_index = i;
    heap_swim(x, y, ctxt);
}

pub fn heap_add_score(x: E, y: E, summand: i32, ctxt: &mut Ctxt) {
    ctxt.classes_xy[idx(x, y, ctxt.n)].score += summand;
    heap_swim(x, y, ctxt);
}

pub fn heap_pop(ctxt: &mut Ctxt) -> Option<(E, E)> {
    if ctxt.heap.is_empty() { return None }
    let (x, y) = ctxt.heap.swap_remove(0);
    ctxt.classes_xy[idx(x, y, ctxt.n)].heap_index = usize::MAX;

    if !ctxt.heap.is_empty() {
        let (x2, y2) = ctxt.heap[0];
        ctxt.classes_xy[idx(x2, y2, ctxt.n)].heap_index = 0;
        heap_sink(x2, y2, ctxt);
    }

    Some((x, y))
}

fn heap_swim(x: E, y: E, ctxt: &mut Ctxt) {
    let i = idx(x, y, ctxt.n);
    let class = &ctxt.classes_xy[i];
    let score = class.score;
    let mut h = class.heap_index;

    while h > 0 {
        let p_h = (h-1)/2;
        let (p_x, p_y) = ctxt.heap[p_h];
        let p_i = idx(p_x, p_y, ctxt.n);
        let p_score = ctxt.classes_xy[p_i].score;
        if p_score >= score { break }

        ctxt.heap.swap(h, p_h);
        ctxt.classes_xy[i].heap_index = p_h;
        ctxt.classes_xy[p_i].heap_index = h;

        h = p_h;
    }
}

fn heap_sink(x: E, y: E, ctxt: &mut Ctxt) {
    let i = idx(x, y, ctxt.n);
    let class = &ctxt.classes_xy[i];
    let score = class.score;
    let mut h = class.heap_index;

    loop {
        let l_h = 2*h+1;
        let r_h = 2*h+2;

        let l_score = ctxt.heap.get(l_h).map(|&(x, y)| ctxt.classes_xy[idx(x, y, ctxt.n)].score).unwrap_or(-1);
        let r_score = ctxt.heap.get(r_h).map(|&(x, y)| ctxt.classes_xy[idx(x, y, ctxt.n)].score).unwrap_or(-1);

        if l_score >= r_score {
            if l_score <= score { break }

            let (l_x, l_y) = ctxt.heap[l_h];
            let l_i = idx(l_x, l_y, ctxt.n);
            ctxt.heap.swap(h, l_h);
            ctxt.classes_xy[i].heap_index = l_h;
            ctxt.classes_xy[l_i].heap_index = h;
            h = l_h;
        } else {
            if r_score <= score { break }

            let (r_x, r_y) = ctxt.heap[r_h];
            let r_i = idx(r_x, r_y, ctxt.n);
            ctxt.heap.swap(h, r_h);
            ctxt.classes_xy[i].heap_index = r_h;
            ctxt.classes_xy[r_i].heap_index = h;
            h = r_h;
        }
    }
}
