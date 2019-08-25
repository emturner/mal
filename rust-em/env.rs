use types::MalType;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type MalEnv = Rc<RefCell<Env>>;

pub struct Env {
    outer: Option<MalEnv>,
    data: HashMap<String, MalType>
}

impl Env {
    pub fn new(outer: Option<MalEnv>, bindings: Vec<&str>, vals: Vec<MalType>) -> Result<MalEnv, String> {
        let mut env = Env {
            outer: outer,
            data: HashMap::new()
        };

        if bindings.len() == vals.len() {
            for (b, v) in bindings.iter().zip(vals) {
                env.set(b, v);
            }
            Ok(Rc::new(RefCell::new(env)))
        } else {
            Err(String::from("Bindings list has a different length to the values list"))
        }
    }

    pub fn set(&mut self, symbol: &str, value: MalType) {
        self.data.insert(String::from(symbol), value);
    }                                              
                                                   
    fn find(&self, symbol: &str) -> Option<MalType> {
        if let Some(val) = self.data.get(symbol) {
            Some(val.clone())
        } else {
            match &self.outer {
                Some(o) => o.borrow().find(symbol),
                None => None
            }
        }
                                                   
    }                                              
                                                   
    pub fn get(&self, symbol: &str) -> Result<MalType, String> {
        match self.find(symbol) {
            Some(val) => Ok(val),
            _ => Err(format!("'{}' not found", symbol))
        }
    }
}
