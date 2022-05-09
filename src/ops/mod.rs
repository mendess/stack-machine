mod binary;
mod nullary;
mod stack;
mod ternary;
mod unary;

use crate::{
    error::{Error, SyntaxError},
    stack::{Stack, Value},
};
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

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
            .or_else(|_| s.parse::<Nullary>().map(cast_box))
            .or_else(|_| s.parse::<Ternary>().map(cast_box))
            .or_else(|_| s.parse::<StackOp>().map(cast_box))
            .map_err(|_| s.into())
    }
}

pub trait Operator: Display + Debug {
    fn run_mut(&mut self, stack: &mut Stack) -> Result<(), crate::Error> {
        self.run(stack)
    }

    fn run(&self, stack: &mut Stack) -> Result<(), crate::Error>;

    fn as_str(&self) -> &str;
}

pub fn parse_and_execute<'s, I>(i: I, stack: &'_ mut Stack) -> Result<(), Error>
where
    I: Iterator<Item = &'s str>,
{
    i.map(str::parse::<Box<dyn Operator>>).try_for_each(|op| {
        let mut op = op?;
        if cfg!(debug_assertions) {
            println!("{:?} apply `{}`", stack.as_slice(), op.as_str());
        }
        op.run_mut(stack)
    })?;
    if cfg!(debug_assertions) {
        println!("END {:?}", stack.as_slice());
    }
    Ok(())
}

pub fn execute<I, O>(i: I, stack: &'_ mut Stack) -> Result<(), crate::Error>
where
    I: IntoIterator<Item = O>,
    O: AsRef<dyn Operator>,
{
    i.into_iter()
        .try_for_each::<_, Result<(), crate::Error>>(|op| {
            if cfg!(debug_assertions) {
                println!("{:?} apply `{}`", stack.as_slice(), op.as_ref().as_str());
            }
            op.as_ref().run(stack)?;
            Ok(())
        })?;
    Ok(())
}

pub fn calculate<I, O>(input: Value, i: I, stack: &mut Stack) -> Result<Value, crate::Error>
where
    I: IntoIterator<Item = O>,
    O: AsRef<dyn Operator>,
{
    stack.push(input);
    execute(i, stack)?;
    Ok(stack.take_as_value()?)
}
