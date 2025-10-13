use crate::c_dpll::*;

pub type HeapIdx = usize; // index into heap[_].
pub type XYIdx = usize; // output of idx(x, y, ctxt.n).

pub fn heap_push(i: XYIdx, ctxt: &mut Ctxt) {
    let h = ctxt.heap.len();
    ctxt.heap.push(i);
    ctxt.classes_xy[i].heap_index = h;
    heap_swim(i, ctxt);
}

pub fn heap_remove(i: XYIdx, ctxt: &mut Ctxt) {
    let h = ctxt.classes_xy[i].heap_index;
    ctxt.heap.swap_remove(h);
    ctxt.classes_xy[i].heap_index = HeapIdx::MAX;

    if h < ctxt.heap.len() {
        let ii = ctxt.heap[h];
        ctxt.classes_xy[ii].heap_index = h;
        heap_sink(ii, ctxt);
    }
}

pub fn heap_swim(i: XYIdx, ctxt: &mut Ctxt) {
    let class = &ctxt.classes_xy[i];
    let score = class.score;
    let mut h = class.heap_index;

    while h > 0 {
        let p_h = (h-1)/2;
        let p_i = ctxt.heap[p_h];
        let p_score = ctxt.classes_xy[p_i].score;
        if p_score >= score { break }

        ctxt.heap.swap(h, p_h);
        ctxt.classes_xy[i].heap_index = p_h;
        ctxt.classes_xy[p_i].heap_index = h;

        h = p_h;
    }
}

pub fn heap_sink(i: XYIdx, ctxt: &mut Ctxt) {
    let class = &ctxt.classes_xy[i];
    let score = class.score;
    let mut h = class.heap_index;

    loop {
        let l_h = 2*h+1;
        let r_h = 2*h+2;

        let l_score = ctxt.heap.get(l_h).map(|&ii| ctxt.classes_xy[ii].score).unwrap_or(-1);
        let r_score = ctxt.heap.get(r_h).map(|&ii| ctxt.classes_xy[ii].score).unwrap_or(-1);

        if l_score >= r_score {
            if l_score <= score { break }

            let l_i = ctxt.heap[l_h];
            ctxt.heap.swap(h, l_h);
            ctxt.classes_xy[i].heap_index = l_h;
            ctxt.classes_xy[l_i].heap_index = h;
            h = l_h;
        } else {
            if r_score <= score { break }

            let r_i = ctxt.heap[r_h];
            ctxt.heap.swap(h, r_h);
            ctxt.classes_xy[i].heap_index = r_h;
            ctxt.classes_xy[r_i].heap_index = h;
            h = r_h;
        }
    }
}
