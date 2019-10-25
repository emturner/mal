extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn mal(input: TokenStream) -> TokenStream {
    format!("fn run_mal() {{ 
        let result = \"{}\";

        println!(\"{{}}\", result);
    }}", input.to_string()).parse().unwrap()
}
