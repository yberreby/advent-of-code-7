use super::*;

fn lex<'input>(input: &'input str) -> Vec<Token<'input>> {
    let lexer = Lexer::new(input.as_bytes());
    lexer.collect()
}

#[test]
fn lex_newline() {
    assert_eq!(lex("\n"), vec![Token::Newline]);
}

#[test]
fn lex_numbers() {
    assert_eq!(lex("1"), vec![Token::Integer(1)]);
    assert_eq!(lex("123"), vec![Token::Integer(123)]);
    // u16::MAX
    assert_eq!(lex("65535"), vec![Token::Integer(65535)]);
}

#[test]
fn lex_identifiers() {
    assert_eq!(lex("ax"), vec![Token::Identifier("ax")]);
    assert_eq!(lex("fu"), vec![Token::Identifier("fu")]);
    assert_eq!(lex("yz"), vec![Token::Identifier("yz")]);
}

#[test]
fn lex_operators() {
    assert_eq!(lex("NOT"), vec![Token::Operator(Operator::Not)]);
    assert_eq!(lex("AND"), vec![Token::Operator(Operator::And)]);
    assert_eq!(lex("OR"), vec![Token::Operator(Operator::Or)]);
    assert_eq!(lex("LSHIFT"), vec![Token::Operator(Operator::Lshift)]);
    assert_eq!(lex("RSHIFT"), vec![Token::Operator(Operator::Rshift)]);

}

#[test]
fn lex_assignment_arrow() {
    assert_eq!(lex("->"), vec![Token::AssignmentArrow]);
}

#[test]
fn lex_simple_instructions() {
    assert_eq!(lex("123 -> ax"),
               vec![Token::Integer(123), Token::AssignmentArrow, Token::Identifier("ax")]);

    assert_eq!(lex("yu -> bp"),
               vec![Token::Identifier("yu"), Token::AssignmentArrow, Token::Identifier("bp")]);
}

#[test]
fn lex_compound_instructions() {
    assert_eq!(lex("x LSHIFT 2 -> f"),
               vec![Token::Identifier("x"),
                    Token::Operator(Operator::Lshift),
                    Token::Integer(2),
                    Token::AssignmentArrow,
                    Token::Identifier("f")]);

    assert_eq!(lex("NOT x -> h"),
               vec![Token::Operator(Operator::Not),
                    Token::Identifier("x"),
                    Token::AssignmentArrow,
                    Token::Identifier("h")]);
}

#[test]
fn lex_program() {
    let program = "bn RSHIFT 2 -> bo
lf RSHIFT 1 -> ly
fo RSHIFT 3 -> fq
cj OR cp -> cq
fo OR fz \
                   -> ga
t OR s -> u
lx -> a";

    let expected = vec![
        Token::Identifier("bn"), Token::Operator(Operator::Rshift), Token::Integer(2), Token::AssignmentArrow, Token::Identifier("bo"), Token::Newline,
        Token::Identifier("lf"), Token::Operator(Operator::Rshift), Token::Integer(1), Token::AssignmentArrow, Token::Identifier("ly"), Token::Newline,
        Token::Identifier("fo"), Token::Operator(Operator::Rshift), Token::Integer(3), Token::AssignmentArrow, Token::Identifier("fq"), Token::Newline,
        Token::Identifier("cj"), Token::Operator(Operator::Or), Token::Identifier("cp"), Token::AssignmentArrow, Token::Identifier("cq"), Token::Newline,
        Token::Identifier("fo"), Token::Operator(Operator::Or), Token::Identifier("fz"), Token::AssignmentArrow, Token::Identifier("ga"), Token::Newline,
        Token::Identifier("t"), Token::Operator(Operator::Or), Token::Identifier("s"), Token::AssignmentArrow, Token::Identifier("u"), Token::Newline,
        Token::Identifier("lx"), Token::AssignmentArrow, Token::Identifier("a"),
    ];

    assert_eq!(lex(program), expected);
}
