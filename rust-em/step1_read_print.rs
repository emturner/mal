use std::io::{self, Write};

mod reader;

fn main()
{
    loop 
    {
        print!("user> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input)
        {
            Ok(0) => break,
            Ok(_) => println!("{}", rep(&input)),
            Err(e) => 
            {
                println!("{}", e);
                break
            }
        }
    }
}

fn read(s: &String) -> String 
{
    s.to_string()
}

fn eval(s: &String) -> String 
{
    s.to_string()
}

fn print(s: &String) -> String 
{
    s.to_string()
}

fn rep(s: &String) -> String {
    print(&eval(&read(s)))
}
