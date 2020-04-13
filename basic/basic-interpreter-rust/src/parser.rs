use crate::lexer::{BufLexer, LexemeNode, LexerError};
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor};

mod assignment;
mod declaration;
mod def_type;
mod expression;
mod for_loop;
mod function_implementation;
mod if_block;
mod name;
mod parse_result;
mod statement;
mod sub_call;
mod types;

#[cfg(test)]
mod test_utils;

pub use self::expression::*;
pub use self::for_loop::*;
pub use self::if_block::*;
pub use self::name::*;
pub use self::statement::*;
pub use self::types::*;

use parse_result::ParseResult;

#[derive(Debug)]
pub struct Parser<T> {
    pub buf_lexer: BufLexer<T>,
}

impl<T: BufRead> Parser<T> {
    pub fn new(buf_lexer: BufLexer<T>) -> Parser<T> {
        Parser { buf_lexer }
    }

    pub fn parse(&mut self) -> Result<ProgramNode, LexerError> {
        let mut v: Vec<TopLevelTokenNode> = vec![];
        loop {
            self.buf_lexer.skip_whitespace_and_eol()?;
            let x = self._parse_top_level_token()?;
            match x {
                Some(t) => v.push(t),
                _ => break,
            };
        }
        Ok(v)
    }

    fn _parse_top_level_token(&mut self) -> Result<Option<TopLevelTokenNode>, LexerError> {
        if let Some(d) = self.try_parse_declaration()? {
            Ok(Some(d))
        } else if let Some(f) = self.try_parse_function_implementation()? {
            Ok(Some(f))
        } else if let Some(x) = self.try_parse_def_type()? {
            Ok(Some(x))
        } else if let Some(s) = self._try_parse_statement_as_top_level_token()? {
            Ok(Some(s))
        } else {
            let lexeme = self.buf_lexer.read()?;
            match lexeme {
                LexemeNode::EOF(_) => {
                    self.buf_lexer.consume();
                    Ok(None)
                }
                _ => Err(LexerError::Unexpected(
                    format!("Unexpected top-level token"),
                    lexeme,
                )),
            }
        }
    }

    fn _try_parse_statement_as_top_level_token(
        &mut self,
    ) -> Result<Option<TopLevelTokenNode>, LexerError> {
        match self.try_parse_statement()? {
            ParseResult::Match(s) => Ok(Some(TopLevelTokenNode::Statement(s))),
            _ => Ok(None),
        }
    }
}

// bytes || &str -> Parser
impl<T> From<T> for Parser<BufReader<Cursor<T>>>
where
    T: AsRef<[u8]>,
{
    fn from(input: T) -> Self {
        Parser::new(BufLexer::from(input))
    }
}

// File -> Parser
impl From<File> for Parser<BufReader<File>> {
    fn from(input: File) -> Self {
        Parser::new(BufLexer::from(input))
    }
}

#[cfg(test)]
mod tests {
    use super::test_utils::*;
    use super::*;

    #[test]
    fn test_parse_fixture_fib() {
        let program = parse_file("FIB.BAS");
        assert_eq!(
            program,
            vec![
                // DECLARE FUNCTION Fib! (N!)
                TopLevelToken::FunctionDeclaration(Name::from("Fib!"), vec![Name::from("N!")]),
                // PRINT "Enter the number of fibonacci to calculate"
                top_sub_call(
                    "PRINT",
                    vec![Expression::from(
                        "Enter the number of fibonacci to calculate",
                    )],
                ),
                // INPUT N
                top_sub_call("INPUT", vec![Expression::variable_name("N")]),
                // FOR I = 0 TO N
                TopLevelToken::Statement(Statement::ForLoop(ForLoop {
                    variable_name: Name::from("I"),
                    lower_bound: Expression::IntegerLiteral(0),
                    upper_bound: Expression::variable_name("N"),
                    step: None,
                    statements: vec![
                        // PRINT "Fibonacci of ", I, " is ", Fib(I)
                        sub_call(
                            "PRINT",
                            vec![
                                Expression::from("Fibonacci of"),
                                Expression::variable_name("I"),
                                Expression::from("is"),
                                Expression::FunctionCall(
                                    Name::from("Fib"),
                                    vec![Expression::variable_name("I")],
                                ),
                            ],
                        ),
                    ],
                    next_counter: None,
                })),
                // FUNCTION Fib (N)
                TopLevelToken::FunctionImplementation(
                    Name::from("Fib"),
                    vec![Name::from("N")],
                    vec![
                        // IF N <= 1 THEN
                        Statement::IfBlock(new_if_else(
                            // N <= 1
                            Expression::lte(
                                Expression::variable_name("N"),
                                Expression::IntegerLiteral(1),
                            ),
                            // Fib = N
                            vec![Statement::Assignment(
                                Name::from("Fib"),
                                Expression::variable_name("N"),
                            )],
                            // ELSE Fib = Fib(N - 1) + Fib(N - 2)
                            vec![Statement::Assignment(
                                Name::from("Fib"),
                                Expression::plus(
                                    Expression::FunctionCall(
                                        Name::from("Fib"),
                                        vec![Expression::minus(
                                            Expression::variable_name("N"),
                                            Expression::IntegerLiteral(1),
                                        )],
                                    ),
                                    Expression::FunctionCall(
                                        Name::from("Fib"),
                                        vec![Expression::minus(
                                            Expression::variable_name("N"),
                                            Expression::IntegerLiteral(2),
                                        )],
                                    ),
                                ),
                            )],
                        )),
                    ],
                ),
            ],
        );
    }
}