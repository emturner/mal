use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join(".malrus.rs");

    let malrus_step = fs::read_to_string(".mal_step")
        .expect(".mal_step not found");

    let mut f = File::create(&dest_path).unwrap();

    let gen = format!(r#"
        use {}::mal_main;

        mal_main!({});
    "#, malrus_step, fs::canonicalize(Path::new(".malrus.mal")).unwrap().to_string_lossy());

    f.write_all(gen.as_bytes()).expect("could not write to file");
}