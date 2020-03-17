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
    SubImplementation
}

pub struct Parser<T> {
    lexer: Lexer<T>,
    _last_read_lexeme: Option<Lexeme>,
}

impl<T: BufRead> Parser<T> {
    pub fn new(reader: T) -> Parser<T> {
        Parser { lexer: Lexer::new(reader), _last_read_lexeme: None }
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

    pub fn parse(&mut self) -> std::io::Result<TopLevelToken> {
        let lexeme = self._read_lexeme()?;
        self._consume_lexeme();
        match lexeme {
            Lexeme::Word(s) => self._parse_word(s),
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
                _ => break
            }
        }
        Ok(())
    }

    fn _parse_word(&mut self, name: String) -> std::io::Result<TopLevelToken> {
        let method_name = name;
        let mut args: Vec<Expression> = vec![];

        // skip whitespace
        self._skip_whitespace()?;

        if let Lexeme::Symbol('"') = self._read_lexeme()? {
            // read string
            args.push(self._read_string()?);
        }

        Ok(TopLevelToken::SubCall(method_name, args))
    }

    fn _read_string(&mut self) -> std::io::Result<Expression> {
        let mut l = self._read_lexeme()?;

        // verify we read a double quote
        match l {
            Lexeme::Symbol('"') => (),
            _ => panic!("Expected double quote")
        };

        self._consume_lexeme();

        let mut buf: String = String::new();

        // read until we hit the next double quote
        loop {
            l = self._read_lexeme()?;
            self._consume_lexeme();
            match l {
                Lexeme::Symbol('"') => break,
                Lexeme::Symbol(c) => { buf.push(c) },
                Lexeme::Whitespace(s) => { buf.push_str(&s) },
                Lexeme::Word(s) => { buf.push_str(&s) },
                _ => panic!("Unexpected lexeme")
            }
        }

        Ok(Expression::StringLiteral(buf))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufReader, Cursor};

    #[test]
    fn test_parse_sub_call_no_args() {
        let input = b"PRINT";
        let c = Cursor::new(input);
        let reader = BufReader::new(c);
        let mut parser = Parser::new(reader);
        let token = parser.parse().unwrap();
        assert_eq!(token, TopLevelToken::SubCall("PRINT".to_string(), vec![]))
    }

    #[test]
    fn test_parse_sub_call_single_arg_string_literal() {
        let input = b"PRINT \"Hello, world!\"";
        let c = Cursor::new(input);
        let reader = BufReader::new(c);
        let mut parser = Parser::new(reader);
        let token = parser.parse().unwrap();
        assert_eq!(
            token,
            TopLevelToken::SubCall(
                "PRINT".to_string(),
                vec![Expression::StringLiteral("Hello, world!".to_string())]
            )
        )
    }
}
