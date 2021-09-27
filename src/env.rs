use crate::value::Value;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn default() -> Environment {
        Environment {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn new(enclosing: Environment) -> Environment {
        Environment {
            enclosing: Some(Box::new(enclosing)),
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, value: Value) {
        self.values.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.values.get(name) {
            return Some(value.clone());
        }
        self.enclosing.as_ref().and_then(|env| env.get(name))
    }
}
