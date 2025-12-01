use crate::*;

pub fn load_file(file: &str) {
    let ss = &format!("Loaded from '{file}'");

    let mut s = std::fs::read_to_string(file).unwrap();

    let mut current = String::new();
    for line in s.split("\n") {
        if line.chars().all(|x| x.is_whitespace() || x.is_digit(10)) && line.trim().len() > 4 {
            current.push_str(&line);
            current.push('\n');
        } else {
            current = current.trim().to_string();
            if !current.is_empty() {
                let m = MatrixMagma::parse(&current);
                present_model(m.n, &ss, |x, y| m.f(x, y));
                current = String::new();
            }
        }
    }
}
