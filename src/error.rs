use crate::stack::Value;
use std::io;

pub mod runtime {
    pub use super::RuntimeError;
    pub use super::RuntimeResult;
}

pub mod syntax {
    pub use super::SyntaxError;
    pub use super::SyntaxResult;
}

pub mod both {
    pub use super::runtime::*;
    pub use super::syntax::*;
}

#[derive(Debug)]
pub enum Error {
    Syntax(SyntaxError),
    Runtime(RuntimeError),
}

impl From<SyntaxError> for Error {
    fn from(e: SyntaxError) -> Self {
        Self::Syntax(e)
    }
}

impl From<RuntimeError> for Error {
    fn from(e: RuntimeError) -> Self {
        Self::Runtime(e)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SyntaxError(String);

impl From<&str> for SyntaxError {
    fn from(s: &str) -> Self {
        Self(s.into())
    }
}

impl From<String> for SyntaxError {
    fn from(s: String) -> Self {
        Self(s)
    }
}

#[derive(Debug)]
pub enum RuntimeError {
    StackEmpty,
    Io(io::Error),
    InvalidOperation(Vec<Value>, &'static str),
    InvalidCast(Value, &'static str),
    OutOfBounds(usize, i64),
}

impl From<io::Error> for RuntimeError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

pub type RuntimeResult<T> = Result<T, RuntimeError>;

#[allow(dead_code)]
pub type SyntaxResult<T> = Result<T, SyntaxError>;
