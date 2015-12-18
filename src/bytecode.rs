



pub fn run(instructions: Vec<Instruction>) -> HashMap<WireName, u16> {
    let mut wires = HashMap::new();

    for instruction in instructions {
        wires.insert(instruction.output_wire, instruction.input);
    }
    unimplemented!()
}
