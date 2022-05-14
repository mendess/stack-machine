use super::Operator;
use crate::{
    error::runtime::*,
    ops::{calculate, execute},
    stack::{value::Value, Stack},
};
use std::{
    fmt::{self, Debug, Display},
    str::FromStr,
};

pub struct UnaryOp(Enum, String);

enum Enum {
    Transform(fn(Value) -> RuntimeResult<Value>),
    TransformStack(fn(Value, &mut Stack) -> Result<Value, crate::Error>),
    TransformStar(fn(Value, &mut Stack) -> Result<Vec<Value>, crate::Error>),
    Borrow(fn(&Value)),
    Calculate(fn(&Value) -> RuntimeResult<Value>),
}

impl FromStr for UnaryOp {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            match s {
                "_" => Enum::Calculate(|v| Ok(v.clone())),
                "~" => Enum::TransformStar(|x, s| match x {
                    Value::Integer(i) => Ok(vec![Value::Integer(!i)]),
                    Value::Array(a) => Ok(a),
                    Value::Block(b) => {
                        execute(b, s)?;
                        Ok(vec![])
                    }
                    _ => crate::rt_error!(op: x => [bit_not_or_spread]),
                }),
                "!" => Enum::Transform(|x| Ok((!bool::from(&x)).into())),
                "c" => Enum::Transform(Value::to_char),
                "f" => Enum::Transform(Value::to_float),
                "i" => Enum::Transform(Value::to_int),
                "s" => Enum::Transform(Value::to_str),
                "p" => Enum::Borrow(|x| println!("{}", x)),
                "," => Enum::TransformStack(|x, s| match x {
                    Value::Integer(i) => Ok((0..i).map(Value::from).collect::<Vec<_>>().into()),
                    Value::Array(a) => Ok(a.len().into()),
                    Value::Str(s) => Ok(s.len().into()),
                    Value::Block(b) => match s.pop()? {
                        Value::Array(mut a) => {
                            let mut indexes = Vec::with_capacity(a.len());
                            for (i, v) in a.iter().enumerate().rev() {
                                if calculate(v.clone(), &b, &mut s.sub_stack())?.into() {
                                    indexes.push(i);
                                }
                            }
                            let mut idx = 0;
                            a.retain(|_| {
                                let keep = match indexes.last() {
                                    Some(i) if *i == idx => {
                                        indexes.pop();
                                        true
                                    }
                                    _ => false,
                                };
                                idx += 1;
                                keep
                            });
                            debug_assert!(indexes.is_empty());
                            Ok(Value::Array(a))
                        }
                        Value::Str(mut string) => {
                            let mut temp_stack = s.sub_stack();
                            let mut indexes = Vec::with_capacity(string.len());
                            for (i, c) in string.char_indices().rev() {
                                if calculate(Value::Char(c), &b, &mut temp_stack)?.into() {
                                    indexes.push(i);
                                }
                            }
                            let mut idx = 0;
                            string.retain(|_| {
                                let keep = match indexes.last() {
                                    Some(i) if *i == idx => {
                                        indexes.pop();
                                        true
                                    }
                                    _ => false,
                                };
                                idx += 1;
                                keep
                            });
                            debug_assert!(indexes.is_empty());
                            Ok(Value::Str(string))
                        }
                        x => crate::rt_error!(op: x => [filter]),
                    },
                    x => crate::rt_error!(op: x => [length_range]),
                }),
                "S/" => Enum::Transform(|x| {
                    if let Value::Str(s) = x {
                        Ok(Value::Array(
                            s.split_whitespace().map(Value::from).collect(),
                        ))
                    } else {
                        crate::rt_error!(op: x => [split_whitespace])
                    }
                }),
                "N/" => Enum::Transform(|x| {
                    if let Value::Str(s) = x {
                        Ok(Value::Array(s.split('\n').map(Value::from).collect()))
                    } else {
                        crate::rt_error!(op: x => [split_newline])
                    }
                }),
                _ => return Err(()),
            },
            s.into(),
        ))
    }
}

impl Operator for UnaryOp {
    fn run(&self, stack: &mut Stack) -> Result<(), crate::Error> {
        match self.0 {
            Enum::Transform(f) => stack
                .pop()
                .and_then(f)
                .map(|v| stack.push(v))
                .map_err(crate::Error::from),
            Enum::TransformStack(f) => stack
                .pop()
                .map_err(crate::Error::from)
                .and_then(|v| f(v, stack))
                .map(|v| stack.push(v)),
            Enum::TransformStar(f) => stack
                .pop()
                .map_err(crate::Error::from)
                .and_then(|v| f(v, stack))
                .map(|v| v.into_iter().for_each(|v| stack.push(v))),
            Enum::Borrow(f) => stack.top().map(f).map_err(crate::Error::from),
            Enum::Calculate(f) => stack
                .top()
                .and_then(f)
                .map(|v| stack.push(v))
                .map_err(crate::Error::from),
        }
    }

    fn as_str(&self) -> &str {
        &self.1
    }
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.1)
    }
}

impl Debug for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.1)
    }
}
