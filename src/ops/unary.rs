use super::Operator;
use crate::{
    error::runtime::*,
    stack::{value::Value, Stack},
};
use std::str::FromStr;

pub enum UnaryOp {
    Transform(fn(Value) -> RuntimeResult<Value>),
    Borrow(fn(&Value)),
    Calculate(fn(&Value) -> RuntimeResult<Value>),
}

impl FromStr for UnaryOp {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "_" => Self::Calculate(|v| Ok(v.clone())),
            "~" => Self::Transform(|x| !x),
            "!" => Self::Transform(|x| Ok((!bool::from(&x)).into())),
            "c" => Self::Transform(Value::to_char),
            "f" => Self::Transform(Value::to_float),
            "i" => Self::Transform(Value::to_int),
            "s" => Self::Transform(Value::to_str),
            "p" => Self::Borrow(|x| println!("{}", x)),
            "," => Self::Transform(|x| match x {
                Value::Integer(i) => Ok((0..i).map(Value::from).collect::<Vec<_>>().into()),
                Value::Array(a) => Ok(a.len().into()),
                Value::Str(s) => Ok(s.len().into()),
                _ => Err(RuntimeError::InvalidOperation(vec![x.clone()], "length")),
            }),
            _ => return Err(()),
        })
    }
}

impl Operator for UnaryOp {
    fn run(&mut self, stack: &mut Stack) -> RuntimeResult<()> {
        match self {
            Self::Transform(f) => stack.pop().map(f).transpose()?.map(|v| stack.push(v)),
            Self::Borrow(f) => stack.top().map(f),
            Self::Calculate(f) => stack.top().map(f).transpose()?.map(|v| stack.push(v)),
        }
        .ok_or(RuntimeError::StackEmpty)
    }
}
