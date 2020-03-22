use super::context::Variant;
use super::*;
use crate::common::Result;
use crate::parser::{Block, Expression, NameWithTypeQualifier};
use std::io::BufRead;

impl<T: BufRead, S: Stdlib> Interpreter<T, S> {
    pub fn for_loop(
        &mut self,
        i: &NameWithTypeQualifier,
        a: &Expression,
        b: &Expression,
        statements: &Block,
    ) -> Result<()> {
        let mut start = self._evaluate_expression(a)?;
        let mut stop = self._evaluate_expression(b)?;
        while start <= stop {
            let counter_var_name = i.name();
            self.context
                .set_variable(counter_var_name, Variant::VNumber(start))?;
            self.statements(&statements)?;

            start += 1;
            stop = self._evaluate_expression(b)?;
        }

        Ok(())
    }

    fn _evaluate_expression(&self, e: &Expression) -> Result<i32> {
        match e {
            Expression::IntegerLiteral(i) => Ok(*i),
            _ => Err(format!("Cannot evaluate expression {:?} as an integer", e)),
        }
    }
}
