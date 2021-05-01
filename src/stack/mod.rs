pub mod value;

use std::{
    fmt,
    io::{self, BufRead /*Write*/, BufReader},
    ops,
    slice::SliceIndex,
};
pub use value::Value;

pub struct Stack {
    io_input: Box<dyn BufRead>,
    // io_output: Box<dyn Write>,
    variables: [Value; (b'Z' - b'A') as usize + 1],
    s: Vec<Value>,
}

impl ops::Index<char> for Stack {
    type Output = Value;
    fn index(&self, name: char) -> &Self::Output {
        &self.variables[name as usize - b'A' as usize]
    }
}

impl ops::IndexMut<char> for Stack {
    fn index_mut(&mut self, name: char) -> &mut Self::Output {
        &mut self.variables[name as usize - b'A' as usize]
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

impl Stack {
    pub fn new() -> Self {
        Self::with_input(BufReader::new(io::stdin()))
    }

    pub fn with_input<I: BufRead + 'static>(i: I) -> Self {
        let variables = Default::default();
        let mut s = Self {
            io_input: Box::new(i),
            // io_output: Box::new(io::stdout()),
            variables,
            s: Default::default(),
        };
        s['A'] = Value::Integer(10);
        s['B'] = Value::Integer(11);
        s['C'] = Value::Integer(12);
        s['D'] = Value::Integer(13);
        s['E'] = Value::Integer(14);
        s['F'] = Value::Integer(15);
        s['N'] = Value::Char('\n');
        s['S'] = Value::Char(' ');
        s['X'] = Value::Integer(0);
        s['Y'] = Value::Integer(1);
        s['Z'] = Value::Integer(2);
        s
    }

    pub fn push(&mut self, v: Value) {
        self.s.push(v)
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.s.pop()
    }

    pub fn top(&self) -> Option<&Value> {
        self.s.last()
    }

    // pub fn top_mut(&mut self) -> Option<&mut Value> {
    //     self.s.last_mut()
    // }

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

    pub fn into_vec(self) -> Vec<Value> {
        self.s
    }

    pub fn as_slice(&self) -> &[Value] {
        &self.s
    }

    pub fn input(&mut self) -> &mut dyn BufRead {
        &mut *self.io_input
    }

    // pub fn output(&mut self) -> &mut dyn Write {
    //     &mut *self.io_output
    // }
}

impl fmt::Display for Stack {
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
