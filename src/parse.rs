use serde::iter::LineColIterator;
use std::io;
use Value;
use Operation;
use WireName;

use super::error::{ParseError, ParseErrorCode, ParseResult};


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    input: Value,
    output_wire: WireName,
}

// Grammar:
// instruction: <value> -> <wire>
// value: <value> <operator> <value>
//        <operator> <value>
//        <wire>
//        <literal>

// Utility functions.

fn is_ascii_lowercase(c: u8) -> bool {
    97 <= c && c <= 122
}

#[test]
fn test_is_ascii_lowercase() {
    assert!(is_ascii_lowercase(b'a'));
    assert!(is_ascii_lowercase(b'd'));
    assert!(is_ascii_lowercase(b'z'));
    assert!(!is_ascii_lowercase(b'A'));
    assert!(!is_ascii_lowercase(b'T'));
    assert!(!is_ascii_lowercase(b'Z'));
}

fn is_ascii_uppercase(c: u8) -> bool {
    65 <= c && c <= 90
}

#[test]
fn test_is_ascii_uppercase() {
    assert!(!is_ascii_uppercase(b'a'));
    assert!(!is_ascii_uppercase(b'd'));
    assert!(!is_ascii_uppercase(b'z'));
    assert!(is_ascii_uppercase(b'A'));
    assert!(is_ascii_uppercase(b'T'));
    assert!(is_ascii_uppercase(b'Z'));
}

pub struct Parser<I: Iterator<Item = io::Result<u8>>> {
    input: LineColIterator<I>,
    current_char: Option<u8>,
}

impl<I: Iterator<Item = io::Result<u8>>> Parser<I> {
    pub fn new(iter: I) -> Parser<I> {
        Parser {
            input: LineColIterator::new(iter),
            current_char: None,
        }
    }

    fn parse(&mut self) -> ParseResult<Instruction> {
        let instruction_input = try!(self.parse_value());
        try!(self.parse_assignment_arrow());
        let output_wire = try!(self.parse_wire_name());

        Ok(Instruction {
            input: instruction_input,
            output_wire: output_wire,
        })
    }

    fn next_char(&mut self) -> ParseResult<Option<u8>> {
        match self.current_char.take() {
            Some(ch) => Ok(Some(ch)),
            None => {
                match self.input.next() {
                    Some(Err(err)) => Err(ParseError::IoError(err)),
                    Some(Ok(ch)) => Ok(Some(ch)),
                    None => Ok(None),
                }
            }
        }
    }

    fn peek(&mut self) -> ParseResult<Option<u8>> {
        match self.current_char {
            Some(ch) => Ok(Some(ch)),
            None => {
                match self.input.next() {
                    Some(Err(err)) => Err(ParseError::IoError(err)),
                    Some(Ok(ch)) => {
                        self.current_char = Some(ch);
                        Ok(self.current_char)
                    }
                    None => Ok(None),
                }
            }
        }
    }

    fn parse_value(&mut self) -> ParseResult<Value> {
        match try!(self.peek()) {
            Some(c) if is_ascii_lowercase(c) => {
                let wire_name = try!(self.parse_wire_name());
                try!(self.parse_whitespace());

                match try!(self.peek()) {
                    Some(c) if is_ascii_uppercase(c) => {}
                    _ => panic!(),
                }

                unimplemented!()
            }
            _ => panic!(),
        }
    }

    fn parse_whitespace(&mut self) -> ParseResult<()> {
        unimplemented!()
    }

    fn parse_wire_name(&mut self) -> ParseResult<String> {
        unimplemented!()
    }

    fn parse_assignment_arrow(&mut self) -> ParseResult<()> {
        unimplemented!()
    }
}


pub fn parse_instruction(s: &str) -> Instruction {
    Parser::new(s.bytes().map(|c| Ok(c))).parse().unwrap()
}

pub fn parse_program(program: &str) -> Vec<Instruction> {
    program.lines().map(|l| parse_instruction(l)).collect()
}


// Tests

#[test]
fn test_parse_constant_instruction() {
    assert_eq!(parse_instruction("123 -> xy"),
               Instruction {
                   input: Value::Constant(123),
                   output_wire: "xy".into(),
               });
}

#[test]
fn test_parse_wire_instruction() {
    assert_eq!(parse_instruction("lx -> a"),
               Instruction {
                   input: Value::Wire("lx".into()),
                   output_wire: "a".into(),
               });
}

#[test]
fn test_parse_NOT_instruction() {
    assert_eq!(parse_instruction("NOT lo -> lp"),
               Instruction {
                   input: Value::Operation(Box::new(Operation::Not(Value::Wire("lo".into())))),
                   output_wire: "lp".into(),
               });

}

#[test]
fn test_parse_RSHIFT_instruction() {
    assert_eq!(parse_instruction("bn RSHIFT 2 -> bo"),
               Instruction {
                   input: Value::Operation(Box::new(Operation::Rshift(Value::Wire("bn".into()),
                                                                      Value::Constant(2)))),
                   output_wire: "bo".into(),
               });
}


#[test]
fn test_parse_LSHIFT_instruction() {
    assert_eq!(parse_instruction("bn LSHIFT 2 -> bo"),
               Instruction {
                   input: Value::Operation(Box::new(Operation::Lshift(Value::Wire("bn".into()),
                                                                      Value::Constant(2)))),
                   output_wire: "bo".into(),
               });
}


#[test]
fn test_parse_AND_instruction() {
    assert_eq!(parse_instruction("bn AND 2 -> bo"),
               Instruction {
                   input: Value::Operation(Box::new(Operation::And(Value::Wire("bn".into()),
                                                                   Value::Constant(2)))),
                   output_wire: "bo".into(),
               });
}


#[test]
fn test_parse_OR_instruction() {
    assert_eq!(parse_instruction("bn OR 2 -> bo"),
               Instruction {
                   input: Value::Operation(Box::new(Operation::Or(Value::Wire("bn".into()),
                                                                  Value::Constant(2)))),
                   output_wire: "bo".into(),
               });
}
