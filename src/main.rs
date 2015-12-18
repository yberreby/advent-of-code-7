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



use std::collections::HashMap;

pub type WireName = String;
pub type InProgressWires = HashMap<WireName, Value>;



#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Operation(Box<Operation>),
    Wire(String),
    Constant(u16),
}

// impl Value {
//     fn expand(&self, wires: &mut InProgressWires) -> u16 {
//         match *self {
//             Value::Operation(ref op) => op.execute(wires),
//             Value::Wire(ref wire_name) => wires.get(wire_name).unwrap().expand(wires),
//             Value::Constant(x) => x,
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    Rshift(Value, Value),
    Lshift(Value, Value),
    Or(Value, Value),
    And(Value, Value),
    Not(Value),
}

// impl Operation {
//     fn execute(&self, wires: &mut HashMap<WireName, Value>) -> u16 {
//         match *self {
//             Operation::Rshift(ref a, ref b) => a.expand(wires) >> b.expand(wires),
//             Operation::Lshift(ref a, ref b) => a.expand(wires) << b.expand(wires),
//             Operation::Or(ref a, ref b) => a.expand(wires) | b.expand(wires),
//             Operation::And(ref a, ref b) => a.expand(wires) & b.expand(wires),
//             Operation::Not(ref a) => !a.expand(wires),
//         }
//     }
// }


mod parse;
use parse::{parse_instruction, Instruction};

// mod bytecode;
// use bytecode::Value;


fn main() {
    // let instructions: Vec<Instruction> = PROGRAM.lines()
    //                                             .map(|l| parse::parse_instruction(l))
    //                                             .collect();
    // let output = run(instructions);

    // println!("Full output: {:?}", output);
    // println!("Wire 'a': {}", output.get("a").unwrap());

}
