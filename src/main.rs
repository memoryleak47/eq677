#![feature(explicit_tail_calls)]

#![allow(unused)] // heh
#![allow(private_interfaces)]
#![allow(irrefutable_let_patterns)]

mod magma;
pub use magma::*;

mod matrix;
pub use matrix::*;

mod eq_dpll;
pub use eq_dpll::*;

mod sym_dpll;
pub use sym_dpll::*;

mod tinv_dpll;
pub use tinv_dpll::*;

mod semitinv_dpll;
pub use semitinv_dpll::*;

mod c_dpll;
pub use c_dpll::*;

mod composite;
pub use composite::*;

mod db;
pub use db::*;

mod conj;
pub use conj::*;

mod parallel;
pub use parallel::*;

mod present;
pub use present::*;

mod search;
pub use search::*;

mod kb;
pub use kb::*;

mod twee;
pub use twee::*;

mod twee_sys;
pub use twee_sys::*;

mod timer;
pub use timer::*;

mod fo;
pub use fo::*;

mod analysis;
pub use analysis::*;

mod one_orbit;
pub use one_orbit::*;

mod autom_search;
pub use autom_search::*;

mod load;
pub use load::*;

mod glue5;
pub use glue5::*;

fn main() {
    setup_panic_hook();
    let _timer = Timer::new();

    let m = MatrixMagma::parse("
0  4  5  6  7  8  9  10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 1  2  3
17 1  15 0  12 20 7  8  19 16 5  9  18 6  23 4  2  13 11 3  14 22 10 24 21
18 0  2  13 8  10 21 17 9  20 16 6  7  5  4  24 12 3  14 23 1  15 19 11 22
16 14 0  3  19 9  11 21 18 7  8  17 4  22 6  5  15 10 1  13 24 2  23 20 12
21 13 23 2  4  0  17 14 10 24 12 19 22 9  20 11 8  7  3  6  15 16 5  1  18
19 3  14 24 18 5  0  22 15 11 23 10 20 12 7  21 1  9  8  17 4  13 16 6  2
20 22 1  15 0  16 6  12 23 13 21 24 11 19 10 8  9  2  7  14 18 5  3  17 4
23 9  20 4  16 6  3  7  21 0  18 2  13 14 1  22 11 15 24 12 5  10 8  19 17
24 5  7  21 1  17 4  0  8  19 14 16 3  23 15 2  22 12 13 11 10 6  18 9  20
22 19 6  8  5  2  18 20 0  9  1  15 17 3  24 13 14 23 10 4  12 11 21 16 7
3  12 21 22 11 7  24 19 5  8  10 0  23 20 16 6  18 1  4  15 2  17 14 13 9
1  23 10 19 22 12 8  9  20 6  24 11 0  4  21 17 5  16 2  18 13 3  7  15 14
2  20 24 11 9  23 10 4  7  21 0  22 12 18 5  19 3  6  17 1  16 14 15 8  13
5  18 11 16 14 1  23 15 2  10 22 12 9  13 3  0  24 8  19 20 7  4  17 21 6
6  17 16 12 24 15 2  11 13 3  7  23 10 0  14 1  20 22 9  5  21 8  4  18 19
4  10 18 17 3  22 13 1  12 14 11 8  24 2  0  15 7  21 23 9  6  19 20 5  16
9  21 8  23 20 19 15 18 3  4  17 13 6  1  11 14 16 0  5  2  22 12 24 7  10
7  24 19 9  13 21 20 5  16 1  4  18 14 15 2  12 6  17 0  10 3  23 11 22 8
8  7  22 20 21 14 19 2  6  17 15 5  16 10 13 3  0  4  18 24 11 1  9  12 23
11 2  13 10 23 3  12 24 17 22 20 7  5  21 8  16 4  18 15 19 9  0  6  14 1
12 11 3  14 10 24 1  23 22 18 6  21 8  17 19 9  13 5  16 0  20 7  2  4  15
10 15 12 1  2  11 22 16 24 23 9  4  19 7  18 20 17 14 6  8  0  21 13 3  5
15 8  4  18 6  13 16 3  14 5  2  1  21 24 9  10 23 19 12 7  17 20 22 0  11
13 16 9  5  17 4  14 6  1  15 19 3  2  11 22 7  10 24 20 21 8  18 12 23 0
14 6  17 7  15 18 5  13 4  2  3  20 1  8  12 23 21 11 22 16 19 9  0  10 24
    ");
    println!("------");
    m.dump();
    println!("------");
    m.cycle_dump();
    println!("------");
    m.column_cycle_dump();
    println!("------");
    m.z_cycle_dump();
}
