mod common;
mod interpreter;
mod lexer;
mod parser;
mod reader;
use std::env;
use std::fs::File;
use std::io::BufReader;

use interpreter::Interpreter;

fn main() {
    let f = File::open(env::args().skip(1).take(1).last().unwrap()).unwrap();
    let r = BufReader::new(f);
    let mut interpreter = Interpreter::new(r);
    interpreter.interpret().unwrap();
}
