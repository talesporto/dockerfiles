use crate::common::Result;
use crate::lexer::*;
use std::io::prelude::*;

mod declaration;

#[derive(Debug, PartialEq)]
pub enum TypeQualifier {
    None,
    BangInteger,
    DollarSignString
}

#[derive(Debug, PartialEq)]
pub struct NameWithTypeQualifier {
    pub name: String,
    pub type_qualifier: TypeQualifier,
}

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
    FunctionDeclaration(NameWithTypeQualifier, Vec<NameWithTypeQualifier>),
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub tokens: Vec<TopLevelToken>,
}

pub struct Parser<T> {
    buf_lexer: BufLexer<T>,
}

impl<T: BufRead> Parser<T> {
    pub fn new(reader: T) -> Parser<T> {
        Parser {
            buf_lexer: BufLexer::new(reader),
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
        match self.try_parse_declaration()? {
            Some(d) => Ok(d),
            None => {
                let lexeme = self.buf_lexer.read()?;
                self.buf_lexer.consume();
                match lexeme {
                    Lexeme::Word(s) => self._parse_sub_call(s),
                    Lexeme::EOF => Ok(TopLevelToken::EOF),
                    _ => Err(format!("Unexpected lexeme {:?}", lexeme)),
                }
            }
        }
    }

    fn _skip_whitespace_and_new_lines(&mut self) -> Result<()> {
        loop {
            let lexeme = self.buf_lexer.read()?;
            match lexeme {
                Lexeme::Whitespace(_) | Lexeme::CRLF | Lexeme::CR | Lexeme::LF => {
                    self.buf_lexer.consume()
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
                    None => return Err("Trailing comma".to_string()),
                }
            }
        }

        self._skip_whitespace_and_new_lines()?;

        Ok(TopLevelToken::SubCall(method_name, args))
    }

    fn _read_argument(&mut self) -> Result<Option<Expression>> {
        // skip whitespace after method name or after comma
        self.buf_lexer.skip_whitespace()?;
        let next = self.buf_lexer.read()?;
        match next {
            Lexeme::Symbol('"') => Ok(Some(self._read_string()?)),
            Lexeme::Word(w) => {
                self.buf_lexer.consume();
                Ok(Some(Expression::VariableName(w)))
            }
            Lexeme::CRLF | Lexeme::CR | Lexeme::LF | Lexeme::EOF => Ok(None),
            _ => Err(format!(
                "Expected argument or end of line, found {:?}",
                next
            )),
        }
    }

    fn _read_comma_between_arguments(&mut self) -> Result<bool> {
        // skip whitespace after previous arg
        self.buf_lexer.skip_whitespace()?;
        let next = self.buf_lexer.read()?;
        match next {
            Lexeme::Symbol(',') => {
                self.buf_lexer.consume();
                Ok(true)
            }
            Lexeme::CRLF | Lexeme::CR | Lexeme::LF | Lexeme::EOF => Ok(false),
            _ => Err(format!("Expected comma or end of line, found {:?}", next)),
        }
    }

    fn _assert_symbol(&mut self, ch: char) -> Result<()> {
        let l = self.buf_lexer.read()?;

        // verify we read a double quote
        match l {
            Lexeme::Symbol(_ch) => {
                if _ch == ch {
                    self.buf_lexer.consume();
                    Ok(())
                } else {
                    Err(format!("Expected {}, found {:?}", ch, l))
                }
            }
            _ => Err(format!("Expected {}, found {:?}", ch, l)),
        }
    }

    fn _read_string(&mut self) -> Result<Expression> {
        // verify we read a double quote
        self._assert_symbol('"')?;

        let mut buf: String = String::new();

        // read until we hit the next double quote
        loop {
            let l = self.buf_lexer.read()?;
            self.buf_lexer.consume();
            match l {
                Lexeme::Symbol('"') => break,
                Lexeme::EOF => return Err("EOF while looking for end of string".to_string()),
                Lexeme::CRLF | Lexeme::CR | Lexeme::LF => {
                    return Err("Unexpected new line while looking for end of string".to_string())
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

    fn parse_file(filename: &str) -> Program {
        let file_path = format!("fixtures/{}", filename);
        let reader = BufReader::new(File::open(file_path).expect("Could not read bas file"));
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
                tokens: vec![
                    TopLevelToken::SubCall(
                        "INPUT".to_string(),
                        vec![Expression::VariableName("N".to_string())]
                    ),
                    TopLevelToken::SubCall(
                        "PRINT".to_string(),
                        vec![Expression::VariableName("N".to_string())]
                    )
                ]
            }
        );
    }
}
