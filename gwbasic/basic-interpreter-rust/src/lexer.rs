use std::io::prelude::*;
use std::io::{Error, ErrorKind};

#[derive(Debug, PartialEq, Copy, Clone)]
enum CharOrEof {
    EOF,

    Char(char),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Lexeme {
    /// Unknown lexeme
    Unknown(char),

    /// EOF
    EOF,

    /// CRLF
    CRLF,

    /// CR, not followed by LF
    CR,

    /// LF, not preceded by CR
    LF,

    /// A sequence of letters (A-Z or a-z)
    Word(String),

    /// A sequence of whitespace (spaces and tabs)
    Whitespace(String),

    /// A punctuation symbol
    Symbol(char),

    /// An integer number
    Number(u32),
}

pub struct Lexer<T> {
    reader: T,
    _buffer: Vec<CharOrEof>,
}

fn _is_letter(ch: char) -> bool {
    (ch >= 'A' && ch <= 'Z') || (ch >= 'a' && ch <= 'z')
}

fn _is_number(ch: char) -> bool {
    ch >= '0' && ch <= '9'
}

fn _is_whitespace(ch: char) -> bool {
    ch == ' ' || ch == '\t'
}

fn _is_symbol(ch: char) -> bool {
    ch == '"'
        || ch == '\''
        || ch == '!'
        || ch == ','
        || ch == '$'
        || ch == '%'
        || ch == '+'
        || ch == '-'
        || ch == '*'
        || ch == '/'
}

impl<T: BufRead> Lexer<T> {
    pub fn new(reader: T) -> Lexer<T> {
        Lexer {
            reader,
            _buffer: vec![],
        }
    }

    /// Peeks the next available char or EOF.
    fn _peek_char_or_eof(&mut self) -> std::io::Result<CharOrEof> {
        self._fill_buffer_if_empty()?;
        let first = self._buffer[0];
        Ok(first)
    }

    fn _peek_char_predicate(&mut self, predicate: fn(char) -> bool) -> std::io::Result<bool> {
        let result = self._peek_char_or_eof()?;
        match result {
            CharOrEof::Char(ch) => Ok(predicate(ch)),
            CharOrEof::EOF => Ok(false),
        }
    }

    /// Reads the next available char or EOF.
    fn _read_char_or_eof(&mut self) -> std::io::Result<CharOrEof> {
        let result = self._peek_char_or_eof()?;
        self._buffer.remove(0);
        Ok(result)
    }

    /// Reads the next available char, errors if EOF.
    fn _read_char(&mut self) -> std::io::Result<char> {
        let result = self._read_char_or_eof()?;
        match result {
            CharOrEof::Char(ch) => Ok(ch),
            CharOrEof::EOF => Err(Error::new(ErrorKind::UnexpectedEof, "Unexpected EOF")),
        }
    }

    fn _fill_buffer_if_empty(&mut self) -> std::io::Result<()> {
        if self._buffer.is_empty() {
            let mut line = String::new();
            let bytes_read: usize = self.reader.read_line(&mut line)?;
            if bytes_read == 0 {
                self._buffer.push(CharOrEof::EOF);
            } else {
                for ch in line.chars() {
                    self._buffer.push(CharOrEof::Char(ch));
                }
                if self._buffer.is_empty() {
                    panic!("Should have found at least one character")
                }
            }
        }
        Ok(())
    }

    fn _read_while(&mut self, predicate: fn(char) -> bool) -> std::io::Result<String> {
        let mut result: String = String::new();
        while self._peek_char_predicate(predicate)? {
            result.push(self._read_char()?);
        }
        Ok(result)
    }

    pub fn read(&mut self) -> std::io::Result<Lexeme> {
        let x = self._peek_char_or_eof()?;
        Ok(match x {
            CharOrEof::EOF => Lexeme::EOF,
            CharOrEof::Char(ch) => {
                if _is_letter(ch) {
                    Lexeme::Word(self._read_while(_is_letter)?)
                } else if _is_whitespace(ch) {
                    Lexeme::Whitespace(self._read_while(_is_whitespace)?)
                } else if _is_symbol(ch) {
                    Lexeme::Symbol(self._read_char()?)
                } else {
                    Lexeme::Unknown(ch)
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufReader, Cursor};

    #[test]
    fn test_lexer() {
        let input = b"PRINT \"Hello, world!\"";
        let c = Cursor::new(input);
        let reader = BufReader::new(c);
        let mut lexer = Lexer::new(reader);
        assert_eq!(lexer.read().unwrap(), Lexeme::Word("PRINT".to_string()));
        assert_eq!(lexer.read().unwrap(), Lexeme::Whitespace(" ".to_string()));
        assert_eq!(lexer.read().unwrap(), Lexeme::Symbol('"'));
        assert_eq!(lexer.read().unwrap(), Lexeme::Word("Hello".to_string()));
        assert_eq!(lexer.read().unwrap(), Lexeme::Symbol(','));
        assert_eq!(lexer.read().unwrap(), Lexeme::Whitespace(" ".to_string()));
        assert_eq!(lexer.read().unwrap(), Lexeme::Word("world".to_string()));
        assert_eq!(lexer.read().unwrap(), Lexeme::Symbol('!'));
        assert_eq!(lexer.read().unwrap(), Lexeme::Symbol('"'));
        assert_eq!(lexer.read().unwrap(), Lexeme::EOF);
    }
}
