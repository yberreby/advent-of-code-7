use std::collections::HashMap;

type Wire = String;
type Data = u16;


enum Operation {
    Rshift(Data, Data),
    Lshift(Data, Data),
    Or(Data, Data),
    And(Data, Data),
    Not(Data),
}

impl Operation {
    fn execute(&self) -> Data {
        match *self {
            Operation::Rshift(a, b) => a >> b,
            Operation::Lshift(a, b) => a << b,
            Operation::Or(a, b) => a | b,
            Operation::And(a, b) => a & b,
            Operation::Not(a) => !a,
        }
    }
}

enum InstructionInput {
    Constant(Data),
    Operation(Operation),
}

struct Instruction {
    input: Input,
    output: Wire,
}

// enum Instruction {
//
// }

type Program = Vec<Instruction>;

type OutputSignals = HashMap<String, Data>;

struct Emulator {
    input: Program,
    output: OutputSignals,
}

/*
 what is an instruction?
 a bitwise operator, its operands and the output register / wire
*/

fn main() {
    // x AND y -> d
    let foo = Instruction {
        
        output: "d"
    }
}
