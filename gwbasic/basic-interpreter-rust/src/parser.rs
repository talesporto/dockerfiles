use crate::lexer::*;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};

#[derive(Debug, PartialEq)]
pub enum Expression {
    StringLiteral(String),
    BinaryExpression(Box<Expression>, Box<Expression>),
}

#[derive(Debug, PartialEq)]
pub enum TopLevelToken {
    EOF,
    SubCall(String, Vec<Expression>),
    SubDeclaration,
    SubImplementation,
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub tokens: Vec<TopLevelToken>,
}

pub struct Parser<T> {
    lexer: Lexer<T>,
    _last_read_lexeme: Option<Lexeme>,
}

impl<T: BufRead> Parser<T> {
    pub fn new(reader: T) -> Parser<T> {
        Parser {
            lexer: Lexer::new(reader),
            _last_read_lexeme: None,
        }
    }

    /// Reads the next lexeme.
    /// The lexeme is stored and no further reads will be done unless
    /// _consume_lexeme is called.
    fn _read_lexeme(&mut self) -> std::io::Result<Lexeme> {
        match self._last_read_lexeme.clone() {
            Some(x) => Ok(x),
            None => {
                let new_lexeme = self.lexer.read()?;
                self._last_read_lexeme = Some(new_lexeme.clone());
                Ok(new_lexeme)
            }
        }
    }

    /// Consumes the previously read lexeme, allowing further reads.
    fn _consume_lexeme(&mut self) {
        self._last_read_lexeme = match self._last_read_lexeme {
            None => panic!("No previously read lexeme!"),
            Some(_) => None,
        }
    }

    pub fn parse(&mut self) -> std::io::Result<Program> {
        let mut v: Vec<TopLevelToken> = vec![];
        loop {
            let x = self._parse_one()?;
            match x {
                TopLevelToken::EOF => break,
                _ => v.push(x),
            };
        }
        Ok(Program { tokens: v })
    }

    fn _parse_one(&mut self) -> std::io::Result<TopLevelToken> {
        let lexeme = self._read_lexeme()?;
        self._consume_lexeme();
        match lexeme {
            Lexeme::Word(s) => self._parse_sub_call(s),
            Lexeme::EOF => Ok(TopLevelToken::EOF),
            Lexeme::Unknown(ch) => Err(Error::new(
                ErrorKind::Other,
                format!("Unknown character {}", ch),
            )),
            _ => Err(Error::new(ErrorKind::Other, "Unexpected lexeme")),
        }
    }

    fn _skip_whitespace(&mut self) -> std::io::Result<()> {
        loop {
            let lexeme = self._read_lexeme()?;
            match lexeme {
                Lexeme::Whitespace(_) => self._consume_lexeme(),
                _ => break,
            }
        }
        Ok(())
    }

    fn _skip_whitespace_and_new_lines(&mut self) -> std::io::Result<()> {
        loop {
            let lexeme = self._read_lexeme()?;
            match lexeme {
                Lexeme::Whitespace(_) | Lexeme::CRLF | Lexeme::CR | Lexeme::LF => {
                    self._consume_lexeme()
                }
                _ => break,
            }
        }
        Ok(())
    }

    fn _parse_sub_call(&mut self, name: String) -> std::io::Result<TopLevelToken> {
        let method_name = name;
        let mut args: Vec<Expression> = vec![];

        let optional_first_arg = self._read_argument()?;

        if let Some(first_arg) = optional_first_arg {
            args.push(first_arg);
            while self._read_comma_between_arguments()? {
                let optional_next_arg = self._read_argument()?;
                match optional_next_arg {
                    Some(next_arg) => args.push(next_arg),
                    None => panic!("Trailing comma")
                }
            }
        }

        self._skip_whitespace_and_new_lines()?;

        Ok(TopLevelToken::SubCall(method_name, args))
    }

    fn _read_argument(
        &mut self
    ) -> std::io::Result<Option<Expression>> {
        // skip whitespace after method name or after comma
        self._skip_whitespace()?;
        let next = self._read_lexeme()?;
        match next {
            Lexeme::Symbol('"') => {
                Ok(Some(self._read_string()?))
            },
            Lexeme::CRLF | Lexeme::CR | Lexeme::LF | Lexeme::EOF => Ok(None),
            _ => panic!("Expected argument or end of line, found {:#?}", next),
        }
    }

