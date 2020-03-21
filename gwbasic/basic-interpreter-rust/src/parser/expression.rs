use super::{Expression, NameWithTypeQualifier, Parser, TypeQualifier};
use crate::common::Result;
use crate::lexer::Lexeme;
use std::convert::TryFrom;
use std::io::BufRead;

impl<T: BufRead> Parser<T> {
    pub fn demand_expression(&mut self) -> Result<Expression> {
        match self.try_parse_expression()? {
            Some(e) => Ok(e),
            None => Err(format!("Expected expression, found {:?}", self.buf_lexer.read()?))
        }
    }

    pub fn try_parse_expression(&mut self) -> Result<Option<Expression>> {
        let next = self.buf_lexer.read()?;
        match next {
            Lexeme::Symbol('"') => Ok(Some(self._read_string()?)),
            Lexeme::Word(w) => {
                self.buf_lexer.consume();
                Ok(Some(Expression::VariableName(NameWithTypeQualifier {
                    name: w,
                    type_qualifier: TypeQualifier::None,
                })))
            }
            Lexeme::Digits(d) => {
                self.buf_lexer.consume();
                Ok(Some(Expression::IntegerLiteral(i32::try_from(d).unwrap())))
            }
            _ => Ok(None),
        }
    }

    fn _read_string(&mut self) -> Result<Expression> {
        // verify we read a double quote
        self.buf_lexer.demand_symbol('"')?;

        let mut buf: String = String::new();

        // read until we hit the next double quote
        loop {
            let l = self.buf_lexer.read()?;
            self.buf_lexer.consume();
            match l {
                Lexeme::Symbol('"') => break,
                Lexeme::EOF => return Err("EOF while looking for end of string".to_string()),
                Lexeme::EOL(_) => {
                    return Err("Unexpected new line while looking for end of string".to_string())
                }
                _ => l.push_to(&mut buf),
            }
        }

        Ok(Expression::StringLiteral(buf))
    }
}
