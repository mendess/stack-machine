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

#[derive(Default)]
pub struct Repl {
    stack: stack::Stack<'static>,
}

impl Repl {
    pub fn new() -> Self {
        Default::default()
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
    run_with_input(s, BufReader::new(io::stdin()))
}

pub fn run_on(s: &str, stack: &mut Stack<'_>) -> Result<Vec<Value>, error::Error> {
    let mut stack = stack.sub_stack();
    ops::parse_and_execute(s.split_tokens(), &mut stack)?;
    Ok(stack.into_vec())
}

pub fn run_with_input<'i, I: BufRead + 'i>(s: &str, i: I) -> Result<Vec<Value>, error::Error> {
    let mut stack = stack::Stack::with_input(i);
    ops::parse_and_execute(s.split_tokens(), &mut stack)?;
    Ok(stack.into_vec())
}
