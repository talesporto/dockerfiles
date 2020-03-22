use super::{NameWithTypeQualifier, Parser, TypeQualifier};
use crate::common::Result;
use crate::lexer::Lexeme;
use std::convert::TryFrom;
use std::io::BufRead;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    StringLiteral(String),
    // BinaryExpression(Box<Expression>, Box<Expression>),
    VariableName(NameWithTypeQualifier),
    IntegerLiteral(i32),
    FunctionCall(String, Vec<Expression>),
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
            _ => Err(format!("Expected variable name, was {:?}", self)),
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
            Lexeme::Symbol('"') => Ok(Some(self._parse_string_literal()?)),
            Lexeme::Word(word) => Ok(Some(self._parse_word(word)?)),
            Lexeme::Digits(digits) => Ok(Some(self._parse_number_literal(digits)?)),
            _ => Ok(None),
        }
    }

    pub fn parse_expression_list(&mut self) -> Result<Vec<Expression>> {
        let mut args: Vec<Expression> = vec![];
        let optional_first_arg = self.try_parse_expression()?;
        if let Some(first_arg) = optional_first_arg {
            args.push(first_arg);
            while self._read_comma_between_arguments()? {
                self.buf_lexer.skip_whitespace()?;
                let next_arg = self.demand_expression()?;
                args.push(next_arg);
            }
        }

        Ok(args)
    }

    fn _read_comma_between_arguments(&mut self) -> Result<bool> {
        // skip whitespace after previous arg
        self.buf_lexer.skip_whitespace()?;
        self.buf_lexer.try_consume_symbol(',')
    }

    fn _parse_string_literal(&mut self) -> Result<Expression> {
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

    fn _parse_number_literal(&mut self, digits: u32) -> Result<Expression> {
        self.buf_lexer.consume();
        match i32::try_from(digits) {
            Ok(i) => Ok(Expression::IntegerLiteral(i)),
            Err(err) => Err(format!("Could not convert digits to i32: {}", err)),
        }
    }

    fn _parse_word(&mut self, word: String) -> Result<Expression> {
        self.buf_lexer.consume();
        // is it maybe a qualified variable name
        let qualifier = self.parse_type_qualifier()?;
        // it could be a function call?
        if self.buf_lexer.try_consume_symbol('(')? {
            let args = self.parse_expression_list()?;
            self.buf_lexer.demand_symbol(')')?;
            Ok(Expression::FunctionCall(word, args))
        } else {
            Ok(Expression::variable_name_qualified(word, qualifier))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_literal_expression() {
        let input = "\"hello, world\"";
        let mut parser = Parser::from(input);
        let expression = parser.demand_expression().unwrap();
        assert_eq!(expression, Expression::string_literal("hello, world"));
    }

    #[test]
    fn test_numeric_expression() {
        let input = "42";
        let mut parser = Parser::from(input);
        let expression = parser.demand_expression().unwrap();
        assert_eq!(expression, Expression::IntegerLiteral(42));
    }

    #[test]
    fn test_variable_expression() {
        let input = "A";
        let mut parser = Parser::from(input);
        let expression = parser.demand_expression().unwrap();
        assert_eq!(expression, Expression::variable_name_unqualified("A"));
    }

    #[test]
    fn test_function_call_expression_no_args() {
        let input = "IsValid()";
        let mut parser = Parser::from(input);
        let expression = parser.demand_expression().unwrap();
        assert_eq!(
            expression,
            Expression::FunctionCall("IsValid".to_string(), vec![])
        );
    }

    #[test]
    fn test_function_call_expression_one_arg() {
        let input = "IsValid(42)";
        let mut parser = Parser::from(input);
        let expression = parser.demand_expression().unwrap();
        assert_eq!(
            expression,
            Expression::FunctionCall("IsValid".to_string(), vec![Expression::IntegerLiteral(42)])
        );
    }

    #[test]
    fn test_function_call_expression_two_args() {
        let input = "CheckProperty(42, \"age\")";
        let mut parser = Parser::from(input);
        let expression = parser.demand_expression().unwrap();
        assert_eq!(
            expression,
            Expression::FunctionCall(
                "CheckProperty".to_string(),
                vec![
                    Expression::IntegerLiteral(42),
                    Expression::string_literal("age")
                ]
            )
        );
    }

    #[test]
    fn test_function_call_in_function_call() {
        let input = "CheckProperty(LookupName(\"age\"), Confirm())";
        let mut parser = Parser::from(input);
        let expression = parser.demand_expression().unwrap();
        assert_eq!(
            expression,
            Expression::FunctionCall(
                "CheckProperty".to_string(),
                vec![
                    Expression::FunctionCall(
                        "LookupName".to_string(),
                        vec![Expression::string_literal("age")]
                    ),
                    Expression::FunctionCall("Confirm".to_string(), vec![])
                ]
            )
        );
    }
}
