use super::{Block, Parser, Expression, NameWithTypeQualifier};
use crate::common::Result;
use std::io::BufRead;

#[derive(Debug, PartialEq)]
pub struct ConditionalBlock {
    condition: Expression,
    block: Block,
}

impl ConditionalBlock {
    pub fn new(condition: Expression, block: Block) -> ConditionalBlock {
        ConditionalBlock {
            condition,
            block
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct IfBlock {
    ifBlock: ConditionalBlock,
    elseIfBlocks: Vec<ConditionalBlock>,
    elseBlock: Option<Block>
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    SubCall(String, Vec<Expression>),
    ForLoop(
        /// The counter of the loop
        NameWithTypeQualifier,
        /// The lower bound
        Expression,
        /// The upper bound
        Expression,
        /// The statements to execute
        Block,
    ),
    IfBlock(IfBlock),
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

    pub fn parse_block(&mut self) -> Result<Block> {
        let mut statements: Block = vec![];
        loop {
            self.buf_lexer.skip_whitespace_and_eol()?;
            match self.try_parse_statement()? {
                Some(s) => statements.push(s),
                None => break
            }
        }
        Ok(statements)
    }
}
