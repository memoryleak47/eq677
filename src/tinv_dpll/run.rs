use crate::tinv_dpll::*;

use std::sync::atomic::{AtomicUsize, Ordering};

const USE_COUNTER: bool = false;

fn threading_depth(n: E) -> E { n + 1 }

static RUNS_STARTED: AtomicUsize = AtomicUsize::new(0);
static RUNS_FINISHED: AtomicUsize = AtomicUsize::new(0);

pub fn tinv_run(n: usize) {
    let mut ctxt = build_ctxt(n);
    prerun(0, &mut ctxt);
}

pub fn tinv_search() {
    for i in 0.. {
        c_run(i);
    }
}

fn inc_counter(ctr: &AtomicUsize) {
    if !USE_COUNTER { return }

    let t = PRINT_MUTEX.lock().unwrap();
    ctr.fetch_add(1, Ordering::SeqCst);

    let started = RUNS_STARTED.load(Ordering::SeqCst);
    let finished = RUNS_FINISHED.load(Ordering::SeqCst);

    let running = started - finished;
    println!("running: {running}, finished: {finished}");
}

fn prerun(depth: E, ctxt: &mut Ctxt) {
    // No need to have a trail, we won't backtrack in the prerun.
    ctxt.trail.clear();

    if depth >= threading_depth(ctxt.n) {
        inc_counter(&RUNS_STARTED);

        main_branch(ctxt);

        inc_counter(&RUNS_FINISHED);
        return;
    }

    let Some(i) = select_p(ctxt) else {
        submit_model(ctxt);
        return;
    };

    range_for_each(ctxt.n, |v| {
        if ctxt.classes_hinv[v as usize].value != E::MAX { return }
        let c = &mut ctxt.clone();

        if prove_pair(i, v, c).is_err() { return }
        if propagate(c).is_err() { return }

        prerun(depth+1, c);
    });
}

fn score_c(c: CH) -> i32 {
    match c {
        CH::C11(..) => C11_SCORE,
        CH::C12(..) => C12_SCORE,
        CH::C21(..) => C21_SCORE,
        CH::C22(..) => C22_SCORE,
    }
}

fn pos_score(i: E, ctxt: &Ctxt) -> i32 {
    let ii = i as i32;
    let ni = ctxt.n as i32;
    ni - ii
}

pub fn compute_base_score(i: E, ctxt: &Ctxt) -> i32 {
    let class = &ctxt.classes_h[i as usize];
    let cs_score = class.cs.iter().map(|c| score_c(*c)).sum::<i32>();
    let pos_score = pos_score(i, ctxt);

    cs_score + pos_score
}

// returns None if we are done.
fn select_p(ctxt: &Ctxt) -> Option<E> {
    // Should hold here:
    // check_score(ctxt);

    let mut best = E::MAX;
    let mut best_score = -1;

    for i in 0..ctxt.n {
        let class = &ctxt.classes_h[i as usize];
        let score = class.score;
        if (class.value == E::MAX) & (score > best_score) {
            best = i;
            best_score = score;
        }
    }

    if best_score == -1 { None }
    else { Some(best) }
}

fn submit_model(ctxt: &Ctxt) {
    present_model(ctxt.n as usize, "tinv_dpll", |x, y| f(x as E, y as E, ctxt) as usize );
}

fn main_branch(ctxt: &mut Ctxt) {
    let Some(i) = select_p(ctxt) else {
        submit_model(ctxt);
        become main_backtrack(ctxt);
    };

    if branch_options(i, 0, ctxt).is_ok() { become main_propagate(ctxt); }
    else { become main_backtrack(ctxt); }
}

// e is the next thing to try. If that doesn't work, we iterate from there.
fn branch_options(i: E, mut e: E, ctxt: &mut Ctxt) -> Result<(), ()> {
    loop {
        if e >= ctxt.n { return Err(()) }
        if ctxt.classes_hinv[e as usize].value == E::MAX { break }
        e += 1;
    }
    ctxt.trail.push(TrailEvent::Decision(i, e));
    prove_pair(i, e, ctxt)?;

    Ok(())
}

fn main_backtrack(ctxt: &mut Ctxt) {
    ctxt.propagate_queue.clear();

    loop {
        let Some(event) = ctxt.trail.pop() else { return };
        match event {
            TrailEvent::Decision(i, e) => {
                if branch_options(i, e+1, ctxt).is_ok() { become main_propagate(ctxt); }
            },
            TrailEvent::DefineClass(i) => {
                let z = std::mem::replace(&mut ctxt.classes_h[i as usize].value, E::MAX);
                ctxt.classes_hinv[z as usize].value = E::MAX;
            },
            TrailEvent::PushCH(i) => {
                let class = &mut ctxt.classes_h[i as usize];
                let c = class.cs.pop().unwrap();
                class.score -= score_c(c);
            }
        }
    }
}

pub fn main_propagate(ctxt: &mut Ctxt) {
    match propagate(ctxt) {
        Ok(()) => become main_branch(ctxt),
        Err(()) => become main_backtrack(ctxt),
    }
}

pub fn prove_pair(i: E, v: E, ctxt: &mut Ctxt) -> Result<(), ()> {
    let i_ref = &mut ctxt.classes_h[i as usize].value;
    let i_v = *i_ref;
    if i_v == v { return Ok(()) }
    if i_v != E::MAX { return Err(()) }

    let hi_ref = &mut ctxt.classes_hinv[v as usize].value;
    if *hi_ref != E::MAX { return Err(()); }

    *i_ref = v;
    *hi_ref = i;
    ctxt.trail.push(TrailEvent::DefineClass(i));
    ctxt.propagate_queue.push((i, v));
    Ok(())
}

// f(x, y) = z <-> x + h(y-x) = z <-> h(y-x) = z-x
pub fn prove_triple(x: E, y: E, z: E, ctxt: &mut Ctxt) -> Result<(), ()> {
    let n = ctxt.n;
    prove_pair((y+n-x)%n, (z+n-x)%n, ctxt)
}

pub fn propagate(ctxt: &mut Ctxt) -> Result<(), ()> {
    let n = ctxt.n;
    while let Some((i, v)) = ctxt.propagate_queue.pop() {
        // spawn constraints!
        visit_c11((n-i)%n, (n-v)%n, ctxt)?;
        visit_c21((n-i)%n, v, ctxt)?;

        let len = ctxt.classes_h[i as usize].cs.len();
        for j in 0..len {
            let c = ctxt.classes_h[i as usize].cs[j];
            progress_c(c, i, v, ctxt)?;
        }
    }

    Ok(())
}

fn check_score(ctxt: &Ctxt) {
    for i in 0..ctxt.n {
        let actual = compute_base_score(i, ctxt);
        let stored = ctxt.classes_h[i as usize].score;
        assert_eq!(actual, stored);
    }
}
