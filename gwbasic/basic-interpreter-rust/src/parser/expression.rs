use super::{NameWithTypeQualifier, Parser, TypeQualifier};
use crate::common::Result;
use crate::lexer::Lexeme;
use std::convert::TryFrom;
use std::io::BufRead;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    StringLiteral(String),
    BinaryExpression(Box<Expression>, Box<Expression>),
    VariableName(NameWithTypeQualifier),
    IntegerLiteral(i32),
}

impl Expression {
    /// Creates a new IntegerLiteral expression
    pub fn integer_literal(value: i32) -> Expression {
        Expression::IntegerLiteral(value)
    }

    /// Creates a new VariableName expression with a qualified type
    pub fn variable_name_qualified<S: AsRef<str>>(
        name: S,
        type_qualifier: TypeQualifier,
    ) -> Expression {
        Expression::VariableName(NameWithTypeQualifier::new(name, type_qualifier))
    }

    /// Creates a new VariableName expression without a qualified type
    pub fn variable_name_unqualified<S: AsRef<str>>(name: S) -> Expression {
        Expression::VariableName(NameWithTypeQualifier::new_unqualified(name))
    }

    /// Creates a new StringLiteral expression
    pub fn string_literal<S: AsRef<str>>(literal: S) -> Expression {
        Expression::StringLiteral(literal.as_ref().to_string())
    }

    /// Returns the variable name if this expression is a VariableName,
    /// errors otherwise.
    pub fn try_to_variable_name(&self) -> Result<String> {
        match self {
            Expression::VariableName(n) => Ok(n.name()),
            _ => Err(format!("Expected variable name, was {:?}", self))
        }
    }
}

impl<T: BufRead> Parser<T> {
    pub fn demand_expression(&mut self) -> Result<Expression> {
        match self.try_parse_expression()? {
            Some(e) => Ok(e),
            None => Err(format!(
                "Expected expression, found {:?}",
                self.buf_lexer.read()?
            )),
        }
    }

    pub fn try_parse_expression(&mut self) -> Result<Option<Expression>> {
        let next = self.buf_lexer.read()?;
        match next {
            Lexeme::Symbol('"') => Ok(Some(self._read_string()?)),
            Lexeme::Word(w) => {
                self.buf_lexer.consume();
                Ok(Some(Expression::variable_name_unqualified(w)))
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
