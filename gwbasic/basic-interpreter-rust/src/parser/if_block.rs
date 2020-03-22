use super::*;
use std::io::BufRead;

#[derive(Debug, PartialEq)]
pub struct ConditionalBlock {
    condition: Expression,
    block: Block,
}

impl ConditionalBlock {
    pub fn new(condition: Expression, block: Block) -> ConditionalBlock {
        ConditionalBlock { condition, block }
    }
}

#[derive(Debug, PartialEq)]
pub struct IfBlock {
    if_block: ConditionalBlock,
    else_if_blocks: Vec<ConditionalBlock>,
    else_block: Option<Block>,
}

impl<T: BufRead> Parser<T> {
    pub fn try_parse_if_block(&mut self) -> Result<Option<Statement>> {
        if self.buf_lexer.try_consume_word("IF")? {
            self.buf_lexer.demand_whitespace()?;
            let condition = self.demand_expression()?;
            self.buf_lexer.demand_whitespace()?;
            self.buf_lexer.demand_specific_word("THEN")?;
            self.buf_lexer.demand_eol()?;
            let block = self.parse_block()?;
            self.buf_lexer.demand_specific_word("END")?;
            self.buf_lexer.demand_whitespace()?;
            self.buf_lexer.demand_specific_word("IF")?;
            self.buf_lexer.demand_eol_or_eof()?;
            Ok(Some(Statement::IfBlock(IfBlock {
                if_block: ConditionalBlock::new(condition, block),
                else_if_blocks: vec![],
                else_block: None,
            })))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if() {
        let input = "IF X THEN\r\nPRINT X\r\nEND IF";
        let mut parser = Parser::from(input);
        let if_block = parser.try_parse_if_block().unwrap().unwrap();
        assert_eq!(
            if_block,
            Statement::IfBlock(IfBlock {
                if_block: ConditionalBlock::new(
                    Expression::variable_name_unqualified("X"),
                    vec![Statement::sub_call(
                        "PRINT",
                        vec![Expression::variable_name_unqualified("X")]
                    )]
                ),
                else_if_blocks: vec![],
                else_block: None
            })
        );
    }
}
