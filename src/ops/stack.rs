use super::Operator;
use crate::{
    error::runtime::*,
    stack::{value::Value, Stack},
};
use std::{
    mem,
    str::{from_utf8, FromStr},
};

pub enum StackOp {
    Simple(fn(&mut Stack) -> RuntimeResult<()>),
    Push(Value),
    Nth(usize, fn(&mut Stack, usize) -> RuntimeResult<()>),
    VarAccess(char, fn(&mut Stack, char) -> RuntimeResult<()>),
}

impl FromStr for StackOp {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.as_bytes() {
            b";" => Ok(StackOp::Simple(|s| {
                s.pop().map(|_| ()).ok_or(RuntimeError::StackEmpty)
            })),
            b"\\" => Ok(StackOp::Simple(|s| {
                let len = s.len();
                s.get_mut((len - 2)..len)
                    .map(|slice| slice.rotate_left(1))
                    .ok_or(RuntimeError::StackEmpty)
            })),
            b"@" => Ok(StackOp::Simple(|s| {
                let len = s.len();
                s.get_mut((len - 3)..len)
                    .map(|slice| slice.rotate_left(1))
                    .ok_or(RuntimeError::StackEmpty)
            })),
            b"(" => Ok(Self::Simple(|s| {
                let top = match s.pop().ok_or(RuntimeError::StackEmpty)? {
                    Value::Array(mut a) => {
                        if a.is_empty() {
                            Err(RuntimeError::InvalidOperation(
                                vec![a.into()],
                                "remove head",
                            ))
                        } else {
                            let v = a.remove(0);
                            s.push(a.into());
                            Ok(v)
                        }
                    }
                    x => x - Value::Integer(1),
                };
                Ok(s.push(top?))
            })),
            b")" => Ok(Self::Simple(|s| {
                let top = match s.pop().ok_or(RuntimeError::StackEmpty)? {
                    Value::Array(mut a) => {
                        if let Some(v) = a.pop() {
                            s.push(a.into());
                            Ok(v)
                        } else {
                            Err(RuntimeError::InvalidOperation(
                                vec![Value::Array(a)],
                                "remove last",
                            ))
                        }
                    }
                    x => x + Value::Integer(1),
                };
                Ok(s.push(top?))
            })),
            [v @ b'A'..=b'Z'] => Ok(Self::VarAccess(*v as _, |s, v| Ok(s.push(s[v].clone())))),
            [b':', v @ b'A'..=b'Z'] => Ok(Self::VarAccess(*v as _, |s, v| {
                s.pop().map(|x| s[v] = x).ok_or(RuntimeError::StackEmpty)
            })),
            b"$" => Ok(StackOp::Simple(|s| match s.pop() {
                Some(Value::Integer(i)) if i >= 0 => {
                    if let Some(v) = s.get_from_end(i as usize).cloned() {
                        Ok(s.push(v))
                    } else {
                        Err(RuntimeError::OutOfBounds(s.len(), i))
                    }
                }
                Some(v) => Err(RuntimeError::InvalidOperation(vec![v], "index")),
                _ => Err(RuntimeError::StackEmpty),
            })),
            // n$
            [rest @ .., b'$'] => {
                if let Ok(n) = from_utf8(rest).unwrap().trim().parse::<usize>() {
                    Ok(StackOp::Nth(n, |s, n| {
                        s.get_from_end(n)
                            .cloned()
                            .map(|p| s.push(p))
                            .ok_or(RuntimeError::StackEmpty)
                    }))
                } else {
                    Err(())
                }
            }
            _ => Ok(StackOp::Push(s.parse()?)),
        }
    }
}

impl Operator for StackOp {
    fn run(&mut self, stack: &mut Stack) -> Result<(), RuntimeError> {
        match self {
            StackOp::Simple(f) => f(stack),
            StackOp::Push(v) => Ok(stack.push(mem::replace(v, Default::default()))),
            StackOp::VarAccess(v, f) => f(stack, *v),
            StackOp::Nth(n, f) => f(stack, *n),
        }
    }
}
