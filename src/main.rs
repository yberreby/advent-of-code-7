// How It Works:
// - parse the string input
// - map each wire to either an operator and its operands (= an operation), or a literal
//
// When the user wants to know the signal on a wire (keyed by String), the emulator recursively
// expands wire signals and caches them

const PROGRAM: &'static str = include_str!("../program.txt");

use std::collections::HashMap;

type WireName = String;
type Wires = HashMap<WireName, Signal>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Signal {
    Computed(u16),
    Operation(Box<Operation>),
}

impl Signal {
    fn expand(&self, wires: &mut Wires) -> u16 {
        match *self {
            Signal::Computed(x) => x,
            Signal::Operation(ref op) => op.execute(wires),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Operation {
    Rshift(Signal, Signal),
    Lshift(Signal, Signal),
    Or(Signal, Signal),
    And(Signal, Signal),
    Not(Signal),
}

impl Operation {
    fn execute(&self, wires: &mut Wires) -> u16 {
        match *self {
            Operation::Rshift(ref a, ref b) => a.expand(wires) >> b.expand(wires),
            Operation::Lshift(ref a, ref b) => a.expand(wires) << b.expand(wires),
            Operation::Or(ref a, ref b) => a.expand(wires) | b.expand(wires),
            Operation::And(ref a, ref b) => a.expand(wires) & b.expand(wires),
            Operation::Not(ref a) => !a.expand(wires),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Instruction {
    op: Operation,
    output: WireName,
}


fn parse_instruction(s: &str) -> Instruction {
    unimplemented!()
}


fn run(instructions: Vec<Instruction>) -> HashMap<WireName, u16> {
    unimplemented!()
}

fn main() {
    let instructions: Vec<Instruction> = PROGRAM.lines().map(|l| parse_instruction(l)).collect();
    let output = run(instructions);

    println!("Full output: {:?}", output);
    println!("Wire 'a': {}", output.get("a").unwrap());

}
