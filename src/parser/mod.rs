use lexer::Token;

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

struct Parser<'input> {
    tokens: Vec<Token<'input>>,
}

impl<'input> Parser<'input> {
    pub fn parse(&mut self) -> Vec<Instruction> {
        unimplemented!()
    }

    fn parse_instruction(&mut self) -> Option<Instruction> {
        unimplemented!()
    }

    fn parse_value(&mut self) -> Option<Value> {
        unimplemented!()
    }
}

pub fn parse<'input>(tokens: Vec<Token<'input>>) -> Vec<Instruction> {
    let mut parser = Parser { tokens: tokens };
    parser.parse()
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
