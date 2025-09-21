use crate::eq_enum2::*;

impl Ctxt {
    pub fn dump_classes(&self) {
        for (i, c) in self.classes.iter().enumerate() {
            let n = match &c.node {
                Node::Elem(e) => format!("Elem({e})"),
                Node::F(l, r) => format!("F({l}, {r})"),
                Node::AssertEq(l, r) => format!("AssertEq({l}, {r})"),
            };
            let v = match c.value {
                None => String::new(),
                Some(v) => format!(" (equal to {v})"),
            };
            let mut u = format!(" [parents = ");
            for (j, p) in c.parents.iter().enumerate() {
                u.push_str(&p.to_string());
                if j != c.parents.len() - 1 {
                    u.push_str(", ");
                }
            };
            u.push(']');
            println!("{i} := {n:20}{v:20}{u}");
        }
    }

    pub fn dump_table(&self) {
        for x in 0..self.n {
            for y in 0..self.n {
                if let Some(z) = self.table.get(&(x, y)) {
                    println!("f({x}, {y}) := {z}");
                }
            }
        }
    }

    pub fn dump_pos_terms(&self) {
        for x in 0..self.n {
            for y in 0..self.n {
                if let Some(z) = self.pos_terms.get(&(x, y)) {
                    println!("pos_terms({x}, {y}) := {z:?}");
                }
            }
        }
    }

    pub fn dump(&self) {
        println!("==============================================================================");
        self.dump_classes();
        println!("------------------------------------------------------------------------------");
        self.dump_table();
        println!("------------------------------------------------------------------------------");
        self.dump_pos_terms();
        println!("==============================================================================");
    }
}
