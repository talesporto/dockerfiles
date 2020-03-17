use crate::parser::*;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};

/// The standard functions that QBasic offers
pub trait Stdlib {
    /// Implementation of PRINT x[, y, z]
    fn print(&self, args: Vec<String>);

    /// Implementation of SYSTEM
    fn system(&self);
}

pub struct DefaultStdlib {

}

impl Stdlib for DefaultStdlib {
    fn print(&self, args: Vec<String>) {
        for a in args {
            print!("{}", a)
        }

        println!("")
    }

    fn system(&self) {
        std::process::exit(0)
    }
}


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
        Interpreter { parser: Parser::new(r), stdlib }
    }

    pub fn interpret(&mut self) -> std::io::Result<()> {
        let p = self.parser.parse()?;
        match p {
            TopLevelToken::SubCall(name, args) => self._sub_call(name, args),
            _ => (),
        }
        Ok(())
    }

    fn _sub_call(&mut self, name: String, args: Vec<Expression>) {
        if name == "PRINT" {
            for a in args {
                match a {
                    Expression::StringLiteral(s) => print!("{}", s),
                    _ => (),
                }
                println!("")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use std::io::{BufReader, Cursor};


    pub struct MockStdlib {

    }

    impl Stdlib for MockStdlib {
        fn print(&self, args: Vec<String>) {
            for a in args {
                print!("{}", a)
            }

            println!("")
        }

        fn system(&self) {
        }
    }

    #[test]
    fn test_interpret_print_hello_world() {
        let input = b"PRINT \"Hello, world!\"";
        let c = Cursor::new(input);
        let reader = BufReader::new(c);
        let stdlib = MockStdlib{};
        let mut interpreter = Interpreter::with_stdlib(reader, stdlib);
        interpreter.interpret().unwrap();
    }
}
