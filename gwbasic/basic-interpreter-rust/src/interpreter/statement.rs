use super::Interpreter;
use super::Stdlib;
use crate::common::Result;
use crate::parser::{Block, Statement};
use std::io::BufRead;

impl<T: BufRead, S: Stdlib> Interpreter<T, S> {
    pub fn statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::SubCall(name, args) => self.sub_call(name, args),
            Statement::ForLoop(i, a, b, statements) => self.for_loop(i, a, b, statements),
            Statement::IfBlock(_) => unimplemented!(),
        }
    }

    pub fn statements(&mut self, statements: &Block) -> Result<()> {
        for statement in statements {
            match self.statement(statement) {
                Err(e) => return Err(e),
                Ok(_) => (),
            }
        }
        Ok(())
    }
}
