use crate::parser::*;
use crate::common::Result;
use std::io::prelude::*;

mod stdlib;

use self::stdlib::{Stdlib, DefaultStdlib};

pub struct Interpreter<T, S> {
    parser: Parser<T>,
    stdlib: S,
}

impl<T: BufRead> Interpreter<T, DefaultStdlib> {
    pub fn new(r: T) -> Interpreter<T, DefaultStdlib> {
        Interpreter::with_stdlib(r, DefaultStdlib {})
    }
}

impl<T: BufRead, S: Stdlib> Interpreter<T, S> {
    pub fn with_stdlib(r: T, stdlib: S) -> Interpreter<T, S> {
        Interpreter {
            parser: Parser::new(r),
            stdlib,
        }
    }

    pub fn interpret(&mut self) -> Result<()> {
        let p = self.parser.parse()?;
        for t in p.tokens {
            match t {
                TopLevelToken::SubCall(name, args) => self._sub_call(name, args)?,
                _ => (),
            }
        }
        Ok(())
    }

    fn _sub_call(&mut self, name: String, args: Vec<Expression>) -> Result<()> {
        if name == "PRINT" {
            let mut strings: Vec<String> = vec![];
            for a in args {
                match a {
                    Expression::StringLiteral(s) => strings.push(format!("{}", s)),
                    _ => (),
                }
            }
            self.stdlib.print(strings);
            Ok(())
        } else if name == "INPUT" {
            unimplemented!();
            Ok(())
        } else if name == "SYSTEM" {
            self.stdlib.system();
            Ok(())
        } else {
            Err(format!("Unknown sub {}", name))
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
            for a in args {
                print!("{}", a)
            }

            println!("")
        }

        fn system(&self) {
            println!("would have exited")
        }

        fn input(&self, args: Vec<String>) {
            unimplemented!();
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
