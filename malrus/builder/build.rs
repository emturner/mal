use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

include!(".mal_step");

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join(".malrus.rs");

    let mut f = File::create(&dest_path).unwrap();

    let mal_input = match fs::read_to_string(Path::new(".malrus.mal")) {
        Ok(input) => input,
        Err(e) => {
            println!("ERROR {}", e);
            "".to_string()
        }
    };

    let gen = malrus::compile_to_main(mal_input);

    f.write_all(gen.as_bytes()).expect("could not write to file");
}