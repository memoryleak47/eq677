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

mod check;
pub use check::*;

mod prop_combo;
pub use prop_combo::*;

mod enumerate;
pub use enumerate::*;

mod combine;
pub use combine::*;

mod uf;
pub use uf::*;

fn main() {
    setup_panic_hook();
    let _timer = Timer::new();

    let m = MatrixMagma::parse("
-  -  -  -  14 19 12 09 18 23 16 13 22 07 20 17 06 11 04 21 10 15 08 05 -
-  -  -  -  06 15 20 13 10 19 04 17 14 23 08 21 18 07 12 05 22 11 16 09 -
-  -  -  -  10 07 16 21 14 11 20 05 18 15 04 09 22 19 08 13 06 23 12 17 -
-  -  -  -  18 11 08 17 22 15 12 21 06 19 16 05 10 23 20 09 14 07 04 13 -
21 09 05 17 -  24 15 14 -  03 23 06 -  00 11 18 -  02 19 10 -  01 07 22 13
18 22 10 06 23 -  24 16 15 -  00 04 07 -  01 12 19 -  03 20 11 -  02 08 14
07 19 23 11 09 04 -  24 17 16 -  01 05 08 -  02 13 20 -  00 21 12 -  03 15
12 08 20 04 00 10 05 -  24 18 17 -  02 06 09 -  03 14 21 -  01 22 13 -  16
05 13 09 21 -  01 11 06 -  24 19 18 -  03 07 10 -  00 15 22 -  02 23 14 17
22 06 14 10 15 -  02 12 07 -  24 20 19 -  00 08 11 -  01 16 23 -  03 04 18
11 23 07 15 05 16 -  03 13 08 -  24 21 20 -  01 09 12 -  02 17 04 -  00 19
16 12 04 08 01 06 17 -  00 14 09 -  24 22 21 -  02 10 13 -  03 18 05 -  20
09 17 13 05 -  02 07 18 -  01 15 10 -  24 23 22 -  03 11 14 -  00 19 06 21
06 10 18 14 07 -  03 08 19 -  02 16 11 -  24 04 23 -  00 12 15 -  01 20 22
15 07 11 19 21 08 -  00 09 20 -  03 17 12 -  24 05 04 -  01 13 16 -  02 23
20 16 08 12 03 22 09 -  01 10 21 -  00 18 13 -  24 06 05 -  02 14 17 -  04
13 21 17 09 -  00 23 10 -  02 11 22 -  01 19 14 -  24 07 06 -  03 15 18 05
10 14 22 18 19 -  01 04 11 -  03 12 23 -  02 20 15 -  24 08 07 -  00 16 06
19 11 15 23 17 20 -  02 05 12 -  00 13 04 -  03 21 16 -  24 09 08 -  01 07
04 20 12 16 02 18 21 -  03 06 13 -  01 14 05 -  00 22 17 -  24 10 09 -  08
17 05 21 13 -  03 19 22 -  00 07 14 -  02 15 06 -  01 23 18 -  24 11 10 09
14 18 06 22 11 -  00 20 23 -  01 08 15 -  03 16 07 -  02 04 19 -  24 12 10
23 15 19 07 13 12 -  01 21 04 -  02 09 16 -  00 17 08 -  03 05 20 -  24 11
08 04 16 20 24 14 13 -  02 22 05 -  03 10 17 -  01 18 09 -  00 06 21 -  12
-  -  -  -  22 23 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 - ");

    c_complete(&m);
}
