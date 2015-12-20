use std::collections::HashMap;
use parser::{Instruction, Value, Operation};

fn evaluate(value: &Value,
            value_map: &HashMap<String, Value>,
            output_map: &mut HashMap<String, u16>)
            -> u16 {
    match *value {
        Value::Integer(x) => x,
        Value::Wire(_) => evaluate(value, value_map, output_map),
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
    use parser::{Instruction, Value};
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
}
