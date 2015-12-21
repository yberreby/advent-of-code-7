use lexer::Token;
use lexer::Operator;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction<'a> {
    pub input: Value<'a>,
    pub output_wire: Cow<'a, str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value<'a> {
    Operation(Box<Operation<'a>>),
    Wire(Cow<'a, str>),
    Integer(u16),
}

impl<'a> Value<'a> {
    pub fn from_operand(operand: Operand<'a>) -> Value<'a> {
        match operand {
            Operand::Integer(x) => Value::Integer(x),
            Operand::Wire(s) => Value::Wire(s.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand<'a> {
    Integer(u16),
    Wire(Cow<'a, str>),
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation<'a> {
    Rshift(Operand<'a>, Operand<'a>),
    Lshift(Operand<'a>, Operand<'a>),
    Or(Operand<'a>, Operand<'a>),
    And(Operand<'a>, Operand<'a>),
    Not(Operand<'a>),
}

use std::cell::Cell;
use std::iter::Peekable;

pub struct Parser<'input, I: Iterator<Item = Token<'input>>> {
    tokens: Peekable<I>,
    __phantom: ::std::marker::PhantomData<&'input i32>,
}

impl<'input, I: Iterator<Item = Token<'input>>> Parser<'input, I> {
    pub fn new(tokens: I) -> Parser<'input, I> {
        Parser {
            tokens: tokens.peekable(),
            __phantom: ::std::marker::PhantomData,
        }
    }

    pub fn parse(&mut self) -> Vec<Instruction<'input>> {
        let mut instructions = Vec::new();

        loop {
            if let Some(instruction) = self.parse_instruction() {
                instructions.push(instruction);
            } else {
                break;
            }

            match self.tokens.next() {
                Some(Token::Newline) => {}
                None => break,
                other => panic!("unexpected token {:?}", other),
            }
        }

        instructions
    }

    fn parse_instruction(&mut self) -> Option<Instruction<'input>> {
        let value = match self.parse_value() {
            Some(v) => v,
            None => return None,
        }; // what to do on None?..;

        // assignment arrow
        match self.tokens.next() {
            Some(Token::AssignmentArrow) => {}
            other => panic!("expected assignment arrow, found {:?}", other),
        }

        let output_wire = match self.tokens.next() {
            Some(Token::Identifier(id)) => id,
            other => panic!("expected identifier, found {:?}", other),
        };

        Some(Instruction {
            input: value,
            output_wire: output_wire.into(),
        })
    }

    fn parse_value(&mut self) -> Option<Value<'input>> {
        let next = match self.tokens.peek() {
            Some(t) => t.clone(),
            None => return None,
        };

        if is_operand(&next) {
            let a = self.parse_operand().unwrap();

            let next = match self.tokens.peek() {
                Some(t) => t.clone(),
                None => panic!(),
            };

            match next {
                Token::Operator(ref operator) => {
                    self.tokens.next();

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
                Token::AssignmentArrow => {
                    match a {
                        Operand::Integer(x) => Some(Value::Integer(x)),
                        Operand::Wire(x) => Some(Value::Wire(x.into())),
                    }
                }
                other => {
                    panic!("expected an operator or an assignment arrow, found {:?}",
                           other)
                }
            }
        } else if let Token::Operator(Operator::Not) = next {
            self.tokens.next();

            let a = self.parse_operand().unwrap();
            Some(Value::Operation(Box::new(Operation::Not(a))))
        } else {
            panic!("unexpected token {:?}", next)
        }
    }

    fn parse_operand(&mut self) -> Option<Operand<'input>> {
        let op = match self.tokens.next() {
            Some(Token::Integer(x)) => Some(Operand::Integer(x)),
            Some(Token::Identifier(id)) => Some(Operand::Wire(id.into())),
            _ => panic!(),
        };

        op
    }
}

fn is_operand(token: &Token) -> bool {
    match *token {
        Token::Integer(_) | Token::Identifier(_) => true,
        _ => false,
    }
}

pub fn parse<'input>(tokens: Vec<Token<'input>>) -> Vec<Instruction> {
    let mut parser = Parser::new(tokens.into_iter());
    parser.parse()
}


mod tests {
    use lexer::Token;
    use lexer::Operator;
    use super::*;

    #[test]
    fn test_parse_value_integer() {
        let mut parser = Parser::new(vec![Token::Integer(123),
                                          Token::AssignmentArrow,
                                          Token::Identifier("aa")]
                                         .into_iter());

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

    #[test]
    fn test_parse_two_instructions() {
        let tokens = vec![Token::Integer(123),
                          Token::AssignmentArrow,
                          Token::Identifier("aa"),
                          Token::Newline,
                          Token::Integer(456),
                          Token::AssignmentArrow,
                          Token::Identifier("zz")];

        assert_eq!(parse(tokens),
                   vec![Instruction {
                            input: Value::Integer(123),
                            output_wire: "aa".into(),
                        },

                        Instruction {
                            input: Value::Integer(456),
                            output_wire: "zz".into(),
                        }]);
    }
}
