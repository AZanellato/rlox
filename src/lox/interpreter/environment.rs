use super::object::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct Environment {
    env_values: HashMap<String, Value>,
    parent_env: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            env_values: HashMap::new(),
            parent_env: None,
        }
    }

    pub fn enclose(&mut self, enclosing: Rc<RefCell<Environment>>) {
        self.parent_env = Some(enclosing);
    }

    pub fn assign(&mut self, name: &str, value: Value) {
        match self.env_values.get(name) {
            Some(_) => self.define(name, value),
            None => match &self.parent_env {
                Some(parent_env) => {
                    let env = &mut *parent_env.borrow_mut();
                    env.assign(name, value)
                }
                None => println!("Variable not declared with name: {}", name),
            },
        }
    }

    pub fn define(&mut self, name: &str, value: Value) {
        self.env_values.insert(name.into(), value);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        let var_in_context = self.env_values.get(name);
        if let Some(var) = var_in_context {
            Some(var.clone())
        } else {
            match &self.parent_env {
                None => None,
                Some(parent_env) => parent_env.borrow().get(name),
            }
        }
    }
}
