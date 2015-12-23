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
