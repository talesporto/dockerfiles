use super::{Block, Parser, Expression, NameWithTypeQualifier};
use crate::common::Result;
use std::io::BufRead;

#[derive(Debug, PartialEq)]
pub enum Statement {
    SubCall(String, Vec<Expression>),
    ForLoop(
        NameWithTypeQualifier,
        Expression,
        Expression,
        Block,
    ),
}

impl Statement {
    pub fn sub_call<S: AsRef<str>>(name: S, args: Vec<Expression>) -> Statement {
        Statement::SubCall(name.as_ref().to_string(), args)
    }
}

impl<T: BufRead> Parser<T> {
    pub fn demand_statement(&mut self) -> Result<Statement> {
        match self.try_parse_statement() {
            Ok(Some(x)) => Ok(x),
            Ok(None) => Err(format!("Expected statement, found {:?}", self.buf_lexer.read()?)),
            Err(e) => Err(e)
        }
    }

    pub fn try_parse_statement(&mut self) -> Result<Option<Statement>> {
        if let Some(f) = self.try_parse_for_loop()? {
            Ok(Some(f))
        } else if let Some(s) = self.try_parse_sub_call()? {
            Ok(Some(s))
        } else {
            Ok(None)
        }
    }
}
