use std::fs;
use std::path::Path;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("db_sources.rs");

    let mut out = String::from("pub static DB_SOURCES: &[&str] = &[");

    for entry in fs::read_dir("db").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        let fname = path.file_name().unwrap().to_str().unwrap();
        let path_str = path.to_str().unwrap();


    out.push_str(&format!(
        "include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/db/{}\")),\n",
        fname,
    ));
    }

    out.push_str("];");

    fs::write(dest, out).unwrap();

    println!("cargo:rerun-if-changed=db");
}
