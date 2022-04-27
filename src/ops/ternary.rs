use super::Operator;
use crate::stack::Stack;
use std::{
    fmt::{self, Debug, Display},
    str::FromStr,
};

pub struct Ternary(&'static str);

impl FromStr for Ternary {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "?" => Ok(Self("?")),
            _ => Err(()),
        }
    }
}

impl Operator for Ternary {
    fn run(&self, stack: &mut Stack) -> Result<(), crate::Error> {
        let elze = stack.pop()?;
        let then = stack.pop()?;
        let cond = stack.pop()?;
        let v = if cond.into() { then } else { elze };
        stack.push(v);
        Ok(())
    }

    fn as_str(&self) -> &str {
        self.0
    }
}

impl Display for Ternary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

impl Debug for Ternary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}
