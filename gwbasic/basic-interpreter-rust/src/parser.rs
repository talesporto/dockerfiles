use crate::common::Result;
use crate::lexer::*;
use std::io::prelude::*;

mod declaration;
mod expression;
mod for_loop;
mod function_implementation;
mod if_block;
mod qname;
mod statement;
mod sub_call;

pub use self::expression::*;
pub use self::qname::*;
pub use self::statement::*;

pub type Block = Vec<Statement>;

#[derive(Debug, PartialEq)]
pub enum TopLevelToken {
    EOF,
    FunctionDeclaration(NameWithTypeQualifier, Vec<NameWithTypeQualifier>),
    Statement(Statement),
    FunctionImplementation(NameWithTypeQualifier, Vec<NameWithTypeQualifier>, Block),
}

impl TopLevelToken {
    pub fn sub_call<S: AsRef<str>>(name: S, args: Vec<Expression>) -> TopLevelToken {
        TopLevelToken::Statement(Statement::sub_call(name, args))
    }
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
        } else if let Some(f) = self.try_parse_function_implementation()? {
            Ok(f)
        } else if let Some(s) = self._try_parse_statement_as_top_level_token()? {
            Ok(s)
        } else {
            let lexeme = self.buf_lexer.read()?;
            match lexeme {
                Lexeme::EOF => {
                    self.buf_lexer.consume();
                    Ok(TopLevelToken::EOF)
                }
                _ => self.buf_lexer.err("[parser] Unexpected lexeme"),
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

    // bytes || &str -> Parser
    impl<T> From<T> for Parser<std::io::BufReader<std::io::Cursor<T>>>
    where
        T: std::convert::AsRef<[u8]>,
    {
        fn from(input: T) -> Self {
            Parser::new(BufReader::new(Cursor::new(input)))
        }
    }

    // File -> Parser
    impl From<File> for Parser<std::io::BufReader<File>> {
        fn from(input: File) -> Self {
            Parser::new(BufReader::new(input))
        }
    }

    pub fn parse<T>(input: T) -> Result<Program>
    where
        T: std::convert::AsRef<[u8]>,
    {
        let mut parser = Parser::from(input);
        parser.parse()
    }

    pub fn parse_file(filename: &str) -> Program {
        let file_path = format!("fixtures/{}", filename);
        let mut parser = Parser::from(File::open(file_path).expect("Could not read bas file"));
        parser.parse().expect("Could not parse program")
    }
}

#[cfg(test)]
mod tests {
    use super::test_utils::*;
    use super::*;

    #[test]
    fn test_parse_sub_call_no_args() {
        let input = "PRINT";
        let program = parse(input).unwrap();
        assert_eq!(program, vec![TopLevelToken::sub_call("PRINT", vec![])]);
    }

    #[test]
    fn test_parse_sub_call_single_arg_string_literal() {
        let input = "PRINT \"Hello, world!\"";
        let program = parse(input).unwrap();
        assert_eq!(
            program,
            vec![TopLevelToken::sub_call(
                "PRINT",
                vec![Expression::string_literal("Hello, world!")]
            )]
        );
    }

    #[test]
    fn test_parse_fixture_hello1() {
        let program = parse_file("HELLO1.BAS");
        assert_eq!(
            program,
            vec![TopLevelToken::sub_call(
                "PRINT",
                vec![Expression::string_literal("Hello, world!")]
            )]
        );
    }

    #[test]
    fn test_parse_fixture_hello2() {
        let program = parse_file("HELLO2.BAS");
        assert_eq!(
            program,
            vec![TopLevelToken::sub_call(
                "PRINT",
                vec![
                    Expression::string_literal("Hello"),
                    Expression::string_literal("world!"),
                ]
            )]
        );
    }

    #[test]
    fn test_parse_fixture_hello_system() {
        let program = parse_file("HELLO_S.BAS");
        assert_eq!(
            program,
            vec![
                TopLevelToken::sub_call(
                    "PRINT",
                    vec![Expression::string_literal("Hello, world!"),]
                ),
                TopLevelToken::sub_call("SYSTEM", vec![])
            ]
        );
    }

    #[test]
    fn test_parse_fixture_input() {
        let program = parse_file("INPUT.BAS");
        assert_eq!(
            program,
            vec![
                TopLevelToken::sub_call("INPUT", vec![Expression::variable_name_unqualified("N")]),
                TopLevelToken::sub_call("PRINT", vec![Expression::variable_name_unqualified("N")])
            ]
        );
    }

    #[test]
    fn test_parse_fixture_fib() {
        let program = parse_file("FIB.BAS");
        assert_eq!(program, vec![]);
    }
}
