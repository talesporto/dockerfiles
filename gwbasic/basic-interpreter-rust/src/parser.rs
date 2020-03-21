use crate::common::Result;
use crate::lexer::*;
use std::io::prelude::*;

mod declaration;
mod expression;
mod for_loop;
mod qname;
mod statement;
mod sub_call;

/// The optional character postfix that specifies the type of a name.
/// Example: A$ denotes a string variable
#[derive(Debug, PartialEq)]
pub enum TypeQualifier {
    None,
    BangInteger,
    DollarSignString,
}

#[derive(Debug, PartialEq)]
pub struct NameWithTypeQualifier {
    pub name: String,
    pub type_qualifier: TypeQualifier,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    StringLiteral(String),
    BinaryExpression(Box<Expression>, Box<Expression>),
    VariableName(NameWithTypeQualifier),
    IntegerLiteral(i32),
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    SubCall(String, Vec<Expression>),
    ForLoop(
        NameWithTypeQualifier,
        Expression,
        Expression,
        Vec<Statement>,
    ),
}

pub type Block = Vec<Statement>;

#[derive(Debug, PartialEq)]
pub enum TopLevelToken {
    EOF,
    FunctionDeclaration(NameWithTypeQualifier, Vec<NameWithTypeQualifier>),
    Statement(Statement),
}

pub type Program = Vec<TopLevelToken>;

pub struct Parser<T> {
    buf_lexer: BufLexer<T>,
}

impl<T: BufRead> Parser<T> {
    pub fn new(reader: T) -> Parser<T> {
        Parser {
            buf_lexer: BufLexer::new(reader),
        }
    }

    pub fn parse(&mut self) -> Result<Program> {
        let mut v: Vec<TopLevelToken> = vec![];
        loop {
            let x = self._parse_top_level_token()?;
            match x {
                TopLevelToken::EOF => break,
                _ => v.push(x),
            };
        }
        Ok(v)
    }

    fn _parse_top_level_token(&mut self) -> Result<TopLevelToken> {
        if let Some(d) = self.try_parse_declaration()? {
            Ok(d)
        } else if let Some(s) = self._try_parse_statement_as_top_level_token()? {
            Ok(s)
        } else {
            let lexeme = self.buf_lexer.read()?;
            self.buf_lexer.consume();
            match lexeme {
                Lexeme::EOF => Ok(TopLevelToken::EOF),
                _ => Err(format!("Unexpected lexeme {:?}", lexeme)),
            }
        }
    }

    fn _try_parse_statement_as_top_level_token(&mut self) -> Result<Option<TopLevelToken>> {
        match self.try_parse_statement()? {
            Some(statement) => Ok(Some(TopLevelToken::Statement(statement))),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod test_utils {
    use super::*;
    use std::fs::File;
    use std::io::{BufReader, Cursor};

    pub fn sub_call_to_top_level_token(name: &str, args: Vec<Expression>) -> TopLevelToken {
        TopLevelToken::Statement(Statement::SubCall(name.to_string(), args))
    }

    pub fn parse(input: &[u8]) -> Result<Program> {
        let c = Cursor::new(input);
        let reader = BufReader::new(c);
        let mut parser = Parser::new(reader);
        parser.parse()
    }

    pub fn parse_file(filename: &str) -> Program {
        let file_path = format!("fixtures/{}", filename);
        let reader = BufReader::new(File::open(file_path).expect("Could not read bas file"));
        let mut parser = Parser::new(reader);
        parser.parse().expect("Could not parse program")
    }

    pub fn string_literal(literal: &str) -> Expression {
        Expression::StringLiteral(literal.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::test_utils::*;
    use super::*;

    #[test]
    fn test_parse_sub_call_no_args() {
        let input = b"PRINT";
        let program = parse(input).unwrap();
        assert_eq!(program, vec![sub_call_to_top_level_token("PRINT", vec![])]);
    }

    #[test]
    fn test_parse_sub_call_single_arg_string_literal() {
        let input = b"PRINT \"Hello, world!\"";
        let program = parse(input).unwrap();
        assert_eq!(
            program,
            vec![sub_call_to_top_level_token(
                "PRINT",
                vec![string_literal("Hello, world!")]
            )]
        );
    }

    #[test]
    fn test_parse_fixture_hello1() {
        let program = parse_file("HELLO1.BAS");
        assert_eq!(
            program,
            vec![sub_call_to_top_level_token(
                "PRINT",
                vec![string_literal("Hello, world!")]
            )]
        );
    }

    #[test]
    fn test_parse_fixture_hello2() {
        let program = parse_file("HELLO2.BAS");
        assert_eq!(
            program,
            vec![sub_call_to_top_level_token(
                "PRINT",
                vec![string_literal("Hello"), string_literal("world!"),]
            )]
        );
    }

    #[test]
    fn test_parse_fixture_hello_system() {
        let program = parse_file("HELLO_S.BAS");
        assert_eq!(
            program,
            vec![
                sub_call_to_top_level_token("PRINT", vec![string_literal("Hello, world!"),]),
                sub_call_to_top_level_token("SYSTEM", vec![])
            ]
        );
    }

    #[test]
    fn test_parse_fixture_input() {
        let program = parse_file("INPUT.BAS");
        assert_eq!(
            program,
            vec![
                sub_call_to_top_level_token(
                    "INPUT",
                    vec![Expression::VariableName(NameWithTypeQualifier {
                        name: "N".to_string(),
                        type_qualifier: TypeQualifier::None
                    })]
                ),
                sub_call_to_top_level_token(
                    "PRINT",
                    vec![Expression::VariableName(NameWithTypeQualifier {
                        name: "N".to_string(),
                        type_qualifier: TypeQualifier::None
                    })]
                )
            ]
        );
    }

    #[test]
    fn test_parse_fixture_fib() {
        let program = parse_file("FIB.BAS");
        assert_eq!(program, vec![]);
    }
}
