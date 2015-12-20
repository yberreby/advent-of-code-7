use std::collections::HashMap;
use parser::{Instruction, Value};

pub fn evaluate(instructions: Vec<Instruction>) -> HashMap<String, u16> {
    unimplemented!()
}

mod tests {
    use super::*;
    use parser::{Instruction, Value};
    use std::collections::HashMap;

    #[test]
    fn test_evaluate_single_constant_instruction() {
        let mut expected: HashMap<String, u16> = HashMap::new();
        expected.insert("ax".into(), 45);

        assert_eq!(evaluate(vec![Instruction {
                                     input: Value::Integer(45),
                                     output_wire: "ax".into(),
                                 }]),
                   expected);
    }
}
