use super::{NameWithTypeQualifier, Parser, TypeQualifier};
use crate::common::Result;
use crate::lexer::Lexeme;
use std::io::BufRead;

impl<T: BufRead> Parser<T> {
    pub fn demand_name_with_type_qualifier(&mut self) -> Result<NameWithTypeQualifier> {
        let name = self.buf_lexer.demand_any_word()?;
        let type_qualifier = self._parse_type_qualifier()?;
        Ok(NameWithTypeQualifier {
            name,
            type_qualifier,
        })
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
