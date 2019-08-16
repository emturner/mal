use std::io::{self, Write};
use std::collections::HashMap;

mod reader;
mod types;
mod printer;

use printer::pr_str;
use types::MalType;

type MalEnv = HashMap::<String, MalType>;

fn add(x: i64, y: i64) -> i64 {
    x + y
}
fn sub(x: i64, y: i64) -> i64 {
    x - y
}
fn mul(x: i64, y: i64) -> i64 {
    x * y
}
fn div(x: i64, y: i64) -> i64 {
    x / y
}

fn main()
{
    let mut env = MalEnv::new();
    env.insert(String::from("+"), MalType::Func(add));
    env.insert(String::from("-"), MalType::Func(sub));
    env.insert(String::from("*"), MalType::Func(mul));
    env.insert(String::from("/"), MalType::Func(div));
    loop 
    {
        print!("user> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input)
        {
            Ok(0) => break,
            Ok(_) => println!("{}", rep(&input, &mut env)),
            Err(e) => 
            {
                println!("{}", e);
                break
            }
        }
    }
}

fn read(s: &String) -> Result<MalType, String> {
    reader::read_str(s)
}

fn eval(s: Result<MalType, String>, env: &MalEnv) -> Result<MalType, String> {
    match s? {
        MalType::List(ref l) if l.len() == 0 => Ok(MalType::List(l.to_vec())),
        MalType::List(ref l) if l.len() > 0 => {
            println!("list {:?}", l);
            let vec = eval(s?, env)?;
            println!("evald {:?}", vec);
            match eval_ast(&vec[0], env)? {
                MalType::Func(f) => Ok(MalType::Int(vec[1..].iter().fold(Ok(0), |acc, x| {
                    acc.and_then(|acc| match x {
                        MalType::Int(i) => Ok(f(acc, *i)),
                        _ => Err("Excepted an int")
                    })})?)),
                _ => Err(String::from("not a function!"))
            }
         },
         s => eval_ast(&s, env) 
     }
}

fn eval_ast(ast: &MalType, env: &MalEnv) -> Result<MalType, String> {
    match ast {
        MalType::Symbol(s) => {
            if let Some(val) = env.get(s) {
                Ok(val.clone())
            } else {
                Err(String::from("Not Found"))
            }
        },
        MalType::List(l) => {
            Ok(MalType::List(eval_vec_elemwise(l, env)?))
        },
        _ => Ok(ast.clone()),
    }
}

fn eval_vec_elemwise(vec: &Vec<MalType>, env: &MalEnv) -> Result<Vec<MalType>, String> {
    vec.iter().map(|elem| eval_ast(elem, env)).collect::<Result<Vec<_>, _>>()
}

fn print(output: Result<MalType, String>) -> String {
    pr_str(output)
}

fn rep(input: &String, mut env: &mut MalEnv) -> String {
    print(eval(read(input), env))
}
