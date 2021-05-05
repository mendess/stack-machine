use crate::Value;

pub enum OneOrMore<T> {
    One(T),
    More(Vec<T>),
}

impl Into<Value> for OneOrMore<Value> {
    fn into(self) -> Value {
        match self {
            Self::One(v) => v,
            Self::More(vs) => vs.into(),
        }
    }
}
