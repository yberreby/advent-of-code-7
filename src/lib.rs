#![feature(hashmap_hasher)]
#![deny(overflowing_literals)]

extern crate fnv;

mod lexer;
mod parser;
mod interpreter;

#[cfg(test)]
mod tests;

use std::collections::HashMap;
use lexer::Lexer;
use parser::Parser;

use std::collections::hash_state::DefaultState;
use fnv::FnvHasher;

pub type FastHashMap<K, V> = HashMap<K, V, DefaultState<FnvHasher>>;

pub fn run_source<'input>(program_source: &'input str) -> FastHashMap<&'input str, u16> {
    let tokens = Lexer::new(program_source.as_bytes());
    let instructions = Parser::new(tokens).parse();
    interpreter::run(instructions)
}
