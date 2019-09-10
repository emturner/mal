#[derive(Clone)]
pub enum MalType {
    List(Vec<MalType>),
    Vector(Vec<MalType>),
    Int(i64),
    Symbol(String),
    Nil(),
    Bool(bool),
    Func(fn(args: &[MalType]) -> Result<MalType, String>),
}

impl MalType {
    pub fn pr_str(&self) -> String {
        match self {
            MalType::List(l) => {
                let vals: String = l
                    .iter()
                    .map(|mt| mt.pr_str())
                    .collect::<Vec<String>>()
                    .join(" ");
                format!("({})", vals)
            }
            MalType::Vector(v) => {
                let vals: String = v
                    .iter()
                    .map(|mt| mt.pr_str())
                    .collect::<Vec<String>>()
                    .join(" ");
                format!("[{}]", vals)
            }
            MalType::Symbol(s) => s.to_string(),
            MalType::Int(i) => i.to_string(),
            MalType::Nil() => String::from("nil"),
            MalType::Bool(b) => b.to_string(),
            MalType::Func(_) => String::from("#<function>"),
        }
    }
}
