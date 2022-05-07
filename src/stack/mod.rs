pub mod value;

use crate::error::runtime::*;
use std::{cell::RefCell, fmt, io::BufRead, ops, rc::Rc, slice::SliceIndex};
pub use value::Value;

#[derive(Debug)]
pub struct Variables([Value; (b'Z' - b'A') as usize + 1]);

impl Default for Variables {
    fn default() -> Self {
        let mut vs = Self(Default::default());
        vs['A'] = Value::Integer(10);
        vs['B'] = Value::Integer(11);
        vs['C'] = Value::Integer(12);
        vs['D'] = Value::Integer(13);
        vs['E'] = Value::Integer(14);
        vs['F'] = Value::Integer(15);
        vs['N'] = Value::Char('\n');
        vs['S'] = Value::Char(' ');
        vs['X'] = Value::Integer(0);
        vs['Y'] = Value::Integer(1);
        vs['Z'] = Value::Integer(2);
        vs
    }
}

impl ops::Index<char> for Variables {
    type Output = Value;
    fn index(&self, name: char) -> &Self::Output {
        &self.0[name as usize - b'A' as usize]
    }
}

impl ops::IndexMut<char> for Variables {
    fn index_mut(&mut self, name: char) -> &mut Self::Output {
        &mut self.0[name as usize - b'A' as usize]
    }
}

pub struct Stack<'i> {
    io_input: &'i mut dyn BufRead,
    variables: Rc<RefCell<Variables>>,
    s: Vec<Value>,
}

impl<'i> Stack<'i> {
    pub fn with_input(io_input: &'i mut dyn BufRead) -> Self {
        Self {
            io_input,
            variables: Default::default(),
            s: Default::default(),
        }
    }

    pub fn sub_stack(&mut self) -> Stack<'_> {
        Stack {
            io_input: &mut self.io_input,
            variables: self.variables.clone(),
            s: Default::default(),
        }
    }

    pub fn push(&mut self, v: Value) {
        self.s.push(v)
    }

    pub fn pop(&mut self) -> RuntimeResult<Value> {
        self.s.pop().ok_or(RuntimeError::StackEmpty)
    }

    pub fn top(&self) -> RuntimeResult<&Value> {
        self.s.last().ok_or(RuntimeError::StackEmpty)
    }

    pub fn get<I>(&self, index: I) -> Option<&<I as SliceIndex<[Value]>>::Output>
    where
        I: SliceIndex<[Value]>,
    {
        self.s.get(index)
    }

    pub fn get_from_end(&self, index: usize) -> Option<&Value> {
        self.s.get(self.len().wrapping_sub(index).wrapping_sub(1))
    }

    pub fn get_mut<I>(&mut self, index: I) -> Option<&mut <I as SliceIndex<[Value]>>::Output>
    where
        I: SliceIndex<[Value]>,
    {
        self.s.get_mut(index)
    }

    pub fn len(&self) -> usize {
        self.s.len()
    }

    pub fn is_empty(&self) -> bool {
        self.s.is_empty()
    }

    pub fn into_vec(self) -> Vec<Value> {
        self.s
    }

    pub fn take(&mut self) -> Vec<Value> {
        std::mem::take(&mut self.s)
    }

    pub fn as_slice(&self) -> &[Value] {
        &self.s
    }

    pub fn input(&mut self) -> &mut dyn BufRead {
        &mut *self.io_input
    }

    pub fn take_as_value(&mut self) -> RuntimeResult<Value> {
        match self.s.len() {
            0 => Err(RuntimeError::StackEmpty),
            1 => Ok(self.s.pop().unwrap()),
            _ => Ok(self.take().into()),
        }
    }

    pub fn push_var(&mut self, var: char) {
        self.s.push(self.variables.borrow()[var].clone());
    }

    pub fn pop_var(&mut self, var: char) -> RuntimeResult<()> {
        self.variables.borrow_mut()[var] = self.top().map(Clone::clone)?;
        Ok(())
    }
}

impl fmt::Display for Stack<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for v in &self.s[..(self.len() - 1)] {
            write!(f, "{}, ", v)?;
        }
        if let Some(x) = self.s.last() {
            write!(f, "{}", x)?;
        }
        write!(f, "]")
    }
}
