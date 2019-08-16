#[derive(Debug)]
pub enum MalType<'a> {
    List(Vec<MalType<'a>>),
    Int(i64),
    Symbol(&'a str),
    Nil(),
    Bool(bool),
}

impl<'a> MalType<'a> {
    pub fn pr_str(&self) -> String {
        match self {
            MalType::List(l) => {
                let vals: String = l.iter().map(|mt| mt.pr_str()).collect::<Vec<String>>().join(" ");
                format!("({})", vals)
            },
            MalType::Symbol(s) => format!("{}", s),
            MalType::Int(i) => format!("{}", i),
            MalType::Nil() => String::from("nil"),
            MalType::Bool(b) => format!("{}", b)
        }
    }
}
