use serde::iter::LineColIterator;
use std::io;
use Value;
use Operation;
use WireName;

use super::error::{Error, ErrorCode, ParseResult};


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

fn is_ascii_number(c: u8) -> bool {
    48 <= c && c <= 57
}

#[test]
fn test_is_ascii_number() {
    assert!(is_ascii_number(b'0'));
    assert!(is_ascii_number(b'5'));
    assert!(is_ascii_number(b'9'));
    assert!(!is_ascii_number(b'a'));
    assert!(!is_ascii_number(b'Z'));
    assert!(!is_ascii_number(b'y'));
}

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
                    Some(Err(err)) => Err(Error::IoError(err)),
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
                    Some(Err(err)) => Err(Error::IoError(err)),
                    Some(Ok(ch)) => {
                        self.current_char = Some(ch);
                        Ok(self.current_char)
                    }
                    None => Ok(None),
                }
            }
        }
    }

    fn syntax_error(&self, code: ErrorCode) -> Error {
        Error::SyntaxError(code, self.input.line(), self.input.col())
    }

    fn parse_value(&mut self) -> ParseResult<Value> {
        match try!(self.peek()) {
            Some(c) if is_ascii_number(c) => self.parse_number().map(Value::Constant),
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

    fn parse_number(&mut self) -> ParseResult<u16> {
        let mut strbuf = String::with_capacity(3);

        while let Some(c) = try!(self.next_char()) {
            if !is_ascii_number(c) {
                return Err(self.syntax_error(ErrorCode::InvalidNumberLiteral));
            }

            strbuf.push(::std::char::from_u32(c as u32).unwrap());
        }

        Ok(strbuf.parse().unwrap())
    }

    fn parse_whitespace(&mut self) -> ParseResult<()> {
        loop {
            match try!(self.peek()) {
                Some(b' ') => {
                    self.next_char();
                }
                _ => return Ok(()),
            }
        }
    }

    fn parse_wire_name(&mut self) -> ParseResult<String> {
        let mut strbuf = String::with_capacity(3);

        while let Some(c) = try!(self.next_char()) {
            if !is_ascii_lowercase(c) {
                return Err(self.syntax_error(ErrorCode::UppercaseLetterInWireName));
            }
            strbuf.push(::std::char::from_u32(c as u32).unwrap());
        }

        Ok(strbuf)
    }

    fn parse_assignment_arrow(&mut self) -> ParseResult<()> {
        match try!(self.peek()) {
            Some(b'-') => {
                match try!(self.peek()) {
                    Some(b'>') => Ok(()),
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }
}

#[test]
fn test_parse_value_number() {
    let mut parser = Parser::new("123".bytes().map(|x| Ok(x)));
    assert_eq!(parser.parse_value().unwrap(), Value::Constant(123));
}

#[test]
fn test_parse_invalid_number() {
    let mut parser = Parser::new("123x".bytes().map(|x| Ok(x)));

    match parser.parse_number() {
        Ok(v) => panic!("invalid number parsed as valid: {}", v),
        Err(e) => {
            match e {
                Error::SyntaxError(ErrorCode::InvalidNumberLiteral, 1, 4) => {}
                e => panic!("unexpected error: {:?}", e),
            }
        }
    }
}

#[test]
fn test_parse_wire_name() {
    let mut parser = Parser::new("ax".bytes().map(|x| Ok(x)));
    assert_eq!(parser.parse_wire_name().unwrap(), "ax".to_owned());
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
