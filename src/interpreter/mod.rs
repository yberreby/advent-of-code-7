use std::collections::HashMap;
use std::borrow::Cow;
use parser::{Instruction, Value, Operation};

fn evaluate<'input>(value: &Value<'input>,
                    value_map: &HashMap<Cow<'input, str>, Value<'input>>,
                    output_map: &mut HashMap<Cow<'input, str>, u16>)
                    -> u16 {

    match *value {
        Value::Integer(x) => x,
        Value::Wire(ref wire_name) => {
            let name: &str = &wire_name;
            if let Some(v) = output_map.get(name) {
                return *v;
            }

            let res = evaluate(value_map.get(name).unwrap(), value_map, output_map);
            output_map.insert(wire_name.clone(), res);
            res
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

pub fn run<'input>(instructions: Vec<Instruction<'input>>) -> HashMap<Cow<'input, str>, u16> {
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
        let mut expected = HashMap::new();
        expected.insert("ax".into(), 45);

        assert_eq!(run(vec![Instruction {
                                input: Value::Integer(45),
                                output_wire: "ax".into(),
                            }]),
                   expected);
    }

    #[test]
    fn test_run_single_rshift_instruction() {
        let mut expected = HashMap::new();
        expected.insert("zz".into(), 30);

        assert_eq!(run(vec![Instruction {
                                input: Value::Operation(Box::new(Operation::Rshift(
                                                Operand::Integer(123),
                                                Operand::Integer(2)))),
                                output_wire: "zz".into(),
                            }]),
                   expected);
    }
}
