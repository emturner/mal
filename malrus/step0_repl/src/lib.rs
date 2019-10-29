pub fn compile_to_main(mal_input: String) -> String {
    format!("fn main() {{ 
        print!(\"{{}}\", r#\"{}\"#);
    }}", mal_input)
}
