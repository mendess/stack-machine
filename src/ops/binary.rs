use super::Operator;
use crate::{
    error::runtime::*,
    stack::{value::Value, Stack},
};
use std::{cmp::Ordering, convert::TryInto, ops::*, str::FromStr};

pub struct BinaryOp(fn(Value, Value) -> RuntimeResult<Value>);

impl FromStr for BinaryOp {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let op = match s {
            "+" => Self(Value::add),
            "-" => Self(Value::sub),
            "*" => Self(Value::mul),
            "/" => Self(Value::div),
            "%" => Self(Value::rem),
            "&" => Self(Value::bitand),
            "|" => Self(Value::bitor),
            "^" => Self(Value::bitxor),
            "e&" => Self(|a, b| Ok(a.and(b))),
            "e|" => Self(|a, b| Ok(a.or(b))),
            "e<" => Self(Value::min),
            "e>" => Self(Value::max),
            ">" => Self(|a, b| match (a, b) {
                (Value::Array(mut arr), Value::Integer(i)) => {
                    if i > arr.len() as i64 || i < 0 {
                        Err(RuntimeError::InvalidOperation(
                            vec![Value::Array(arr), Value::Integer(i)],
                            "slice begining",
                        ))
                    } else {
                        drop(arr.drain(..(arr.len() - i as usize)));
                        Ok(Value::Array(arr))
                    }
                }
                (Value::Str(mut s), Value::Integer(i)) => match i {
                    0 => Ok(Value::Str(String::new())),
                    i => {
                        if let Some((i, _)) = s.char_indices().nth_back((i - 1) as usize) {
                            drop(s.drain(..i));
                            Ok(Value::Str(s))
                        } else {
                            Err(RuntimeError::InvalidOperation(
                                vec![Value::Str(s), Value::Integer(i)],
                                "str begining",
                            ))
                        }
                    }
                },
                (a, b) => Ok((a.partial_cmp(&b) == Some(Ordering::Greater)).into()),
            }),
            "<" => Self(|a, b| match (a, b) {
                (Value::Array(mut arr), Value::Integer(i)) => {
                    if i > arr.len() as i64 || i < 0 {
                        Err(RuntimeError::InvalidOperation(
                            vec![Value::Array(arr), Value::Integer(i)],
                            "slice end",
                        ))
                    } else {
                        drop(arr.drain((i as usize)..));
                        Ok(Value::Array(arr))
                    }
                }
                (Value::Str(mut s), Value::Integer(i)) => match i {
                    0 => Ok(Value::Str(String::new())),
                    i => {
                        if let Some((i, _)) = s.char_indices().nth(i as usize) {
                            drop(s.drain(i..));
                            Ok(Value::Str(s))
                        } else {
                            Err(RuntimeError::InvalidOperation(
                                vec![Value::Str(s), Value::Integer(i)],
                                "str begining",
                            ))
                        }
                    }
                },
                (a, b) => Ok((a.partial_cmp(&b) == Some(Ordering::Less)).into()),
            }),
            "=" => Self(|a, b| match (&a, &b) {
                (Value::Array(arr), Value::Integer(i)) => {
                    match <i64 as TryInto<usize>>::try_into(*i) {
                        Ok(i) => Ok(arr[i].clone()),
                        Err(_) => Err(RuntimeError::InvalidOperation(vec![a, b], "index")),
                    }
                }
                (a, b) => Ok((a.partial_cmp(&b) == Some(Ordering::Equal)).into()),
            }),
            "#" => Self(|a, b| a.pow(b)),
            _ => return Err(()),
        };
        Ok(op)
    }
}

impl Operator for BinaryOp {
    fn run(&mut self, stack: &mut Stack) -> Result<(), RuntimeError> {
        let snd = stack.pop().ok_or(RuntimeError::StackEmpty)?;
        let fst = stack.pop().ok_or(RuntimeError::StackEmpty)?;
        Ok(stack.push(self.0(fst, snd)?))
    }
}
