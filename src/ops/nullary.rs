use super::Operator;
use crate::{
    error::runtime::*,
    stack::{value::Value, Stack},
};
use std::{
    fmt::{self, Debug, Display},
    str::FromStr, borrow::Cow,
};

pub struct Nullary(for<'fv> fn(stack: &mut Stack<'_, 'fv>) -> RuntimeResult<'fv, ()>, String);

impl FromStr for Nullary {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "t" => Ok(Self(
                |s| {
                    let mut buf = String::new();
                    s.input().read_to_string(&mut buf)?;
                    s.push(buf.into());
                    Ok(())
                },
                s.into(),
            )),
            "l" => Ok(Self(
                |s| {
                    let mut buf = String::new();
                    s.input().read_line(&mut buf)?;
                    if buf.ends_with('\n') {
                        buf.pop();
                    }
                    s.push(Value::Str(Cow::Owned(buf)));
                    Ok(())
                },
                s.into(),
            )),
            _ => Err(()),
        }
    }
}

impl Operator for Nullary {
    fn run<'v>(&self, stack: &mut Stack<'_, 'v>) -> RuntimeResult<'v, ()> {
        self.0(stack)
    }

    fn as_str(&self) -> &str {
        &self.1
    }
}

impl Display for Nullary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.1)
    }
}

impl Debug for Nullary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.1)
    }
}
