use crate::{
    error::both::*,
    ops::{operator_from_str, Operator},
    util::str_ext::StrExt,
};
use itertools::Itertools;
use std::{
    borrow::Cow,
    cmp,
    convert::TryInto,
    fmt::{self, Write},
    ops::{self, Deref},
    rc::Rc,
};

#[derive(Clone, Debug)]
pub enum Value<'s> {
    Char(char),
    Integer(i64),
    Float(f64),
    Str(Cow<'s, str>), // TODO: try to make it a cow
    Array(Vec<Value<'s>>),
    Block(Vec<Rc<dyn Operator<'s> + 's>>),
}

impl Default for Value<'_> {
    fn default() -> Self {
        Self::Integer(0)
    }
}

impl cmp::PartialEq for Value<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Char(c0), Value::Char(c1)) => c0 == c1,
            (Value::Integer(i0), Value::Integer(i1)) => i0 == i1,
            (Value::Float(f0), Value::Float(f1)) => f0 == f1,
            (Value::Str(s0), Value::Str(s1)) => s0 == s1,
            (Value::Array(a0), Value::Array(a1)) => a0 == a1,
            (Value::Block(b0), Value::Block(b1)) => b0
                .iter()
                .map(|o| o.as_str())
                .zip(b1.iter().map(|o| o.as_str()))
                .all(|(b0, b1)| b0 == b1),
            _ => false,
        }
    }
}

impl cmp::PartialOrd for Value<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match (self, other) {
            (Value::Char(c0), Value::Char(c1)) => c0.partial_cmp(c1),
            (Value::Integer(i0), Value::Integer(i1)) => i0.partial_cmp(i1),
            (Value::Float(f0), Value::Float(f1)) => f0.partial_cmp(f1),
            (Value::Str(s0), Value::Str(s1)) => s0.partial_cmp(s1),
            (Value::Array(a0), Value::Array(a1)) => a0.partial_cmp(a1),
            (Value::Block(_), Value::Block(_)) => None,
            _ => None,
        }
    }
}

impl Eq for Value<'_> {}

impl cmp::Ord for Value<'_> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        #[allow(clippy::comparison_chain)]
        if self < other {
            cmp::Ordering::Less
        } else if self > other {
            cmp::Ordering::Greater
        } else {
            cmp::Ordering::Equal
        }
    }
}

impl From<&Value<'_>> for bool {
    fn from(v: &Value) -> Self {
        match v {
            Value::Char('\0') | Value::Integer(0) => false,
            Value::Float(f) if *f == 0.0 => false,
            Value::Str(a) if a.is_empty() => false,
            Value::Array(v) if v.is_empty() => false,
            _ => true,
        }
    }
}

impl From<Value<'_>> for bool {
    fn from(v: Value) -> Self {
        match v {
            Value::Char('\0') | Value::Integer(0) => false,
            Value::Float(f) if f == 0.0 => false,
            Value::Str(a) if a.is_empty() => false,
            Value::Array(v) if v.is_empty() => false,
            _ => true,
        }
    }
}

macro_rules! impl_from_rust {
    ($($variant:ident [$($t:ty),* $(,)?]);* $(;)?) => {
        $($(
        impl From<$t> for Value<'_> {
            fn from(t: $t) -> Self {
                Self::$variant(t as _)
            }
        }
        )*)*
    }
}

impl_from_rust! {
    Integer [usize, i64, u32, i32, u16, u8, bool];
    Float [f32, f64];
    Char [char];
    // Str [String];
}

impl From<String> for Value<'_> {
    fn from(s: String) -> Self {
        Self::Str(Cow::Owned(s))
    }
}

impl<'v> From<Cow<'v, str>> for Value<'v> {
    fn from(s: Cow<'v, str>) -> Self {
        Self::Str(s)
    }
}

impl<'s> From<&'s str> for Value<'s> {
    fn from(s: &'s str) -> Self {
        Self::Str(Cow::Borrowed(s))
    }
}

