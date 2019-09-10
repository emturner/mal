use std::io::{self, Write};

mod printer;
mod reader;
mod types;

use printer::pr_str;
use types::MalType;

fn main() {
    loop {
        print!("user> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => println!("{}", rep(&input)),
            Err(e) => {
                println!("{}", e);
                break;
            }
        }
    }
}

fn read(s: &str) -> Result<MalType, String> {
    reader::read_str(s)
}

fn eval(s: Result<MalType, String>) -> Result<MalType, String> {
    s
}

fn print(s: Result<MalType, String>) -> String {
    pr_str(s)
}

fn rep(s: &str) -> String {
    print(eval(read(s)))
}
