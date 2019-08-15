use types::MalType;

pub fn pr_str(mal_type: Result<MalType, String>) -> String {
    match mal_type {
        Ok(mt) => mt.pr_str(),
        Err(m) => m
    }
}
