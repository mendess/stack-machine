use super::Operator;
use crate::{error::runtime::*, stack::Stack};
use std::str::FromStr;

pub struct Ternary;

impl FromStr for Ternary {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "?" => Ok(Self),
            _ => Err(()),
        }
    }
}

impl Operator for Ternary {
    fn run(&mut self, stack: &mut Stack) -> RuntimeResult<()> {
        let elze = stack.pop();
        let then = stack.pop();
        let cond = stack.pop();
        if let (Some(cond), Some(then), Some(elze)) = (cond, then, elze) {
            let v = if cond.into() { then } else { elze };
            Ok(stack.push(v))
        } else {
            Err(RuntimeError::StackEmpty)
        }
    }
}
