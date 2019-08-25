use std::io::{self, Write};

mod env;
mod printer;
mod reader;
mod types;

use env::{Env, MalEnv};
use printer::pr_str;
use types::MalType;

fn expect_int_args_fold(args: &[MalType], acc: &MalType, op: fn(i64, i64) -> i64) -> Result<MalType, String> {
    if let MalType::Int(a) = acc {
        let result = args.iter().fold(Ok(*a), |acc, y| match y {
            MalType::Int(i) => Ok(op(acc?, *i)),
            _ => Err(String::from("Expected int")),
        });
        Ok(MalType::Int(result?))
    } else {
        Err(String::from("Expected int"))
    }
}

fn main() -> Result<(), String> {

    let bindings = vec!("+", "-", "*", "/");
    let vals = vec!(
        MalType::Func(|args| expect_int_args_fold(&args[1..], &args[0], |x, y| x + y)),
        MalType::Func(|args| expect_int_args_fold(&args[1..], &args[0], |x, y| x - y)),
        MalType::Func(|args| expect_int_args_fold(&args[1..], &args[0], |x, y| x * y)),
        MalType::Func(|args| expect_int_args_fold(&args[1..], &args[0], |x, y| x / y)),
    );
    let env = Env::new(None, bindings, vals)?;

    loop {
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
        MalType::List(ref l) => {
            match l[0] {
                MalType::Symbol(ref s) if s == "def!" => {
                    if l.len() == 3 {
                        match &l[1] {
                            MalType::Symbol(ref binding) => {
                                let val = eval(Ok(l[2].clone()), env.clone());
                                env.borrow_mut().set(binding, val.clone()?);
                                val
                            },
                            _ => Err(String::from("Expected Symbol"))
                        }
                    } else {
                        Err(String::from("Expected 2 parameters to 'def!' binding"))
                    }
                }
                MalType::Symbol(ref s) if s == "let*" => {
                    if l.len() == 3 {
                        let mut bindings = match &l[1] {
                            MalType::List(ref b) => b,
                            MalType::Vector(ref b) => b,
                            _ => return Err(String::from("Expected List or Vector"))
                        }.iter();
                        let inner = Env::new(Some(env.clone()), vec!(), vec!())?;
                        
                        while let Some(b) = bindings.next() {
                            if let MalType::Symbol(s) = b {
                                if let Some(val) = bindings.next() {
                                    let val = eval(Ok(val.clone()), inner.clone())?;
                                    inner.borrow_mut().set(s, val);
                                } else {
                                    return Err(String::from("Expected value for binding"));
                                }
                            } else {
                                return Err(String::from("Expected symbol"));
                            }
                        }

                        eval(Ok(l[2].clone()), inner)
                    } else {
                        Err(String::from("Expected 2 parameters to 'let*' bindings"))
                    }
                },
                MalType::Symbol(ref s) if s == "do" => {
                    let vec = eval_vec_elemwise(&l[1..], env.clone())?;
                    Ok(vec[vec.len() - 1].clone())
                }
                _ => {
                    let vec = eval_vec_elemwise(l, env.clone())?;
                    match eval_ast(&vec[0], env.clone())? {
                        MalType::Func(f) => f(&vec[1..]),
                        _ => Err(String::from("not a function!"))
                    }
                }
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

fn eval_vec_elemwise(vec: &[MalType], env: MalEnv) -> Result<Vec<MalType>, String> {
    vec.iter().map(|elem| eval(Ok(elem.clone()), env.clone())).collect::<Result<Vec<_>, _>>()
}

fn print(output: Result<MalType, String>) -> String {
    pr_str(output)
}

fn rep(input: &String, env: MalEnv) -> String {
    print(eval(read(input), env))
}
