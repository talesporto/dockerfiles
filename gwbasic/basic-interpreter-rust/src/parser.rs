use crate::common::Result;
use crate::lexer::*;
use std::io::prelude::*;

#[derive(Debug, PartialEq)]
pub enum Expression {
    StringLiteral(String),
    BinaryExpression(Box<Expression>, Box<Expression>),
    VariableName(String),
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
    fn _read_lexeme(&mut self) -> LexerResult {
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

    pub fn parse(&mut self) -> Result<Program> {
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

    fn _parse_one(&mut self) -> Result<TopLevelToken> {
        let lexeme = self._read_lexeme()?;
        self._consume_lexeme();
        match lexeme {
            Lexeme::Word(s) => self._parse_sub_call(s),
            Lexeme::EOF => Ok(TopLevelToken::EOF),
            _ => Err("Unexpected lexeme".to_string()),
        }
    }

    fn _skip_whitespace(&mut self) -> Result<()> {
        loop {
            let lexeme = self._read_lexeme()?;
            match lexeme {
                Lexeme::Whitespace(_) => self._consume_lexeme(),
                _ => break,
            }
        }
        Ok(())
    }

    fn _skip_whitespace_and_new_lines(&mut self) -> Result<()> {
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

    fn _parse_sub_call(&mut self, name: String) -> Result<TopLevelToken> {
        let method_name = name;
        let mut args: Vec<Expression> = vec![];

        let optional_first_arg = self._read_argument()?;

        if let Some(first_arg) = optional_first_arg {
            args.push(first_arg);
            while self._read_comma_between_arguments()? {
                let optional_next_arg = self._read_argument()?;
                match optional_next_arg {
                    Some(next_arg) => args.push(next_arg),
                    None => return Err("Trailing comma".to_string())
                }
            }
        }

        self._skip_whitespace_and_new_lines()?;

        Ok(TopLevelToken::SubCall(method_name, args))
    }

    fn _read_argument(
        &mut self
    ) -> Result<Option<Expression>> {
        // skip whitespace after method name or after comma
        self._skip_whitespace()?;
        let next = self._read_lexeme()?;
        match next {
            Lexeme::Symbol('"') => {
                Ok(Some(self._read_string()?))
            },
            Lexeme::Word(w) => {
                self._consume_lexeme();
                Ok(Some(Expression::VariableName(w)))
            },
            Lexeme::CRLF | Lexeme::CR | Lexeme::LF | Lexeme::EOF => Ok(None),
            _ => Err(format!("Expected argument or end of line, found {:?}", next)),
        }
    }

    fn _read_comma_between_arguments(
        &mut self
    ) -> Result<bool> {
        // skip whitespace after previous arg
        self._skip_whitespace()?;
        let next = self._read_lexeme()?;
        match next {
            Lexeme::Symbol(',') => {
                self._consume_lexeme();
                Ok(true)
            },
            Lexeme::CRLF | Lexeme::CR | Lexeme::LF | Lexeme::EOF => Ok(false),
            _ => Err(format!("Expected comma or end of line, found {:?}", next)),
        }
    }

    fn _assert_symbol(&mut self, ch: char) -> Result<()> {
        let l = self._read_lexeme()?;

        // verify we read a double quote
        match l {
            Lexeme::Symbol(_ch) => {
                if _ch == ch {
                    self._consume_lexeme();
                    Ok(())
                } else {
                    Err(format!("Expected {}, found {:?}", ch, l))
                }
            },
            _ => Err(format!("Expected {}, found {:?}", ch, l))
        }
    }

    fn _read_string(&mut self) -> Result<Expression> {
        // verify we read a double quote
        self._assert_symbol('"')?;

        let mut buf: String = String::new();

        // read until we hit the next double quote
        loop {
            let l = self._read_lexeme()?;
            self._consume_lexeme();
            match l {
                Lexeme::Symbol('"') => break,
                Lexeme::EOF => return Err("EOF while looking for end of string".to_string()),
                Lexeme::CRLF | Lexeme::CR | Lexeme::LF =>
                    return Err("Unexpected new line while looking for end of string".to_string()),
                _ => l.push_to(&mut buf)
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

    fn parse_file(filename: &str) -> Program {
        let file_path = format!("fixtures/{}", filename);
        let reader =
            BufReader::new(File::open(file_path).expect("Could not read bas file"));
        let mut parser = Parser::new(reader);
        parser.parse().expect("Could not parse program")
    }

    #[test]
    fn test_parse_fixture_hello1() {
        let program = parse_file("HELLO1.BAS");
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
        let program = parse_file("HELLO2.BAS");
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
        let program = parse_file("HELLO_S.BAS");
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

    #[test]
    fn test_parse_fixture_fib() {
        let program = parse_file("FIB.BAS");
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
    fn test_parse_fixture_input() {
        let program = parse_file("INPUT.BAS");
        assert_eq!(
            program,
            Program {
                tokens: vec![TopLevelToken::SubCall(
                    "INPUT".to_string(),
                    vec![Expression::VariableName("N".to_string())]
                ),
                TopLevelToken::SubCall(
                    "PRINT".to_string(),
                    vec![Expression::VariableName("N".to_string())]
                )]
            }
        );
    }
}
