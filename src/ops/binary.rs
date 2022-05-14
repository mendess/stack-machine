use super::Operator;
use crate::{
    error::runtime::*,
    ops::calculate,
    stack::{value::Value, Stack},
};
use std::{
    cmp::Ordering,
    convert::TryInto,
    fmt::{self, Debug, Display},
    mem::take,
    ops::*,
    str::FromStr,
};

#[derive(Clone)]
/* Copy, */
pub struct BinaryOp(
    fn(Value, Value, &mut Stack) -> Result<Value, crate::Error>,
    String,
);

impl FromStr for BinaryOp {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let op: fn(Value, Value, &mut Stack) -> Result<Value, crate::Error> = match s {
            "+" => |a, b, _| Value::add(a, b).map_err(crate::Error::from),
            "-" => |a, b, _| Value::sub(a, b).map_err(crate::Error::from),
            "*" => |a, b, s| match (a, b) {
                (Value::Array(a), Value::Block(b)) => {
                    let mut a = a.into_iter();
                    let first = a.next().ok_or(RuntimeError::FoldingEmptyArray)?;
                    a.try_fold(first, |acc, v| {
                        let mut temp_stack = s.sub_stack();
                        temp_stack.push(acc);
                        calculate(v, &b, &mut temp_stack)
                    })
                }
                (a, b) => Value::mul(a, b).map_err(crate::Error::from),
            },
            "/" => |a, b, _| Value::div(a, b).map_err(crate::Error::from),
            "&" => |a, b, _| Value::bitand(a, b).map_err(crate::Error::from),
            "|" => |a, b, _| Value::bitor(a, b).map_err(crate::Error::from),
            "^" => |a, b, _| Value::bitxor(a, b).map_err(crate::Error::from),
            "%" => |a, b, s| match (a, b) {
                (Value::Array(mut a), Value::Block(b)) => {
                    for v in &mut a {
                        *v = calculate(take(v), &b, &mut s.sub_stack())?;
                    }
                    Ok(Value::Array(a))
                }
                (Value::Str(string), Value::Block(b)) => Ok(Value::Str(
                    string
                        .chars()
                        .map(Value::Char)
                        .map(|c| -> Result<char, crate::Error> {
                            let mut temp_stack = s.sub_stack();
                            match calculate(c, &b, &mut temp_stack)? {
                                Value::Char(c) if temp_stack.is_empty() => Ok(c),
                                x => crate::rt_error!(convert: x, char),
                            }
                        })
                        .collect::<Result<_, _>>()?,
                )),
                (a, b) => Value::rem(a, b).map_err(crate::Error::from),
            },
            "e&" => |a: Value, b, _| Ok(a.and(b)),
            "e|" => |a: Value, b, _| Ok(a.or(b)),
            "e<" => |a, b, _| Value::min(a, b).map_err(crate::Error::from),
            "e>" => |a, b, _| Value::max(a, b).map_err(crate::Error::from),
            ">" => |a, b, _| match (a, b) {
                (Value::Array(mut arr), Value::Integer(i)) => {
                    if i > arr.len() as i64 || i < 0 {
                        crate::rt_error!(op: arr, i => [slice_begining])
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
                            crate::rt_error!(op: s, i => [str_begining])
                        }
                    }
                },
                (a, b) => Ok((a.partial_cmp(&b) == Some(Ordering::Greater)).into()),
            },
            "<" => |a, b, _| match (a, b) {
                (Value::Array(mut arr), Value::Integer(i)) => {
                    if i > arr.len() as i64 || i < 0 {
                        crate::rt_error!(op: arr, i => [slice_end])
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
                            crate::rt_error!(op: s, i => [str_end])
                        }
                    }
                },
                (a, b) => Ok((a.partial_cmp(&b) == Some(Ordering::Less)).into()),
            },
            "=" => |a, b, _| match (&a, &b) {
                (Value::Array(arr), Value::Integer(i)) => {
                    match <i64 as TryInto<usize>>::try_into(*i).map(|i| arr.get(i)) {
                        Ok(Some(v)) => Ok(v.clone()),
                        _ => crate::rt_error!(op: a, b => [index]),
                    }
                }
                (Value::Str(s), Value::Integer(i)) => {
                    match <i64 as TryInto<usize>>::try_into(*i).map(|i| s.chars().nth(i)) {
                        Ok(Some(v)) => Ok(v.into()),
                        _ => crate::rt_error!(op: a, b => [index]),
                    }
                }
                (a, b) => Ok((a.partial_cmp(b) == Some(Ordering::Equal)).into()),
            },
            "#" => |a: Value, b, _| a.pow(b).map_err(crate::Error::from),
            _ => return Err(()),
        };
        Ok(Self(op, s.into()))
    }
}

impl Operator for BinaryOp {
    fn run(&self, stack: &mut Stack) -> Result<(), crate::Error> {
        let snd = stack.pop()?;
        let fst = stack.pop()?;
        let r = self.0(fst, snd, stack)?;
        stack.push(r);
        Ok(())
    }

    fn as_str(&self) -> &str {
        &self.1
    }
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.1)
    }
}

impl Debug for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.1)
    }
}
