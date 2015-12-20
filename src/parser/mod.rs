use lexer::Token;

pub type Ast = Vec<Instruction>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    input: Value,
    output_wire: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Operation(Box<Operation>),
    Wire(String),
    Integer(u16),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    Rshift(Value, Value),
    Lshift(Value, Value),
    Or(Value, Value),
    And(Value, Value),
    Not(Value),
}

pub fn parse(tokens: Vec<Token>) -> Ast {
    unimplemented!()
}


mod tests {
    use lexer::Token;
    use super::*;

    #[test]
    fn test_parse_simple_instruction() {
        let tokens = vec![Token::Integer(123), Token::AssignmentArrow, Token::Identifier("ax")];

        assert_eq!(parse(tokens),
                   vec![Instruction {
                            input: Value::Integer(123),
                            output_wire: "ax".into(),
                        }]);
    }

}
