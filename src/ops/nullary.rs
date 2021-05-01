use super::Operator;
use crate::{
    error::runtime::*,
    stack::{value::Value, Stack},
};
use std::str::FromStr;

pub struct Nullary(fn(stack: &mut Stack) -> RuntimeResult<()>);

impl FromStr for Nullary {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "t" => Ok(Self(|s| {
                let mut buf = String::new();
                s.input().read_to_string(&mut buf)?;
                Ok(s.push(buf.into()))
            })),
            "l" => Ok(Self(|s| {
                let mut buf = String::new();
                s.input().read_line(&mut buf)?;
                if buf.ends_with('\n') { buf.pop(); }
                Ok(s.push(Value::Str(buf)))
            })),
            _ => Err(()),
        }
    }
}

impl Operator for Nullary {
    fn run(&mut self, stack: &mut Stack) -> RuntimeResult<()> {
        self.0(stack)
    }
}
