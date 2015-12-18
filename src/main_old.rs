use std::collections::HashMap;

type Wire = String;
type Data = u16;



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
