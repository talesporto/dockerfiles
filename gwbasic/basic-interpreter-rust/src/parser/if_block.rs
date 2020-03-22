use super::*;
use std::io::BufRead;

impl<T: BufRead> Parser<T> {
    pub fn try_parse_if_block(&mut self) -> Result<Option<Statement>> {
        Ok(None)
    }
}
