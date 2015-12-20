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

use std::cell::Cell;

struct Parser<'input> {
    tokens: Vec<Token<'input>>,
    idx: Cell<usize>,
}

impl<'input> Parser<'input> {
    pub fn new(tokens: Vec<Token<'input>>) -> Parser<'input> {
        Parser {
            tokens: tokens,
            idx: Cell::new(0),
        }
    }

    pub fn parse(&self) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        while let Some(instruction) = self.parse_instruction() {
            instructions.push(instruction);
        }

        instructions
    }

    fn current_token(&self) -> Option<&Token<'input>> {
        self.tokens.get(self.idx())
    }

    fn bump(&self) {
        self.idx.set(self.idx() + 1)
    }

    fn idx(&self) -> usize {
        self.idx.get()
    }

    fn parse_instruction(&self) -> Option<Instruction> {
        let value = match self.parse_value() {
            Some(v) => v,
            None => return None,
        }; // what to do on None?..;

        // assignment arrow
        match self.current_token() {
            Some(&Token::AssignmentArrow) => {
                self.bump();
            }
            other => panic!("expected assignment arrow, found {:?}", other),
        }

        let output_wire = match self.current_token() {
            Some(&Token::Identifier(id)) => id,
            other => panic!("expected identifier, found {:?}", other),
        };

        self.bump();

        Some(Instruction {
            input: value,
            output_wire: output_wire.into(),
        })
    }

    fn parse_value(&self) -> Option<Value> {
        match self.current_token() {
            Some(&Token::Integer(x)) => {
                self.bump();
                match self.current_token() {
                    Some(&Token::AssignmentArrow) => Some(Value::Integer(x)),
                    // Some(&Token::Operator(op)) => {}
                    _ => panic!(),
                }
            }
            None => None,
            other => panic!("unexpected {:?}", other),
        }
    }
}

pub fn parse<'input>(tokens: Vec<Token<'input>>) -> Vec<Instruction> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}


mod tests {
    use lexer::Token;
    use super::*;
    use super::Parser;

    #[test]
    fn test_parse_value_integer() {
        let mut parser = Parser::new(vec![Token::Integer(123),
                                          Token::AssignmentArrow,
                                          Token::Identifier("aa")]);

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
