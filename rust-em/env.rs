use types::MalType;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct MalEnv {
    outer: Option<Rc<RefCell<MalEnv>>>,
    data: HashMap<String, MalType>
}

impl MalEnv {
    pub fn new(outer: Option<Rc<RefCell<MalEnv>>>, bindings: Vec<&str>, vals: Vec<MalType>) -> Result<Rc<RefCell<MalEnv>>, String> {
        let mut env = MalEnv {
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
