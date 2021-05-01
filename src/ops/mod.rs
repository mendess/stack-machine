mod binary;
mod nullary;
mod stack;
mod ternary;
mod unary;

use crate::{
    error::{Error, RuntimeError, SyntaxError},
    stack::Stack,
};
use std::str::FromStr;

use binary::BinaryOp;
use nullary::Nullary;
use stack::StackOp;
use ternary::Ternary;
use unary::UnaryOp;

impl FromStr for Box<dyn Operator> {
    type Err = SyntaxError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn cast_box<T: Operator + 'static>(t: T) -> Box<dyn Operator> {
            Box::new(t)
        }
        s.parse::<BinaryOp>()
            .map(cast_box)
            .or_else(|_| s.parse::<UnaryOp>().map(cast_box))
            .or_else(|_| s.parse::<StackOp>().map(cast_box))
            .or_else(|_| s.parse::<Ternary>().map(cast_box))
            .or_else(|_| s.parse::<Nullary>().map(cast_box))
            .map_err(|_| s.into())
    }
}

trait Operator {
    fn run(&mut self, stack: &mut Stack) -> Result<(), RuntimeError>;
}

pub fn execute<'s, I>(mut i: I, stack: &'_ mut Stack) -> Result<(), Error>
where
    I: Iterator<Item = &'s str>,
{
    i.try_for_each(|token| {
        let op = token.parse::<Box<dyn Operator>>();
        if cfg!(debug_assertions) {
            println!("{:?} apply `{}`", stack.as_slice(), token);
        }
        let r = Ok(op?.run(stack)?);
        r
    })
}
