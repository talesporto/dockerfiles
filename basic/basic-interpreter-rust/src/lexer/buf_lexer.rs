use super::{LexemeNode, Lexer, LexerError};
use crate::common::*;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor};

#[derive(Debug)]
pub struct BufLexer<T> {
    lexer: Lexer<T>,
    _history: Vec<LexemeNode>,
    _index: usize,
    _mark_index: Option<usize>,
}

impl<T: BufRead> BufLexer<T> {
    pub fn new(lexer: Lexer<T>) -> BufLexer<T> {
        BufLexer {
            lexer: lexer,
            _history: vec![],
            _index: 0,
            _mark_index: None,
        }
    }

    /// Reads the next lexeme.
    /// The lexeme is stored and no further reads will be done unless
    /// consume is called.
    pub fn read(&mut self) -> Result<LexemeNode, LexerError> {
        if self.needs_to_read() {
            self._history.push(self.lexer.read()?);
        }
        Ok(self._history[self._index].clone())
    }

    pub fn read_ref(&mut self) -> Result<&LexemeNode, LexerError> {
        if self.needs_to_read() {
            self._history.push(self.lexer.read()?);
        }
        Ok(&self._history[self._index])
    }

    fn needs_to_read(&self) -> bool {
        self._index >= self._history.len()
    }

    /// Consumes the previously read lexeme, allowing further reads.
    pub fn consume(&mut self) {
        if self._history.is_empty() {
            panic!("No previously read lexeme!");
        } else {
            self._index += 1;
        }
    }

    pub fn mark(&mut self) {
        match self._mark_index {
            Some(_) => panic!("Mark called without backtrack or clear!"),
            None => self._mark_index = Some(self._index),
        }
    }

    pub fn backtrack(&mut self) {
        self._index = self._mark_index.take().unwrap();
    }

    pub fn clear(&mut self) {
        while self._index > 0 {
            self._history.remove(0);
            self._index -= 1;
        }
        self._mark_index = None;
    }

    /// Tries to read the given word. If the next lexeme is this particular word,
    /// it consumes it and it returns true.
    pub fn try_consume_word(&mut self, word: &str) -> Result<Option<Location>, LexerError> {
        let lexeme = self.read()?;
        match lexeme {
            LexemeNode::Word(w, pos) => {
                if w.to_uppercase() == word.to_uppercase() {
                    self.consume();
                    Ok(Some(pos))
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
        }
    }

    pub fn try_consume_any_word(&mut self) -> Result<Option<(String, Location)>, LexerError> {
        let lexeme = self.read()?;
        match lexeme {
            LexemeNode::Word(w, pos) => {
                self.consume();
                Ok(Some((w, pos)))
            }
            _ => Ok(None),
        }
    }

    pub fn try_consume_symbol(&mut self, ch: char) -> Result<Option<Location>, LexerError> {
        let lexeme = self.read()?;
        match lexeme {
            LexemeNode::Symbol(w, pos) => {
                if w == ch {
                    self.consume();
                    Ok(Some(pos))
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
        }
    }

    pub fn try_consume_symbol_one_of(
        &mut self,
        symbols: Vec<char>,
    ) -> Result<Option<(char, Location)>, LexerError> {
        let lexeme = self.read()?;
        match lexeme {
            LexemeNode::Symbol(w, pos) => {
                if symbols.contains(&w) {
                    self.consume();
                    Ok(Some((w, pos)))
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
        }
    }

    pub fn demand_any_word(&mut self) -> Result<(String, Location), LexerError> {
        let lexeme = self.read()?;
        match lexeme {
            LexemeNode::Word(w, pos) => {
                self.consume();
                Ok((w, pos))
            }
            _ => Err(LexerError::Unexpected("Expected word".to_string(), lexeme)),
        }
    }

    pub fn demand_specific_word(&mut self, expected: &str) -> Result<Location, LexerError> {
        let (word, pos) = self.demand_any_word()?;
        if word.to_uppercase() != expected.to_uppercase() {
            Err(LexerError::Unexpected(
                format!("Expected {}", expected),
                LexemeNode::Word(word, pos),
            ))
        } else {
            Ok(pos)
        }
    }

    pub fn demand_symbol(&mut self, ch: char) -> Result<Location, LexerError> {
        let lexeme = self.read()?;
        match lexeme {
            LexemeNode::Symbol(s, _) => {
                if s == ch {
                    self.consume();
                    return Ok(lexeme.location());
                }
            }
            _ => (),
        }

        Err(LexerError::Unexpected(
            format!("Expected symbol {}", ch),
            lexeme,
        ))
    }

    pub fn demand_eol(&mut self) -> Result<(), LexerError> {
        let lexeme = self.read()?;
        match lexeme {
            LexemeNode::EOL(_, _) => {
                self.consume();
                Ok(())
            }
            _ => Err(LexerError::Unexpected("Expected EOL".to_string(), lexeme)),
        }
    }

    pub fn demand_eol_or_eof(&mut self) -> Result<(), LexerError> {
        let lexeme = self.read()?;
        match lexeme {
            LexemeNode::EOL(_, _) | LexemeNode::EOF(_) => {
                self.consume();
                Ok(())
            }
            _ => Err(LexerError::Unexpected(
                "Expected EOL or EOF".to_string(),
                lexeme,
            )),
        }
    }

    pub fn demand_whitespace(&mut self) -> Result<(), LexerError> {
        let lexeme = self.read()?;
        match lexeme {
            LexemeNode::Whitespace(_, _) => {
                self.consume();
                Ok(())
            }
            _ => Err(LexerError::Unexpected(
                "Expected whitespace".to_string(),
                lexeme,
            )),
        }
    }

    /// Reads and consumes while the next lexeme is Whitespace.
    ///
    /// Returns true if at least one Whitespace was found, false otherwise.
    pub fn skip_whitespace(&mut self) -> Result<bool, LexerError> {
        let mut found = false;
        loop {
            let lexeme = self.read()?;
            match lexeme {
                LexemeNode::Whitespace(_, _) => {
                    found = true;
                    self.consume();
                }
                _ => break,
            }
        }
        Ok(found)
    }

    pub fn skip_whitespace_and_eol(&mut self) -> Result<(), LexerError> {
        loop {
            let lexeme = self.read()?;
            match lexeme {
                LexemeNode::Whitespace(_, _) | LexemeNode::EOL(_, _) => self.consume(),
                _ => break,
            }
        }
        Ok(())
    }
}

// bytes || &str -> BufLexer
impl<T> From<T> for BufLexer<BufReader<Cursor<T>>>
where
    T: AsRef<[u8]>,
{
    fn from(input: T) -> Self {
        BufLexer::new(Lexer::from(input))
    }
}

// File -> BufLexer
impl From<File> for BufLexer<BufReader<File>> {
    fn from(input: File) -> Self {
        BufLexer::new(Lexer::from(input))
    }
}
