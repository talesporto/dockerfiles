use super::{NameWithTypeQualifier, Parser, TopLevelToken, TypeQualifier};
use crate::common::Result;
use crate::lexer::Lexeme;
use std::io::BufRead;

impl<T: BufRead> Parser<T> {
    pub fn try_parse_declaration(&mut self) -> Result<Option<TopLevelToken>> {
        if self.buf_lexer.try_consume_word("DECLARE")? {
            match self._parse_declaration() {
                Ok(d) => Ok(Some(d)),
                Err(x) => Err(x),
            }
        } else {
            Ok(None)
        }
    }

    fn _demand_name_with_type_qualifier(&mut self) -> Result<NameWithTypeQualifier> {
        let name = self.buf_lexer.demand_word()?;
        let type_qualifier = self._parse_type_qualifier()?;
        Ok(NameWithTypeQualifier {
            name,
            type_qualifier,
        })
    }

    fn _parse_declaration(&mut self) -> Result<TopLevelToken> {
        self.buf_lexer.skip_whitespace()?;
        let next_word = self.buf_lexer.demand_word()?;
        if next_word == "FUNCTION" {
            self.buf_lexer.skip_whitespace()?;
            let function_name = self._demand_name_with_type_qualifier()?;
            let mut function_arguments: Vec<NameWithTypeQualifier> = vec![];
            self.buf_lexer.skip_whitespace()?;
            if self.buf_lexer.try_consume_symbol('(')? {
                self.buf_lexer.skip_whitespace()?;
                let mut is_first_parameter = true;
                while !self.buf_lexer.try_consume_symbol(')')? {
                    if is_first_parameter {
                        is_first_parameter = false;
                    } else {
                        self.buf_lexer.demand_symbol(',')?;
                        self.buf_lexer.skip_whitespace()?;
                    }
                    function_arguments.push(self._demand_name_with_type_qualifier()?);
                    self.buf_lexer.skip_whitespace()?;
                }
            }

            self.buf_lexer.demand_eol_or_eof()?;
            Ok(TopLevelToken::FunctionDeclaration(
                function_name,
                function_arguments,
            ))
        } else {
            Err(format!("Unknown declaration: {}", next_word))
        }
    }

    fn _parse_type_qualifier(&mut self) -> Result<TypeQualifier> {
        let next = self.buf_lexer.read()?;
        match next {
            Lexeme::Symbol(ch) => {
                if ch == '!' {
                    self.buf_lexer.consume();
                    Ok(TypeQualifier::BangInteger)
                } else {
                    Ok(TypeQualifier::None)
                }
            }
            _ => Ok(TypeQualifier::None),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{NameWithTypeQualifier, Parser, Program, TopLevelToken, TypeQualifier};
    use std::io::{BufReader, Cursor};

    #[test]
    fn test_fn() {
        let input = b"DECLARE FUNCTION Fib! (N!)";
        let c = Cursor::new(input);
        let reader = BufReader::new(c);
        let mut parser = Parser::new(reader);
        let result = parser.parse().unwrap();
        assert_eq!(
            result,
            Program {
                tokens: vec![TopLevelToken::FunctionDeclaration(
                    NameWithTypeQualifier {
                        name: "Fib".to_string(),
                        type_qualifier: TypeQualifier::BangInteger
                    },
                    vec![NameWithTypeQualifier {
                        name: "N".to_string(),
                        type_qualifier: TypeQualifier::BangInteger
                    }]
                )]
            }
        )
    }
}
