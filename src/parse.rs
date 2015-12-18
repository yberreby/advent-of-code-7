use serde::iter::LineColIterator;
use std::io;
use Value;
use Operation;
use WireName;




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


pub struct InstructionParser<I: Iterator<Item = io::Result<u8>>> {
    input: LineColIterator<I>,
}

impl<I: Iterator<Item = io::Result<u8>>> InstructionParser<I> {
    pub fn new(iter: I) -> InstructionParser<I> {
        InstructionParser { input: LineColIterator::new(iter) }
    }

    fn parse(&mut self) -> Instruction {
        unimplemented!()
    }
}


pub fn parse_instruction(s: &str) -> Instruction {
    InstructionParser::new(s.bytes().map(|c| Ok(c))).parse()
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
