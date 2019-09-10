use std::io::{self, Write};

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

fn read(s: &str) -> String {
    s.to_string()
}

fn eval(s: &str) -> String {
    s.to_string()
}

fn print(s: &str) -> String {
    s.to_string()
}

fn rep(s: &str) -> String {
    print(&eval(&read(s)))
}