    fn _read_comma_between_arguments(
        &mut self
    ) -> std::io::Result<bool> {
        // skip whitespace after previous arg
        self._skip_whitespace()?;
        let next = self._read_lexeme()?;
        match next {
            Lexeme::Symbol(',') => {
                self._consume_lexeme();
                Ok(true)
            },
            Lexeme::CRLF | Lexeme::CR | Lexeme::LF | Lexeme::EOF => Ok(false),
            _ => panic!("Expected comma or end of line, found {:#?}", next),
        }
    }

    fn _read_string(&mut self) -> std::io::Result<Expression> {
        let mut l = self._read_lexeme()?;

        // verify we read a double quote
        match l {
            Lexeme::Symbol('"') => (),
            _ => panic!("Expected double quote"),
        };

        self._consume_lexeme();

        let mut buf: String = String::new();

        // read until we hit the next double quote
        loop {
            l = self._read_lexeme()?;
            self._consume_lexeme();
            match l {
                Lexeme::Symbol('"') => break,
                Lexeme::EOF => panic!("EOF while looking for end of string"),
                Lexeme::CRLF | Lexeme::CR | Lexeme::LF => {
                    panic!("Unexpected new line while looking for end of string")
                }
                _ => l.push_to(&mut buf),
            }
        }

        Ok(Expression::StringLiteral(buf))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufReader, Cursor};

    #[test]
    fn test_parse_sub_call_no_args() {
        let input = b"PRINT";
        let c = Cursor::new(input);
        let reader = BufReader::new(c);
        let mut parser = Parser::new(reader);
        let program = parser.parse().unwrap();
        assert_eq!(
            program,
            Program {
                tokens: vec![TopLevelToken::SubCall("PRINT".to_string(), vec![])]
            }
        );
    }

    #[test]
    fn test_parse_sub_call_single_arg_string_literal() {
        let input = b"PRINT \"Hello, world!\"";
        let c = Cursor::new(input);
        let reader = BufReader::new(c);
        let mut parser = Parser::new(reader);
        let program = parser.parse().unwrap();
        assert_eq!(
            program,
            Program {
                tokens: vec![TopLevelToken::SubCall(
                    "PRINT".to_string(),
                    vec![Expression::StringLiteral("Hello, world!".to_string())]
                )]
            }
        );
    }

    #[test]
    fn test_parse_fixture_hello1() {
        let reader =
            BufReader::new(File::open("fixtures/HELLO1.BAS").expect("Could not read bas file"));
        let mut parser = Parser::new(reader);
        let program = parser.parse().unwrap();
        assert_eq!(
            program,
            Program {
                tokens: vec![TopLevelToken::SubCall(
                    "PRINT".to_string(),
                    vec![Expression::StringLiteral("Hello, world!".to_string())]
                )]
            }
        );
    }

    #[test]
    fn test_parse_fixture_hello2() {
        let reader =
            BufReader::new(File::open("fixtures/HELLO2.BAS").expect("Could not read bas file"));
        let mut parser = Parser::new(reader);
        let program = parser.parse().unwrap();
        assert_eq!(
            program,
            Program {
                tokens: vec![TopLevelToken::SubCall(
                    "PRINT".to_string(),
                    vec![
                        Expression::StringLiteral("Hello".to_string()),
                        Expression::StringLiteral("world!".to_string())
                    ]
                )]
            }
        );
    }

    #[test]
    fn test_parse_fixture_hello_system() {
        let reader =
            BufReader::new(File::open("fixtures/HELLO_S.BAS").expect("Could not read bas file"));
        let mut parser = Parser::new(reader);
        let program = parser.parse().unwrap();
        assert_eq!(
            program,
            Program {
                tokens: vec![
                    TopLevelToken::SubCall(
                        "PRINT".to_string(),
                        vec![Expression::StringLiteral("Hello, world!".to_string())]
                    ),
                    TopLevelToken::SubCall("SYSTEM".to_string(), vec![])
                ]
            }
        );
    }
}
