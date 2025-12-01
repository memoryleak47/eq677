use std::fs;
use std::path::Path;

fn iter_dir(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    for entry in fs::read_dir(s).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        let fname = path.file_name().unwrap().to_str().unwrap();

        out.push(fname.to_string());
    }
    out
}

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("db_sources.rs");

    let mut out = String::from("pub static DB_SOURCES: &[((usize, usize), &str)] = &[");

    let mut files = Vec::new();

    for i in iter_dir("db") {
        let i = i.parse::<u32>().unwrap();
        for f in iter_dir(&format!("db/{i}")) {
            let f = f.parse::<u32>().unwrap();
            files.push((i, f));
        }
    }

    files.sort();

    for (i, f) in files {
        out.push_str(&format!(
            "(({i}, {f}), include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/db/{i}/{f}\"))),\n",
        ));
    }

    out.push_str("];");

    fs::write(dest, out).unwrap();

    println!("cargo:rerun-if-changed=db");
}
