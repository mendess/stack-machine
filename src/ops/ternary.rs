use super::Operator;
use crate::{error::runtime::*, stack::Stack};
use std::{
    fmt::{self, Debug, Display},
    str::FromStr,
};

pub struct Ternary(String);

impl FromStr for Ternary {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "?" => Ok(Self(s.into())),
            _ => Err(()),
        }
    }
}

impl Operator for Ternary {
    fn run(&self, stack: &mut Stack) -> RuntimeResult<()> {
        let elze = stack.pop()?;
        let then = stack.pop()?;
        let cond = stack.pop()?;
        let v = if cond.into() { then } else { elze };
        stack.push(v);
        Ok(())
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for Ternary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Debug for Ternary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
