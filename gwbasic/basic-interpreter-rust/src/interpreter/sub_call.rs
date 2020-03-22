use super::Interpreter;
use super::Stdlib;
use super::context::Variant;
use crate::common::Result;
use crate::parser::Expression;
use std::io::BufRead;

impl<T: BufRead, TStdlib: Stdlib> Interpreter<T, TStdlib> {
    pub fn sub_call(&mut self, name: &String, args: &Vec<Expression>) -> Result<()> {
        if name == "PRINT" {
            self._do_print(args)
        } else if name == "INPUT" {
            self._do_input(args)
        } else if name == "SYSTEM" {
            self.stdlib.system();
            Ok(())
        } else {
            Err(format!("Unknown sub {}", name))
        }
    }

    fn _do_print(&mut self, args: &Vec<Expression>) -> Result<()> {
        let mut strings: Vec<String> = vec![];
        for a in args {
            strings.push(self._do_print_map_arg(a)?);
        }
        self.stdlib.print(strings);
        Ok(())
    }

    fn _do_print_map_arg(&self, arg: &Expression) -> Result<String> {
        match arg {
            Expression::StringLiteral(s) => Ok(format!("{}", s)),
            Expression::VariableName(v) => {
                let var_name = v.name();
                self._do_print_variant(self.context.get_variable(&var_name)?)
            }
            _ => Err(format!("Cannot format argument {:?}", arg)),
        }
    }

    fn _do_print_variant(&self, v: Variant) -> Result<String> {
        match v {
            Variant::VString(s) => Ok(s),
            Variant::VNumber(n) => Ok(format!("{}", n)),
        }
    }

    fn _do_input(&mut self, args: &Vec<Expression>) -> Result<()> {
        for a in args {
            let variable_name = a.try_to_variable_name()?;
            let variable_value = self.stdlib.input()?;
            self.context.set_variable(variable_name, Variant::VString(variable_value))?;
        }
        Ok(())
    }
}
