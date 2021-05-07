use super::Operator;
use crate::{
    error::runtime::*,
    ops::{calculate, execute},
    stack::{value::Value, Stack},
};
use std::{
    fmt::{self, Debug, Display},
    mem::take,
    str::{from_utf8, FromStr},
};

pub struct StackOp(Enum, String);

enum Enum {
    Simple(fn(&mut Stack<'_>) -> RuntimeResult<()>),
    Push(Value),
    Nth(usize, fn(&mut Stack<'_>, usize) -> RuntimeResult<()>),
    VarAccess(char, fn(&mut Stack<'_>, char) -> RuntimeResult<()>),
}

impl FromStr for StackOp {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let e = match s.as_bytes() {
            b";" => Ok(Enum::Simple(|s| s.pop().map(|_| ()))),
            b"\\" => Ok(Enum::Simple(|s| {
                let len = s.len();
                s.get_mut((len - 2)..len)
                    .map(|slice| slice.rotate_left(1))
                    .ok_or(RuntimeError::StackEmpty)
            })),
            b"@" => Ok(Enum::Simple(|s| {
                let len = s.len();
                s.get_mut((len - 3)..len)
                    .map(|slice| slice.rotate_left(1))
                    .ok_or(RuntimeError::StackEmpty)
            })),
            b"(" => Ok(Enum::Simple(|s| {
                let top = match s.pop()? {
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
            b")" => Ok(Enum::Simple(|s| {
                let top = match s.pop()? {
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
            b"w" => Ok(Enum::Simple(|s| {
                let v = s.pop()?;
                if let Value::Block(b) = v {
                    while s.top()?.into() {
                        execute(&b, s)?;
                    }
                    Ok(())
                } else {
                    crate::rt_error!(op: v => [while])
                }
            })),
            [v @ b'A'..=b'Z'] => Ok(Enum::VarAccess(*v as _, |s, v| Ok(s.push_var(v)))),
            [b':', v @ b'A'..=b'Z'] => Ok(Enum::VarAccess(*v as _, |s, v| s.pop_var(v))),
            b"$" => Ok(Enum::Simple(|s| {
                let top = s.pop()?;
                if let Value::Integer(i) = top {
                    if i < 0 {
                        return Err(RuntimeError::OutOfBounds(s.len(), i));
                    }
                    if let Some(v) = s.get_from_end(i as usize).cloned() {
                        Ok(s.push(v))
                    } else {
                        Err(RuntimeError::OutOfBounds(s.len(), i))
                    }
                } else if let (Value::Array(mut a), Value::Block(b)) = (s.pop()?, &top) {
                    {
                        let mut temp_stack = Stack::with_input(s.input());
                        let mut keys = take(&mut a)
                            .into_iter()
                            .map(|v| calculate(v.clone(), b, &mut temp_stack).map(|key| (v, key)))
                            .collect::<Result<Vec<_>, _>>()?;
                        keys.sort_by(|(_, key0), (_, key1)| key0.cmp(key1));
                        a.extend(keys.into_iter().map(|(v, _)| v));
                    }
                    Ok(s.push(a.into()))
                } else {
                    crate::rt_error!(op: top => [index_sort])
                }
            })),
            // n$
            [rest @ .., b'$'] => {
                if let Ok(n) = from_utf8(rest).unwrap().trim().parse::<usize>() {
                    Ok(Enum::Nth(n, |s, n| {
                        s.get_from_end(n)
                            .cloned()
                            .map(|p| s.push(p))
                            .ok_or_else(|| RuntimeError::OutOfBounds(s.len(), n as _))
                    }))
                } else {
                    Err(())
                }
            }
            _ => Ok(Enum::Push(s.parse()?)),
        };
        e.map(|e| Self(e, s.into()))
    }
}

impl Operator for StackOp {
    fn run(&self, stack: &mut Stack) -> Result<(), RuntimeError> {
        match &self.0 {
            Enum::Simple(f) => f(stack),
            Enum::Push(v) => Ok(stack.push(v.clone())),
            Enum::VarAccess(v, f) => f(stack, *v),
            Enum::Nth(n, f) => f(stack, *n),
        }
    }

    fn run_mut(&mut self, stack: &mut Stack) -> Result<(), RuntimeError> {
        use std::mem::replace;
        match &mut self.0 {
            Enum::Simple(f) => f(stack),
            Enum::Push(v) => Ok(stack.push(replace(v, Default::default()))),
            Enum::VarAccess(v, f) => f(stack, *v),
            Enum::Nth(n, f) => f(stack, *n),
        }
    }

    fn as_str(&self) -> &str {
        &self.1
    }
}

impl Display for StackOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.1)
    }
}

impl Debug for StackOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.1)
    }
}
