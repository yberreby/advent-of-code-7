use std::collections::HashMap;
use parser::{Instruction, Value, Operation};

fn evaluate(value: &Value,
            value_map: &HashMap<String, Value>,
            output_map: &mut HashMap<String, u16>)
            -> u16 {
    match *value {
        Value::Integer(x) => x,
        Value::Wire(ref wire_name) => {
            evaluate(value_map.get(wire_name).unwrap(), value_map, output_map)
        }
        Value::Operation(ref op) => {
            match **op {
                Operation::Rshift(ref a, ref b) => {
                    evaluate(&Value::from_operand(a.clone()), value_map, output_map) >>
                    evaluate(&Value::from_operand(b.clone()), value_map, output_map)
                }
                Operation::Lshift(ref a, ref b) => {
                    evaluate(&Value::from_operand(a.clone()), value_map, output_map) <<
                    evaluate(&Value::from_operand(b.clone()), value_map, output_map)
                }
                Operation::And(ref a, ref b) => {
                    evaluate(&Value::from_operand(a.clone()), value_map, output_map) &
                    evaluate(&Value::from_operand(b.clone()), value_map, output_map)
                }
                Operation::Or(ref a, ref b) => {
                    evaluate(&Value::from_operand(a.clone()), value_map, output_map) |
                    evaluate(&Value::from_operand(b.clone()), value_map, output_map)
                }
                Operation::Not(ref a) => {
                    !evaluate(&Value::from_operand(a.clone()), value_map, output_map)
                }
            }
        }
    }
}

pub fn run(instructions: Vec<Instruction>) -> HashMap<String, u16> {
    let mut value_map = HashMap::new();

    for instruction in instructions {
        value_map.insert(instruction.output_wire, instruction.input);
    }

    let mut output_map = HashMap::new();

    for (key, value) in &value_map {
        let result = evaluate(value, &value_map, &mut output_map);
        output_map.insert(key.clone(), result);
    }

    output_map
}

mod tests {
    use super::*;
    use parser::{Instruction, Value, Operation, Operand};
    use std::collections::HashMap;

    #[test]
    fn test_run_single_constant_instruction() {
        let mut expected: HashMap<String, u16> = HashMap::new();
        expected.insert("ax".into(), 45);

        assert_eq!(run(vec![Instruction {
                                input: Value::Integer(45),
                                output_wire: "ax".into(),
                            }]),
                   expected);
    }

    #[test]
    fn test_run_single_rshift_instruction() {
        let mut expected: HashMap<String, u16> = HashMap::new();
        expected.insert("zz".into(), 30);

        assert_eq!(run(vec![Instruction {
                                input: Value::Operation(Box::new(Operation::Rshift(
                                                Operand::Integer(123),
                                                Operand::Integer(2)))),
                                output_wire: "zz".into(),
                            }]),
                   expected);
    }

    #[test]
    fn test_run_full_program() {
        let program_source = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT \
                              2 -> g
NOT x -> h
NOT y -> i";

        let tokens = ::lexer::lex(program_source);
        let instructions = ::parser::parse(tokens);
        let output = run(instructions);

        //
        // d: 72
        // e: 507
        // f: 492
        // g: 114
        // h: 65412
        // i: 65079
        // x: 123
        // y: 456
        //

        let mut expected = HashMap::new();
        expected.insert("d".into(), 72);
        expected.insert("e".into(), 507);
        expected.insert("f".into(), 492);
        expected.insert("g".into(), 114);
        expected.insert("h".into(), 65412);
        expected.insert("i".into(), 65079);
        expected.insert("x".into(), 123);
        expected.insert("y".into(), 456);

        assert_eq!(output, expected);
    }
}
