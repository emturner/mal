use std::io::{self, Write};
use std::rc::Rc;
use std::cell::RefCell;

mod env;
mod printer;
mod reader;
mod types;

use env::MalEnv;
use printer::pr_str;
use types::MalType;

type RefMalEnv = Rc<RefCell<MalEnv>>;

fn main()
{
    let env = MalEnv::new(None);
    env.borrow_mut().set("+", MalType::Func(|x, y| x + y));
    env.borrow_mut().set("-", MalType::Func(|x, y| x - y));
    env.borrow_mut().set("*", MalType::Func(|x, y| x * y));
    env.borrow_mut().set("/", MalType::Func(|x, y| x / y));

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
}

fn read(s: &String) -> Result<MalType, String> {
    reader::read_str(s)
}

fn eval(s: Result<MalType, String>, env: RefMalEnv) -> Result<MalType, String> {
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
                            x => Err(String::from(format!("Expected Symbol, got {:?}", x)))
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
                        let inner = MalEnv::new(Some(env.clone()));
                        
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
                _ => {
                    let vec = eval_vec_elemwise(l, env.clone())?;
                    match (eval_ast(&vec[0], env.clone())?, &vec[1]) {
                        (MalType::Func(f), MalType::Int(acc)) => Ok(MalType::Int(vec[2..].iter().fold(Ok(*acc), |acc, x| {
                            acc.and_then(|acc| match x {
                                MalType::Int(i) => Ok(f(acc, *i)),
                                _ => Err("Excepted an int")
                            })})?)),
                        _ => Err(String::from("not a function!"))

                    }
                }
            }
         },
         s => eval_ast(&s, env) 
     }
}

fn eval_ast(ast: &MalType, env: RefMalEnv) -> Result<MalType, String> {
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

fn eval_vec_elemwise(vec: &Vec<MalType>, env: RefMalEnv) -> Result<Vec<MalType>, String> {
    vec.iter().map(|elem| eval(Ok(elem.clone()), env.clone())).collect::<Result<Vec<_>, _>>()
}

fn print(output: Result<MalType, String>) -> String {
    pr_str(output)
}

fn rep(input: &String, env: RefMalEnv) -> String {
    print(eval(read(input), env))
}
