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
pub enum Error<'v> {
    Syntax(SyntaxError),
    Runtime(RuntimeError<'v>),
}

#[derive(Debug)]
pub enum RuntimeError<'v> {
    StackEmpty,
    Io(io::Error),
    InvalidOperation(Vec<Value<'v>>, &'static str),
    InvalidCast(Value<'v>, &'static str),
    OutOfBounds(usize, i64),
    FoldingEmptyArray,
}

impl<'v> From<RuntimeError<'v>> for Error<'v> {
    fn from(e: RuntimeError<'v>) -> Self {
        Self::Runtime(e)
    }
}

impl<'v> From<io::Error> for RuntimeError<'v> {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

pub type RuntimeResult<'v, T> = Result<T, RuntimeError<'v>>;

#[macro_export]
macro_rules! rt_error {
    (convert: $a:expr, $t:ty) => {
        return ::std::result::Result::Err($crate::error::RuntimeError::InvalidCast(
            $crate::Value::from($a),
            ::std::stringify!($t),
        ))
    };
    (op: $a:expr => [$op:ident]) => {
        return ::std::result::Result::Err($crate::error::RuntimeError::InvalidOperation(
            ::std::vec![$crate::Value::from($a)],
            ::std::stringify!($op),
        ))
    };
    (op: $a:expr, $b:expr => [$op:ident]) => {
        return ::std::result::Result::Err($crate::error::RuntimeError::InvalidOperation(
            ::std::vec![$crate::Value::from($a), $crate::Value::from($b)],
            ::std::stringify!($op),
        ))
    };
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

impl From<SyntaxError> for Error<'_> {
    fn from(e: SyntaxError) -> Self {
        Self::Syntax(e)
    }
}

#[allow(dead_code)]
pub type SyntaxResult<T> = Result<T, SyntaxError>;
