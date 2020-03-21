use super::{NameWithTypeQualifier, Parser, Statement, TypeQualifier};
use crate::common::Result;
use crate::lexer::Lexeme;
use std::io::BufRead;

impl<T: BufRead> Parser<T> {
    pub fn try_parse_for_loop(&mut self) -> Result<Option<Statement>> {
        if self.buf_lexer.try_consume_word("FOR")? {
            self.buf_lexer.demand_whitespace()?;
            let for_counter_variable = self.demand_name_with_type_qualifier()?;
            self.buf_lexer.skip_whitespace()?;
            self.buf_lexer.demand_symbol('=')?;
            self.buf_lexer.skip_whitespace()?;
            let lower_bound = self.demand_expression()?;
            self.buf_lexer.demand_whitespace()?;
            self.buf_lexer.demand_specific_word("TO")?;
            self.buf_lexer.demand_whitespace()?;
            let upper_bound = self.demand_expression()?;
            self.buf_lexer.skip_whitespace()?;
            self.buf_lexer.demand_eol()?;
            self.buf_lexer.skip_whitespace_and_eol()?;

            let mut statements: Vec<Statement> = vec![];

            // might have a dummy empty for loop
            while !self.buf_lexer.try_consume_word("NEXT")? {
                statements.push(self.demand_statement()?);
                self.buf_lexer.skip_whitespace_and_eol()?;
            }

            // TODO support "NEXT FOR"
            self.buf_lexer.demand_eol_or_eof()?;

            Ok(Some(Statement::ForLoop(
                for_counter_variable,
                lower_bound,
                upper_bound,
                statements
            )))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::test_utils::*;
    use crate::parser::{
        Expression, NameWithTypeQualifier, Statement, TopLevelToken, TypeQualifier,
    };

    #[test]
    fn test_for_loop() {
        let input = b"FOR I = 1 TO 10\r\nPRINT I\r\nNEXT";
        let result = parse(input).unwrap();
        assert_eq!(
            result,
            vec![TopLevelToken::Statement(Statement::ForLoop(
                NameWithTypeQualifier {
                    name: "I".to_string(),
                    type_qualifier: TypeQualifier::None
                },
                Expression::IntegerLiteral(1),
                Expression::IntegerLiteral(10),
                vec![Statement::SubCall(
                    "PRINT".to_string(),
                    vec![Expression::VariableName(NameWithTypeQualifier {
                        name: "I".to_string(),
                        type_qualifier: TypeQualifier::None
                    })]
                )]
            ))]
        );
    }
}
