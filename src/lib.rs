#![deny(unused_must_use)]

mod error;
mod ops;
mod stack;
mod util;

pub use error::Error;
use stack::Stack;
pub use stack::Value;
use std::io::{self, BufRead, BufReader};
use util::str_ext::StrExt;

pub struct Repl<'i> {
    stack: stack::Stack<'i>,
}

impl<'i> Repl<'i> {
    pub fn new<I: BufRead>(i: &'i mut I) -> Self {
        Self { stack: Stack::with_input(i) }
    }

    pub fn next_line(&mut self, s: &str) {
        if let Err(e) = ops::parse_and_execute(s.split_tokens(), &mut self.stack) {
            eprintln!("{:?}", e);
        }
    }

    pub fn into_vec(self) -> Vec<Value> {
        self.stack.into_vec()
    }
}

pub fn run(s: &str) -> Result<Vec<Value>, error::Error> {
    run_with_input(s, &mut BufReader::new(io::stdin()))
}

pub fn run_on(s: &str, mut stack: Stack<'_>) -> Result<Vec<Value>, error::Error> {
    ops::parse_and_execute(s.split_tokens(), &mut stack)?;
    Ok(stack.into_vec())
}

pub fn run_with_input(s: &str, i: &mut dyn BufRead) -> Result<Vec<Value>, error::Error> {
    let mut stack = stack::Stack::with_input(i);
    ops::parse_and_execute(s.split_tokens(), &mut stack)?;
    Ok(stack.into_vec())
}
