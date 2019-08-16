#[derive(Debug, Clone)]
pub enum MalType {
    List(Vec<MalType>),
    Vector(Vec<MalType>),
    Int(i64),
    Symbol(String),
    Nil(),
    Bool(bool),
    Func(fn (i64, i64) -> i64)
}

impl MalType {
    pub fn pr_str(&self) -> String {
        match self {
            MalType::List(l) => {
                let vals: String = l.iter().map(|mt| mt.pr_str()).collect::<Vec<String>>().join(" ");
                format!("({})", vals)
            },
            MalType::Vector(v) => {
                let vals: String = v.iter().map(|mt| mt.pr_str()).collect::<Vec<String>>().join(" ");
                format!("[{}]", vals)
            },
            MalType::Symbol(s) => format!("{}", s),
            MalType::Int(i) => format!("{}", i),
            MalType::Nil() => String::from("nil"),
            MalType::Bool(b) => format!("{}", b),
            MalType::Func(f) => format!("{:?}", f)
        }
    }
}
