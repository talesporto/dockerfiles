use super::{Expression, Parser, Statement};
use crate::common::Result;
use crate::lexer::Lexeme;
use std::io::BufRead;

fn _is_allowed_sub_name(word: String) -> bool {
    word != "NEXT"
}

impl<T: BufRead> Parser<T> {
    pub fn try_parse_sub_call(&mut self) -> Result<Option<Statement>> {
        match self.buf_lexer.read()? {
            Lexeme::Word(w) => {
                if _is_allowed_sub_name(w) {
                    Ok(Some(self._parse_sub_call()?))
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
        }
    }

    fn _parse_sub_call(&mut self) -> Result<Statement> {
        let method_name = self.buf_lexer.demand_any_word()?;
        let mut args: Vec<Expression> = vec![];
        self.buf_lexer.skip_whitespace()?;
        let optional_first_arg = self.try_parse_expression()?;

        if let Some(first_arg) = optional_first_arg {
            args.push(first_arg);
            while self._read_comma_between_arguments()? {
                self.buf_lexer.skip_whitespace()?;
                let next_arg = self.demand_expression()?;
                args.push(next_arg);
            }
        }

        self.buf_lexer.demand_eol_or_eof()?;

        Ok(Statement::SubCall(method_name, args))
    }

    fn _read_comma_between_arguments(&mut self) -> Result<bool> {
        // skip whitespace after previous arg
        self.buf_lexer.skip_whitespace()?;
        let next = self.buf_lexer.read()?;
        match next {
            Lexeme::Symbol(',') => {
                self.buf_lexer.consume();
                Ok(true)
            }
            Lexeme::EOL(_) | Lexeme::EOF => Ok(false),
            _ => Err(format!("Expected comma or end of line, found {:?}", next)),
        }
    }
}
