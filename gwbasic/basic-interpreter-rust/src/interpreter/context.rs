use crate::common::Result;
use std::collections::HashMap;

/// A variable context
pub struct Context {
    variable_map: HashMap<String, String>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            variable_map: HashMap::new(),
        }
    }

    pub fn get_variable(&self, variable_name: &String) -> Result<String> {
        match self.variable_map.get(variable_name) {
            Some(v) => Ok(v.to_owned()),
            None => Err(format!("Variable {} is not defined", variable_name)),
        }
    }

    pub fn set_variable(&mut self, variable_name: String, variable_value: String) -> Result<()> {
        self.variable_map.insert(variable_name, variable_value);
        Ok(())
    }
}