impl<T> From<Vec<T>> for Value<'_>
where
    T: Into<Self>,
{
    fn from(t: Vec<T>) -> Self {
        Self::Array(t.into_iter().map(T::into).collect())
    }
}

impl<T, const N: usize> From<[T; N]> for Value<'_>
where
    T: Into<Self>,
{
    fn from(a: [T; N]) -> Self {
        Self::Array(a.into_iter().map(Into::into).collect())
    }
}

impl<'v> Value<'v> {
    pub fn into_owned(self) -> Value<'static> {
        match self {
            Self::Str(s) => Value::Str(Cow::Owned(s.into_owned())),
            Self::Char(c) => Value::Char(c),
            Self::Float(f) => Value::Float(f),
            Self::Integer(i) => Value::Integer(i),
            Value::Array(a) => Value::Array(a.into_iter().map(Value::into_owned).collect()),
            Value::Block(b) => Value::Block(b),
        }
    }

    pub fn and(self, other: Value<'v>) -> Value<'v> {
        bool::from(&self).then(|| other).unwrap_or(self)
    }

    pub fn or(self, other: Value<'v>) -> Value<'v> {
        bool::from(&self).then(|| self).unwrap_or(other)
    }

    pub fn min(self, other: Value<'v>) -> RuntimeResult<'v, Value<'v>> {
        Ok(if self < other { self } else { other })
    }

    pub fn max(self, other: Value<'v>) -> RuntimeResult<'v, Value<'v>> {
        Ok(if other < self { self } else { other })
    }

    pub fn pow(self, other: Value<'v>) -> RuntimeResult<'v, Self> {
        match (&self, &other) {
            (Self::Integer(i1), Self::Integer(i2)) => match (*i2).try_into() {
                Ok(i2) => Ok(Self::Integer(i1.pow(i2))),
                Err(_) if *i2 < 0 => {
                    crate::rt_error!(op: self, other => [pow_with_negative_number])
                }
                Err(_) if *i2 > u32::MAX as i64 => {
                    crate::rt_error!(op: self, other => [pow_too_large])
                }
                Err(_) => unreachable!(),
            },
            (Self::Str(haystack), Self::Str(needle)) => Ok(haystack
                .find(needle.deref())
                .map(|i| i as i64)
                .unwrap_or(-1)
                .into()),
            (Self::Str(haystack), Self::Char(needle)) => Ok(haystack
                .find(*needle)
                .map(|i| i as i64)
                .unwrap_or(-1)
                .into()),
            _ => crate::rt_error!(op: self, other => [pow_substr]),
        }
    }

    pub fn to_char(self) -> RuntimeResult<'v, Self> {
        Ok(Value::Char(match self {
            Value::Char(c) => c,
            Value::Integer(i) => match <i64 as TryInto<u32>>::try_into(i).map(TryInto::try_into) {
                Ok(Ok(c)) => c,
                _ => crate::rt_error!(convert: self, char),
            },
            Value::Float(f) if (0.0..i8::MAX as f64).contains(&f) && f.fract() == 0.0 => {
                f as u8 as _
            }
            _ => crate::rt_error!(convert: self, char),
        }))
    }

    pub fn to_float(self) -> RuntimeResult<'v, Self> {
        Ok(Value::Float(match self {
            Value::Char(c) => match c.is_ascii().then(|| c as u8) {
                Some(c) => c as _,
                None => crate::rt_error!(convert: self, f64),
            },
            Value::Integer(i) => i as f64,
            Value::Float(f) => f,
            _ => crate::rt_error!(convert: self, f64),
        }))
    }

    pub fn to_int(self) -> RuntimeResult<'v, Self> {
        Ok(Value::Integer(match self {
            Value::Char(c) => c as i64,
            Value::Integer(i) => i,
            Value::Float(f) => f as i64,
            Value::Str(ref s) => {
                if let Ok(i) = s.parse() {
                    i
                } else {
                    crate::rt_error!(convert: self, i64)
                }
            }
            _ => crate::rt_error!(convert: self, i64),
        }))
    }

    pub fn to_str(self) -> RuntimeResult<'v, Self> {
        Ok(Value::Str(match self {
            Value::Char(c) => Cow::Owned(c.to_string()),
            Value::Integer(i) => Cow::Owned(i.to_string()),
            Value::Float(f) => Cow::Owned(f.to_string()),
            Value::Str(s) => s,
            Value::Array(a) => {
                let mut s = String::new();
                let _ = write!(s, "{:?}", a);
                Cow::Owned(s)
            }
            Value::Block(_) => crate::rt_error!(convert: self, String),
        }))
    }
}

