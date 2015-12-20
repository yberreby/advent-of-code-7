use lexer::Token;
use lexer::Operator;

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
pub enum Operand {
    Integer(u16),
    Wire(String),
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    Rshift(Operand, Operand),
    Lshift(Operand, Operand),
    Or(Operand, Operand),
    And(Operand, Operand),
    Not(Operand),
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
            Some(&Token::Identifier(id)) => {
                self.bump();
                id
            }
            other => panic!("expected identifier, found {:?}", other),
        };

        Some(Instruction {
            input: value,
            output_wire: output_wire.into(),
        })
    }

    fn parse_value(&self) -> Option<Value> {
        match self.current_token() {
            Some(&Token::Integer(_)) | Some(&Token::Identifier(_)) => {
                let a = self.parse_operand().unwrap();

                match self.current_token() {
                    Some(&Token::Operator(ref operator)) => {
                        self.bump();

                        let b = self.parse_operand().unwrap();

                        let operation = match *operator {
                            Operator::Lshift => Operation::Lshift(a, b),
                            Operator::Rshift => Operation::Rshift(a, b),
                            Operator::And => Operation::And(a, b),
                            Operator::Or => Operation::Or(a, b),
                            _ => panic!(),
                        };

                        Some(Value::Operation(Box::new(operation)))
                    }
                    Some(&Token::AssignmentArrow) => {
                        match a {
                            Operand::Integer(x) => Some(Value::Integer(x)),
                            Operand::Wire(x) => Some(Value::Wire(x)),
                        }
                    }
                    other => {
                        panic!("expected an operator or an assignment arrow, found {:?}",
                               other)
                    }
                }
            }
            Some(&Token::Operator(Operator::Not)) => {
                self.bump();

                let a = self.parse_operand().unwrap();
                Some(Value::Operation(Box::new(Operation::Not(a))))
            }
            None => None,
            other => panic!("unexpected {:?}", other),
        }

        // match self.current_token() {
        //     Some(&Token::Integer(x1)) => {
        //         self.bump();
        //         match self.current_token() {
        //             Some(&Token::AssignmentArrow) => Some(Value::Integer(x1)),
        //             Some(&Token::Operator(Operator::Lshift)) => {
        //                 self.bump();

        //                 match self.current_token() {
        //                     Some(&Token::Integer(x2)) => {
        //                         Some(Value::Operation(Box::new(Operation::Lshift(Value::Integer(x1), Value::Integer(x2)))))
        //                     }
        //                     _ => panic!(),
        //                 }
        //             }
        //             _ => panic!(),
        //         }
        //     }
        //     None => None,
        //     other => panic!("unexpected {:?}", other),
        // }
    }

    fn parse_operand(&self) -> Option<Operand> {
        let op = match self.current_token() {
            Some(&Token::Integer(x)) => Some(Operand::Integer(x)),
            Some(&Token::Identifier(id)) => Some(Operand::Wire(id.into())),
            _ => panic!(),
        };

        self.bump();

        op
    }
}

pub fn parse<'input>(tokens: Vec<Token<'input>>) -> Vec<Instruction> {
    let parser = Parser::new(tokens);
    parser.parse()
}


mod tests {
    use lexer::Token;
    use lexer::Operator;
    use super::*;
    use super::Parser;

    #[test]
    fn test_parse_value_integer() {
        let parser = Parser::new(vec![Token::Integer(123),
                                      Token::AssignmentArrow,
                                      Token::Identifier("aa")]);

        assert_eq!(parser.parse_value(), Some(Value::Integer(123)));
    }

    #[test]
    fn test_parse_constant_assignment_instruction() {
        let tokens = vec![Token::Integer(123), Token::AssignmentArrow, Token::Identifier("ax")];

        assert_eq!(parse(tokens),
                   vec![Instruction {
                            input: Value::Integer(123),
                            output_wire: "ax".into(),
                        }]);
    }

    #[test]
    fn test_parse_not_instruction() {
        let tokens = vec![Token::Operator(Operator::Not),
                          Token::Integer(123),
                          Token::AssignmentArrow,
                          Token::Identifier("ax")];

        assert_eq!(parse(tokens),
                   vec![Instruction {
                            input: Value::Operation(Box::new(Operation::Not(Operand::Integer(123)))),
                            output_wire: "ax".into(),
                        }]);
    }
}
