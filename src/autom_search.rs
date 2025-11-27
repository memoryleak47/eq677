use crate::*;

pub fn db_autom_search() {
    for (name, m) in db() {
        if m.n < 2 { continue }

        println!("Considering magma {name}");
        let grp = m.autom_group_mini();
        for x in &grp {
            draw_cycle(0, m.n, |i| x[i]);
            println!();
        }
        println!("group computed!");
        let grp = grp.into_iter().map(|x| x.into_iter().map(|y| y as E).collect()).collect();
        c_run(m.n, grp);
    }
}