macro_rules! impl_math {
    ($trait:path, $name:ident) => {
        impl_math!($trait, $name {  });
    };
    ($trait:path, $name:ident {
        $(($left:pat, $right:pat) => $do:expr),* $(,)?
    }) => {

        impl<'v> $trait for Value<'v> {
            type Output = RuntimeResult<'v, Self>;

            fn $name(self, other: Self) -> Self::Output {
                let v = match (self, other) {
                    (Self::Integer(i1), Self::Integer(i2)) => paste::paste! {
                        match i1.[<checked_ $name>](i2).map(Self::Integer) {
                            Some(x) => x,
                            None => crate::rt_error!(op: Self::Integer(i1), Self::Integer(i2) => [$name])
                        }
                    },
                    (Self::Integer(i1), Self::Float(f1)) => Self::Float((i1 as f64).$name(f1)),
                    (Self::Float(f1), Self::Integer(i1)) => Self::Float(f1.$name(i1 as f64)),
                    (Self::Float(f1), Self::Float(f2)) => Self::Float(f1.$name(f2)),
                    $(($left, $right) => $do,)*
                    (a, b) => crate::rt_error!(op: a, b => [$name]),
                };
                Ok(v)
            }
        }
    }
}

impl_math!(ops::Add, add {
    (Self::Char(c), Self::Integer(i)) => Self::Char((c as u8 + i as u8) as char),
    (Self::Str(s1), Self::Str(s2)) => Self::Str(Cow::Owned(s1.into_owned() + &s2)),
    (Self::Str(s), any) => Self::Str(Cow::Owned(
        s.into_owned() + &if let Value::Str(s) = any.to_str()? { s } else { unreachable!() }
    )),
    (Self::Char(c), Self::Str(mut s)) => Self::Str({
        let mut owned = s.into_owned();
        owned.push(c);
        Cow::Owned(owned)
    }),
    (Self::Array(mut a1), Self::Array(a2)) => {
        a1.extend(a2);
        Self::Array(a1)
    },
    (Self::Array(mut a1), any) => {
        a1.push(any);
        Self::Array(a1)
    },
    (any, Self::Array(mut a1)) => {
        a1.insert(0, any);
        Self::Array(a1)
    },
});
impl_math!(ops::Sub, sub {
    (Self::Char(c), Self::Integer(i)) => Self::Char((c as u8 - i as u8) as char),
});
impl_math!(ops::Mul, mul {
    (Self::Array(mut a), Self::Integer(i)) => Self::Array({
        let clone = a.clone();
        for _ in 0..(i.saturating_sub(2)) {
            a.extend(clone.clone());
        }
        a.extend(clone);
        a
    }),
    (Self::Str(s), Self::Integer(i)) => Self::Str(Cow::Owned(s.repeat(i as usize))),
});
impl_math!(ops::Div, div {
    (Self::Str(s), Self::Str(delim)) => Self::Array(s.split(delim.deref()).map(Value::from).collect()),
});
impl_math!(ops::Rem, rem);

