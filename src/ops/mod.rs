mod binary;
mod nullary;
mod stack;
mod ternary;
mod unary;

use crate::{
    error::{runtime::*, Error, SyntaxError},
    stack::{Stack, Value},
};
use std::fmt::{Debug, Display};

use binary::BinaryOp;
use nullary::Nullary;
use stack::StackOp;
use ternary::Ternary;
use unary::UnaryOp;

pub fn operator_from_str<'v>(s: &'v str) -> Result<Box<dyn Operator<'v> + 'v>, SyntaxError> {
    fn cast_box<'v, T: Operator<'v> + 'v>(t: T) -> Box<dyn Operator<'v> + 'v> {
        Box::new(t)
    }
    s.parse::<BinaryOp>()
        .map(cast_box)
        .or_else(|_| s.parse::<UnaryOp>().map(cast_box))
        .or_else(|_| s.parse::<Nullary>().map(cast_box))
        .or_else(|_| StackOp::from_str(s).map(cast_box).ok_or(()))
        .or_else(|_| s.parse::<Ternary>().map(cast_box))
        .map_err(|_| s.into())
}

pub trait Operator<'v>: Display + Debug {
    fn run_mut(&mut self, stack: &mut Stack<'_, 'v>) -> Result<(), RuntimeError<'v>> {
        self.run(stack)
    }

    fn run(&self, stack: &mut Stack<'_, 'v>) -> Result<(), RuntimeError<'v>>;

    fn as_str(&self) -> &str;

    fn into_owned(self: Box<Self>) -> Box<dyn Operator<'static>>;
}

pub fn parse_and_execute<'s, I>(i: I, stack: &'_ mut Stack<'_, 's>) -> Result<(), Error<'s>>
where
    I: Iterator<Item = &'s str>,
{
    i.map(|s| operator_from_str(s)).try_for_each(|op| {
        let mut op = op?;
        if cfg!(debug_assertions) {
            println!("{:?} apply `{}`", stack.as_slice(), op.as_str());
        }
        Ok(op.run_mut(stack)?)
    })
}

pub fn execute<'v, I, O>(i: I, stack: &'_ mut Stack<'_, 'v>) -> RuntimeResult<'v, ()>
where
    I: IntoIterator<Item = O>,
    O: AsRef<dyn Operator<'v>>,
{
    i.into_iter().try_for_each::<_, RuntimeResult<_>>(|op| {
        if cfg!(debug_assertions) {
            println!("{:?} apply `{}`", stack.as_slice(), op.as_ref().as_str());
        }
        op.as_ref().run(stack)?;
        Ok(())
    })?;
    if cfg!(debug_assertions) {
        println!("END {:?}", stack.as_slice());
    }
    Ok(())
}

pub fn calculate<'i, 'v, I, O>(
    input: Value<'v>,
    i: I,
    stack: &mut Stack<'i, 'v>,
) -> RuntimeResult<'v, Value<'v>>
where
    I: IntoIterator<Item = O>,
    O: AsRef<dyn Operator<'v>>,
{
    stack.push(input);
    execute(i, stack)?;
    stack.take_as_value()
}
