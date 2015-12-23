use FastHashMap;
use parser::{Instruction, Value, Operation};

#[cfg(test)]
mod tests;

fn evaluate<'input>(value: &Value<'input>,
                    value_map: &FastHashMap<&'input str, Value<'input>>,
                    output_map: &mut FastHashMap<&'input str, u16>)
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

pub fn run<'input>(instructions: Vec<Instruction<'input>>) -> FastHashMap<&'input str, u16> {
    let mut value_map = FastHashMap::default();

    for instruction in instructions {
        value_map.insert(instruction.output_wire, instruction.input);
    }

    let mut output_map = FastHashMap::default();

    for (key, value) in &value_map {
        let result = evaluate(value, &value_map, &mut output_map);
        output_map.insert(key.clone(), result);
    }

    output_map
}
