use lexer::Token;
use lexer::Operator;
use super::*;

pub fn parse<'input>(tokens: Vec<Token<'input>>) -> Vec<Instruction> {
    let mut parser = Parser::new(tokens.into_iter());
    parser.parse()
}


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
