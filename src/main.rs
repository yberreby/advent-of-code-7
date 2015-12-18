// How It Works:
// - parse the string input
// - map each wire to either an operator and its operands (= an operation), or a literal
//
// When the user wants to know the signal on a wire (keyed by String), the emulator recursively
// expands wire signals and caches them

// (?:(?:(\w+) (AND|OR|RSHIFT|LSHIFT) (\w+))|(?:(NOT) (\w+))|(\w+)) -> (\w+)
// that's awful, better have three small regexes

const PROGRAM: &'static str = include_str!("../program.txt");

extern crate regex;
#[macro_use]
extern crate lazy_static;

mod parse;
use parse::{parse_instruction, Instruction};

mod bytecode;
use bytecode::Value;


fn main() {
    let instructions: Vec<Instruction> = PROGRAM.lines()
                                                .map(|l| parse::parse_instruction(l))
                                                .collect();
    let output = run(instructions);

    println!("Full output: {:?}", output);
    println!("Wire 'a': {}", output.get("a").unwrap());

}