macro_rules! impl_bit {
    ($trait:path, $name:ident) => {
        impl_bit!($trait, $name {  });
    };
    ($trait:path, $name:ident {
        $($pattern:pat => $do:expr),* $(,)?
    }) => {

        impl<'v> $trait for Value<'v> {
            type Output = RuntimeResult<'v, Self>;

            fn $name(self, other: Self) -> Self::Output {
                let v = match (self, other) {
                    (Self::Integer(i1), Self::Integer(i2)) => Self::Integer(i1.$name(i2)),
                    $($pattern => $do,)*
                    (a, b) => crate::rt_error!(op: a, b => [$name]),
                };
                Ok(v)
            }
        }
    }
}

impl_bit!(ops::BitAnd, bitand);
impl_bit!(ops::BitOr, bitor);
impl_bit!(ops::BitXor, bitxor);

use either::{Either, Left, Right};

#[derive(Debug, Default, Clone)]
pub struct ExprVec<'s>(pub Vec<Rc<dyn Operator<'s> + 's>>);
#[derive(Debug, Clone)]
pub struct ProtoValue<'s>(pub Either<Value<'s>, ExprVec<'s>>);

impl Default for ProtoValue<'_> {
    fn default() -> Self {
        Self(Left(Value::default()))
    }
}

impl<'v> ProtoValue<'v> {
    pub fn from_str(s: &'v str) -> Option<Self> {
        None.or_else(|| {
            matches!(s.as_bytes(), [b'a'..=b'z'])
                .then(|| s.parse().map(Value::Char).map(Left).ok())
                .flatten()
        })
        .or_else(|| s.parse().map(Value::Integer).map(Left).ok())
        .or_else(|| s.parse().map(Value::Float).map(Left).ok())
        .or_else(|| Value::parse_array(s).map(Right))
        .or_else(|| Value::parse_string(s).map(Left))
        .or_else(|| Value::parse_block(s).map(Left))
        .map(Self)
    }

    // pub fn into_owned(self) -> ProtoValue<'static> {
    //     ProtoValue(match self.0 {
    //         Either::Left(l) => Either::Left(l.into_owned()),
    //         Either::Right(r) => Either::Right(r.into_owned()),
    //     })
    // }
}

impl<'s> Value<'s> {
    fn parse_array(s: &'s str) -> Option<ExprVec<'s>> {
        if s.starts_with('[') && s.ends_with(']') {
            Some(ExprVec(
                s.trim_matches(&['[', ']'][..])
                    .split_tokens()
                    .map(operator_from_str)
                    .map(Result::ok)
                    .map(|x| x.map(Rc::from))
                    .collect::<Option<_>>()?,
            ))
        } else {
            None
        }
    }

    fn parse_string(s: &'s str) -> Option<Self> {
        if s.starts_with('"') && s.ends_with('"') {
            Some(Value::Str(s.trim_matches('"').into()))
        } else {
            None
        }
    }

    fn parse_block(s: &'s str) -> Option<Self> {
        if s.starts_with('{') && s.ends_with('}') {
            Some(Value::Block(
                s.trim_matches(&['{', '}'][..])
                    .split_tokens()
                    .map(|t| operator_from_str(t).map(Rc::from))
                    .map(Result::ok)
                    .collect::<Option<_>>()?,
            ))
        } else {
            None
        }
    }
}

impl fmt::Display for Value<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Char(c) => write!(f, "c({})", c),
            Value::Integer(i) => write!(f, "i({})", i),
            Value::Float(d) => write!(f, "f({})", d),
            Value::Str(s) => write!(f, "s({})", s),
            Value::Array(a) => write!(f, "a([{}])", a.iter().format(",")),
            Value::Block(b) => write!(f, "b([{}])", b.iter().format(",")),
        }
    }
}
impl ExprVec<'_> {
    fn into_owned(self) -> ExprVec<'static> {
        ExprVec(self.0.into_owned())
    }
}
