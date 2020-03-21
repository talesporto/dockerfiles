use crate::common::Result;
use crate::parser::*;
use std::io::prelude::*;

mod context;
mod stdlib;

use self::context::Context;
use self::stdlib::{DefaultStdlib, Stdlib};

pub struct Interpreter<T, S> {
    parser: Parser<T>,
    stdlib: S,
    context: Context,
}

impl<T: BufRead> Interpreter<T, DefaultStdlib> {
    pub fn new(reader: T) -> Interpreter<T, DefaultStdlib> {
        Interpreter::with_stdlib(reader, DefaultStdlib {})
    }
}

impl<T: BufRead, S: Stdlib> Interpreter<T, S> {
    pub fn with_stdlib(reader: T, stdlib: S) -> Interpreter<T, S> {
        Interpreter {
            parser: Parser::new(reader),
            stdlib,
            context: Context::new(),
        }
    }

    pub fn interpret(&mut self) -> Result<()> {
        let program = self.parser.parse()?;
        for top_level_token in program {
            if let Err(err) = self._top_level_token(top_level_token) {
                return Err(err);
            }
        }
        Ok(())
    }

    fn _top_level_token(&mut self, top_level_token: TopLevelToken) -> Result<()> {
        match top_level_token {
            TopLevelToken::Statement(statement) => self._statement(statement),
            _ => Err(format!("Unexpected top level token: {:?}", top_level_token))
        }
    }

    fn _statement(&mut self, statement: Statement) -> Result<()> {
        match statement {
            Statement::SubCall(name, args) => self._sub_call(name, args),
            _ => Err(format!("Unexpected statement: {:?}", statement))
        }
    }

    fn _sub_call(&mut self, name: String, args: Vec<Expression>) -> Result<()> {
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

    fn _do_print(&mut self, args: Vec<Expression>) -> Result<()> {
        let mut strings: Vec<String> = vec![];
        for a in args {
            strings.push(self._do_print_map_arg(a)?);
        }
        self.stdlib.print(strings);
        Ok(())
    }

    fn _do_print_map_arg(&self, arg: Expression) -> Result<String> {
        match arg {
            Expression::StringLiteral(s) => Ok(format!("{}", s)),
            Expression::VariableName(v) => self.context.get_variable(&v.name),
            _ => Err(format!("Cannot format argument {:?}", arg)),
        }
    }

    fn _do_input(&mut self, args: Vec<Expression>) -> Result<()> {
        for a in args {
            let variable_name = self._do_get_variable_name(a)?;
            let variable_value = self.stdlib.input()?;
            self.context.set_variable(variable_name, variable_value)?;
        }
        Ok(())
    }

    fn _do_get_variable_name(&self, arg: Expression) -> Result<String> {
        match arg {
            Expression::VariableName(n) => Ok(n.name),
            _ => Err(format!("Expected variable name, found {:?}", arg)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufReader, Cursor};

    pub struct MockStdlib {}

    impl Stdlib for MockStdlib {
        fn print(&self, args: Vec<String>) {
            let mut is_first = true;
            for a in args {
                if is_first {
                    is_first = false;
                } else {
                    print!(" ");
                }
                print!("{}", a)
            }

            println!("")
        }

        fn system(&self) {
            println!("would have exited")
        }

        fn input(&self) -> Result<String> {
            Ok("foo".to_string())
        }
    }

    #[test]
    fn test_interpret_print_hello_world() {
        let input = b"PRINT \"Hello, world!\"";
        let c = Cursor::new(input);
        let reader = BufReader::new(c);
        let stdlib = MockStdlib {};
        let mut interpreter = Interpreter::with_stdlib(reader, stdlib);
        interpreter.interpret().unwrap();
    }

    fn test_file(filename: &str, stdlib: MockStdlib) {
        let file_path = format!("fixtures/{}", filename);
        let file =
            File::open(file_path).expect(format!("Could not read file {}", filename).as_ref());
        let reader = BufReader::new(file);
        let mut interpreter = Interpreter::with_stdlib(reader, stdlib);
        interpreter.interpret().unwrap();
    }

    #[test]
    fn test_interpreter_fixture_hello1() {
        let stdlib = MockStdlib {};
        test_file("HELLO1.BAS", stdlib);
    }

    #[test]
    fn test_interpreter_fixture_hello2() {
        let stdlib = MockStdlib {};
        test_file("HELLO2.BAS", stdlib);
    }

    #[test]
    fn test_interpreter_fixture_hello_s() {
        let stdlib = MockStdlib {};
        test_file("HELLO_S.BAS", stdlib);
    }

    #[test]
    fn test_interpreter_for_print_10() {
        let stdlib = MockStdlib {};
        test_file("FOR_PRINT_10.BAS", stdlib);
    }

    #[test]
    fn test_interpreter_fixture_fib() {
        let stdlib = MockStdlib {};
        test_file("FIB.BAS", stdlib);
    }

    #[test]
    fn test_interpreter_fixture_input() {
        let stdlib = MockStdlib {};
        test_file("INPUT.BAS", stdlib);
    }
}
