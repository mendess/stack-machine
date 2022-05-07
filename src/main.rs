#![deny(unused_must_use)]

use stack_machine::{run, Repl};

use std::{
    env::args,
    fs,
    io::{self, BufReader},
};

fn main() -> io::Result<()> {
    if let Some(file) = args().nth(1) {
        let f = fs::read_to_string(file)?;
        println!("{:?}", run(&f));
    } else {
        let mut s = String::new();
        let stdin = io::stdin();
        let mut input = BufReader::new(io::stdin());
        let mut repl = Repl::new(&mut input);
        while {
            s.clear();
            stdin.read_line(&mut s)? > 0
        } {
            repl.next_line(&s);
        }
        println!("{:?}", repl.into_vec());
    }
    Ok(())
}
