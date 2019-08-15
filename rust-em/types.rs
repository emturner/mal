#[derive(Debug)]
pub enum MalType<'a> {
    List(Vec<MalType<'a>>),
    Int(i64),
    Symbol(&'a str)
}

impl<'a> MalType<'a> {
    pub fn pr_str(&self) -> String {
        match self {
            MalType::List(l) => {
                let vals: String = l.iter().map(|mt| mt.pr_str()).collect();
                format!("({})", vals)
            },
            MalType::Symbol(s) => format!("{}", s),
            MalType::Int(i) => format!("{}", i)
        }
    }
}
