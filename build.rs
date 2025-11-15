use std::fs;
use std::path::Path;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("db_sources.rs");

    let mut out = String::from("pub static DB_SOURCES: &[(&str, &str)] = &[");

    let mut files = Vec::new();

    for entry in fs::read_dir("db").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        let fname = path.file_name().unwrap().to_str().unwrap();
        let path_str = path.to_str().unwrap();

        files.push(fname.to_string());
    }

    files.sort_by_cached_key(decompose_filename);

    for fname in files {
        out.push_str(&format!(
            "(\"{fname}\", include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/db/{fname}\"))),\n",
        ));
    }

    out.push_str("];");

    fs::write(dest, out).unwrap();

    println!("cargo:rerun-if-changed=db");
}

fn decompose_filename(x: &String) -> (u32, u32) {
    let mut it = x.split("_");
    let a = it.next().unwrap();
    let b = it.next().unwrap();
    assert!(it.next().is_none());

    let a = a.parse::<u32>().unwrap();
    let b = b.parse::<u32>().unwrap();
    (a, b)
}
