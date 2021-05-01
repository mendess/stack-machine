#![deny(unused_must_use)]

mod error;
mod ops;
mod stack;
mod util;

pub use error::Error;
pub use stack::Value;
use std::io::{self, BufRead, BufReader};
use util::StrExt;

#[derive(Default)]
pub struct Repl {
    stack: stack::Stack,
}

impl Repl {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn next_line(&mut self, s: &str) {
        if let Err(e) = ops::execute(s.split_tokens(), &mut self.stack) {
            eprintln!("{:?}", e);
        }
    }

    pub fn into_vec(self) -> Vec<Value> {
        self.stack.into_vec()
    }
}

pub fn run(s: &str) -> Result<Vec<Value>, error::Error> {
    run_with_input(s, BufReader::new(io::stdin()))
}

pub fn run_with_input<I: BufRead + 'static>(s: &str, i: I) -> Result<Vec<Value>, error::Error> {
    let mut stack = stack::Stack::with_input(i);
    ops::execute(s.split_tokens(), &mut stack)?;
    Ok(stack.into_vec())
}
