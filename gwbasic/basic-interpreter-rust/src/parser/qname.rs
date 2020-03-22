use super::Parser;
use crate::common::Result;
use crate::lexer::Lexeme;
use std::io::BufRead;

/// The optional character postfix that specifies the type of a name.
/// Example: A$ denotes a string variable
#[derive(Debug, Clone, PartialEq)]
pub enum TypeQualifier {
    None,
    BangInteger,
    DollarSignString,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NameWithTypeQualifier {
    name: String,
    type_qualifier: TypeQualifier,
}

impl NameWithTypeQualifier {
    pub fn new<S: AsRef<str>>(name: S, type_qualifier: TypeQualifier) -> NameWithTypeQualifier {
        NameWithTypeQualifier {
            name: name.as_ref().to_string(),
            type_qualifier: type_qualifier,
        }
    }

    pub fn new_unqualified<S: AsRef<str>>(name: S) -> NameWithTypeQualifier {
        NameWithTypeQualifier::new(name, TypeQualifier::None)
    }

    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    pub fn type_qualifier(&self) -> TypeQualifier {
        self.type_qualifier.clone()
    }
}

impl<T: BufRead> Parser<T> {
    pub fn demand_name_with_type_qualifier(&mut self) -> Result<NameWithTypeQualifier> {
        let name = self.buf_lexer.demand_any_word()?;
        let type_qualifier = self._parse_type_qualifier()?;
        Ok(NameWithTypeQualifier::new(name, type_qualifier))
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
