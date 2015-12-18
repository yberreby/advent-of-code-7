use std::collections::HashMap;

pub type WireName = String;
pub type InProgressWires = HashMap<WireName, Value>;


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Operation(Box<Operation>),
    Wire(String),
    Constant(u16),
}

impl Value {
    fn expand(&self, wires: &mut InProgressWires) -> u16 {
        match *self {
            Value::Operation(ref op) => op.execute(wires),
            Value::Wire(ref wire_name) => wires.get(wire_name).unwrap().expand(wires),
            Value::Constant(x) => x,
        }
    }
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
    fn execute(&self, wires: &mut HashMap<WireName, Value>) -> u16 {
        match *self {
            Operation::Rshift(ref a, ref b) => a.expand(wires) >> b.expand(wires),
            Operation::Lshift(ref a, ref b) => a.expand(wires) << b.expand(wires),
            Operation::Or(ref a, ref b) => a.expand(wires) | b.expand(wires),
            Operation::And(ref a, ref b) => a.expand(wires) & b.expand(wires),
            Operation::Not(ref a) => !a.expand(wires),
        }
    }
}


pub fn run(instructions: Vec<Instruction>) -> HashMap<WireName, u16> {
    let mut wires = HashMap::new();

    for instruction in instructions {
        wires.insert(instruction.output_wire, instruction.input);
    }
    unimplemented!()
}
