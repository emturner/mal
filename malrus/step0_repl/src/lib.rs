use std::fs;
use std::path::Path;

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn mal(input: TokenStream) -> TokenStream {

    let mal_file: String  = input.into_iter().map(|t| t.to_string()).collect();

    let mal_path = Path::new(&mal_file);
    let mal_input = fs::read_to_string(mal_path).expect(&mal_path.to_string_lossy());

    format!("fn run_mal() {{ 
        print!(\"{{}}\", r#\"{}\"#);
    }}", mal_input).parse().unwrap()
}
