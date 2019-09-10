use std::io::{self, Write};

mod env;
mod reader;
mod types;
mod printer;

use env::{Env, MalEnv};
use printer::pr_str;
use types::MalType;

fn expect_int_args_fold(args: &[MalType], acc: &MalType, op: fn(i64, i64) -> i64) -> Result<MalType, String> {
    if let MalType::Int(i) = acc {
        let result = args.iter().fold(Ok(*i), |acc, y| match y {
            MalType::Int(i) => Ok(op(acc?, *i)),
            _ => Err(String::from("Expected int")),
        });

        Ok(MalType::Int(result?))
    } else {
        Err(String::from("Expected int"))
    }
}

fn main() -> Result<(), String>
{
    let bindings = vec!("+", "-", "*", "/");
    let vals = vec!(
        MalType::Func(|args| expect_int_args_fold(&args[1..], &args[0], |x, y| x + y)),
        MalType::Func(|args| expect_int_args_fold(&args[1..], &args[0], |x, y| x - y)),
        MalType::Func(|args| expect_int_args_fold(&args[1..], &args[0], |x, y| x * y)),
        MalType::Func(|args| expect_int_args_fold(&args[1..], &args[0], |x, y| x / y)),
    );
    let env = Env::new(None, bindings, vals)?;

    loop 
    {
        print!("user> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input)
        {
            Ok(0) => break,
            Ok(_) => println!("{}", rep(&input, env.clone())),
            Err(e) => 
            {
                println!("{}", e);
                break
            }
        }
    }

    Ok(())
}

fn read(s: &String) -> Result<MalType, String> {
    reader::read_str(s)
}

fn eval(s: Result<MalType, String>, env: MalEnv) -> Result<MalType, String> {
    match s? {
        MalType::List(ref l) if l.len() == 0 => Ok(MalType::List(l.to_vec())),
        MalType::List(ref l) if l.len() > 0 => {
            //println!("l {:?}", l);
            let vec = eval_vec_elemwise(l, env.clone())?;
            //println!("vec {:?}", vec);
            match eval_ast(&vec[0], env.clone())? {
                MalType::Func(f) => f(&vec[1..]),
                _ => Err(String::from("not a function!"))
            }
         },
         s => eval_ast(&s, env) 
     }
}

fn eval_ast(ast: &MalType, env: MalEnv) -> Result<MalType, String> {
    match ast {
        MalType::Symbol(s) => env.borrow().get(s),
        MalType::List(l) => {
            Ok(MalType::List(eval_vec_elemwise(l, env)?))
        },
        MalType::Vector(v) => {
            Ok(MalType::Vector(eval_vec_elemwise(v, env)?))
        }
        _ => Ok(ast.clone()),
    }
}

fn eval_vec_elemwise(vec: &Vec<MalType>, env: MalEnv) -> Result<Vec<MalType>, String> {
    vec.iter().map(|elem| eval(Ok(elem.clone()), env.clone())).collect::<Result<Vec<_>, _>>()
}

fn print(output: Result<MalType, String>) -> String {
    pr_str(output)
}

fn rep(input: &String, env: MalEnv) -> String {
    print(eval(read(input), env))
}
