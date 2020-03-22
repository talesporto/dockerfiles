use crate::common::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Variant {
    VString(String),
    VNumber(i32),
}

/// A variable context
pub struct Context {
    variable_map: HashMap<String, Variant>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            variable_map: HashMap::new(),
        }
    }

    pub fn get_variable(&self, variable_name: &String) -> Result<Variant> {
        match self.variable_map.get(variable_name) {
            Some(v) => Ok(v.clone()),
            None => Err(format!("Variable {} is not defined", variable_name)),
        }
    }

    pub fn set_variable(
        &mut self,
        variable_name: String,
        variable_value: Variant,
    ) -> Result<()> {
        self.variable_map.insert(variable_name, variable_value);
        Ok(())
    }
}
