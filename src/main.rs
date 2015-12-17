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
use regex::Regex;

#[macro_use]
extern crate lazy_static;
lazy_static! {
    static ref ASSIGN_REGEX: Regex = Regex::new(r"^(\w+) -> (\w+)$").unwrap();
    static ref NOT_REGEX: Regex = Regex::new(r"^NOT (\w+) -> (\w+)$").unwrap();
    static ref BINARY_OP_REGEX: Regex = Regex::new(r"^(\w+) ([A-Z]+) (\w+) -> [a-z]+$").unwrap();
}


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
enum Value {
    Operation(Box<Operation>),
    Wire(String),
    Constant(u16),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Operation {
    Rshift(Value, Value),
    Lshift(Value, Value),
    Or(Value, Value),
    And(Value, Value),
    Not(Value),
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
    input: Value,
    output_wire: WireName,
}

// Grammar:
// instruction: <value> -> <wire>
// value: <value> <operator> <value>
//        <operator> <value>
//        <wire>
//        <literal>


fn parse_instruction(s: &str) -> Instruction {
    unimplemented!()
}

#[test]
fn test_parse_instruction() {
    assert_eq!(parse_instruction("bn RSHIFT 2 -> bo"),
               Instruction {
                   input: Value::Operation(Box::new(Operation::Rshift(Value::Wire("bn".into()),
                                                                      Value::Constant(2)))),
                   output_wire: "bo".into(),
               });

    assert_eq!(parse_instruction("NOT lo -> lp"),
               Instruction {
                   input: Value::Operation(Box::new(Operation::Not(Value::Wire("lo".into())))),
                   output_wire: "lp".into(),
               });

    assert_eq!(parse_instruction("lx -> a"),
               Instruction {
                   input: Value::Wire("lx".into()),
                   output_wire: "a".into(),
               });

    assert_eq!(parse_instruction("123 -> xy"),
               Instruction {
                   input: Value::Constant(123),
                   output_wire: "xy".into(),
               });
}


fn run(instructions: Vec<Instruction>) -> HashMap<WireName, u16> {
    let mut wires = HashMap::new();

    for instruction in instructions {
        wires.insert(instruction.output_wire, instruction.operation);
    }
    unimplemented!()
}

fn main() {
    let instructions: Vec<Instruction> = PROGRAM.lines().map(|l| parse_instruction(l)).collect();
    let output = run(instructions);

    println!("Full output: {:?}", output);
    println!("Wire 'a': {}", output.get("a").unwrap());

}
