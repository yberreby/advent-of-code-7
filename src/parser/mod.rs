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
    idx: usize,
}

impl<'input> Parser<'input> {
    pub fn parse(&mut self) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        while let Some(instruction) = self.parse_instruction() {
            instructions.push(instruction);
        }

        instructions
    }

    fn current_token(&self) -> Option<&Token<'input>> {
        self.tokens.get(self.idx)
    }

    fn bump(&mut self) {
        self.idx += 1
    }

    fn idx(&self) -> usize {
        self.idx
    }

    fn parse_instruction(&mut self) -> Option<Instruction> {
        let value = self.parse_value();

        // assignment arrow

        // output wire

        unimplemented!()
    }



    fn parse_value(&mut self) -> Option<Value> {
        match self.current_token() {
            Some(&Token::Integer(x)) => {
                self.bump();
                match self.current_token() {
                    Some(&Token::AssignmentArrow) => Some(Value::Integer(x)),
                    // Some(&Token::Operator(op)) => {}
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }
}

pub fn parse<'input>(tokens: Vec<Token<'input>>) -> Vec<Instruction> {
    let mut parser = Parser {
        tokens: tokens,
        idx: 0,
    };
    parser.parse()
}


mod tests {
    use lexer::Token;
    use super::*;
    use super::Parser;

    #[test]
    fn test_parse_value_integer() {
        let mut parser = Parser {
            tokens: vec![Token::Integer(123), Token::AssignmentArrow, Token::Identifier("aa")],
            idx: 0,
        };

        assert_eq!(parser.parse_value(), Some(Value::Integer(123)));
    }

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
