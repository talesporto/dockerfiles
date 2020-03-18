use std::io::prelude::*;
use std::io::{Error, ErrorKind};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum CharOrEof {
    EOF,

    Char(char),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct RowCol {
    row: u32,
    col: u32,
}

impl RowCol {
    pub fn new() -> RowCol {
        RowCol { row: 0, col: 0 }
    }

    pub fn row(&self) -> u32 {
        self.row
    }
    pub fn col(&self) -> u32 {
        self.col
    }
    pub fn inc_col(&mut self) {
        self.col += 1
    }
    pub fn inc_row(&mut self) {
        self.row += 1;
        self.col = 0;
    }
}

#[derive(Debug)]
pub struct CharOrEofReader<T> {
    reader: T,
    _buffer: Vec<CharOrEof>,
    _pos: RowCol,
}

impl<T: BufRead> CharOrEofReader<T> {
    pub fn new(reader: T) -> CharOrEofReader<T> {
        CharOrEofReader {
            reader,
            _buffer: vec![],
            _pos: RowCol::new(),
        }
    }

    pub fn read(&mut self) -> std::io::Result<CharOrEof> {
        self._fill_buffer_if_empty()?;
        Ok(self._buffer[0])
    }

    pub fn consume(&mut self) -> std::io::Result<CharOrEof> {
        if self._buffer.is_empty() {
            Err(Error::new(ErrorKind::Other, "Buffer underrun"))
        } else {
            self._pos.inc_col();
            Ok(self._buffer.remove(0))
        }
    }

    pub fn read_and_consume(&mut self) -> std::io::Result<CharOrEof> {
        self._fill_buffer_if_empty()?;
        self._pos.inc_col();
        Ok(self._buffer.remove(0))
    }

    pub fn pos(&self) -> RowCol {
        self._pos
    }

    fn _fill_buffer_if_empty(&mut self) -> std::io::Result<()> {
        if self._buffer.is_empty() {
            let mut line = String::new();
            let bytes_read = self.reader.read_line(&mut line)?;
            if bytes_read <= 0 {
                self._buffer.push(CharOrEof::EOF);
            } else {
                self._pos.inc_row();
                for c in line.chars() {
                    self._buffer.push(CharOrEof::Char(c))
                }
                if self._buffer.is_empty() {
                    panic!("Should have found at least one character")
                }
            }
        }
        Ok(())
    }
}
