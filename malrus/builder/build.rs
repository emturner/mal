use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join(".malrus.rs");

    let malrus_step = env::var("STEP").expect("$STEP not found");
    let malrus_input = env::var("MALRUS_INPUT").expect("$MALRUS_INPUT not found");

    let mut f = File::create(&dest_path).unwrap();

    let gen = format!("
        use {}::mal;

        mal!({});
    ", malrus_step, malrus_input);
    
    f.write_all(gen.as_bytes()).expect("could not write to file");
}