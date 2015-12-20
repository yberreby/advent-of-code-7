use std::iter::Iterator;

mod utils;
use self::utils::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Token<'input> {
    Integer(u16),
    Identifier(&'input str),
    Operator(Operator),
    AssignmentArrow,
    Newline,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Lshift,
    Rshift,
    Or,
    And,
    Not,
}

struct Lexer<'input> {
    buffer: &'input [u8],
    idx: usize,
}

enum State {
    Start,
}


pub fn lex<'input>(input: &'input [u8]) -> Vec<Token<'input>> {
    let mut lexer = Lexer::new(input);
    lexer.lex()
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input [u8]) -> Lexer<'input> {
        Lexer {
            buffer: input,
            idx: 0,
        }
    }

    fn lex(&mut self) -> Vec<Token<'input>> {
        let mut tokens = Vec::new();

        while let Some(token) = self.read_token() {
            tokens.push(token)
        }

        tokens
    }

    #[inline]
    fn current_char(&self) -> Option<u8> {
        self.buffer.get(self.idx).map(|x| *x)
    }

    #[inline]
    fn bump(&mut self) {
        self.idx += 1
    }

    fn read_token(&mut self) -> Option<Token<'input>> {
        // Ignore leading spaces.
        while let Some(b' ') = self.current_char() {
            self.bump();
        }

        let start_idx = self.idx;

        match self.current_char() {
            Some(b'\n') => {
                self.bump();
                return Some(Token::Newline);
            }
            Some(b'0'...b'9') => {
                while let Some(b'0'...b'9') = self.current_char() {
                    self.bump();
                }

                let num_buf = &self.buffer[start_idx..self.idx];

                // It is safe to unwrap here, because we just checked the buffer was all
                // ASCII digits.
                let num_str = ::std::str::from_utf8(num_buf).unwrap();

                // Same here.
                let num = num_str.parse()
                                 .unwrap();
                return Some(Token::Integer(num));
            }
            Some(b'a'...b'z') => {
                while let Some(b'a'...b'z') = self.current_char() {
                    self.bump();
                }

                let identifier_buf = &self.buffer[start_idx..self.idx];
                let identifier_str = ::std::str::from_utf8(identifier_buf).unwrap();

                return Some(Token::Identifier(identifier_str));
            }
            Some(b'A'...b'Z') => {
                while let Some(b'A'...b'Z') = self.current_char() {
                    self.bump();
                }

                let keyword_buf = &self.buffer[start_idx..self.idx];
                let keyword_str = ::std::str::from_utf8(keyword_buf).unwrap();

                match keyword_str {
                    "NOT" => return Some(Token::Operator(Operator::Not)),
                    "AND" => return Some(Token::Operator(Operator::And)),
                    "OR" => return Some(Token::Operator(Operator::Or)),
                    "LSHIFT" => return Some(Token::Operator(Operator::Lshift)),
                    "RSHIFT" => return Some(Token::Operator(Operator::Rshift)),
                    _ => panic!("unknown keyword '{}'", keyword_str),
                }
            }
            Some(b'-') => {
                self.bump();
                match self.current_char() {
                    Some(b'>') => {
                        self.bump();
                        return Some(Token::AssignmentArrow);
                    }
                    c => panic!("expected '>', found '{:?}'", c),
                }
            }
            Some(c) => panic!("unexpected character '{}' at index {}", c, self.idx),
            None => return None,
        }



        //
        //
        // let real_state =
        // match state.input.next().unw
        //

        unimplemented!()
    }
}


mod tests {
    use super::*;

    #[test]
    fn lex_newline() {
        assert_eq!(lex(b"\n"), vec![Token::Newline]);
    }

    #[test]
    fn lex_numbers() {
        assert_eq!(lex(b"1"), vec![Token::Integer(1)]);
        assert_eq!(lex(b"123"), vec![Token::Integer(123)]);
        // u16::MAX
        assert_eq!(lex(b"65535"), vec![Token::Integer(65535)]);
    }

    #[test]
    fn lex_identifiers() {
        assert_eq!(lex(b"ax"), vec![Token::Identifier("ax")]);
        assert_eq!(lex(b"fu"), vec![Token::Identifier("fu")]);
        assert_eq!(lex(b"yz"), vec![Token::Identifier("yz")]);
    }

    #[test]
    fn lex_operators() {
        assert_eq!(lex(b"NOT"), vec![Token::Operator(Operator::Not)]);
        assert_eq!(lex(b"AND"), vec![Token::Operator(Operator::And)]);
        assert_eq!(lex(b"OR"), vec![Token::Operator(Operator::Or)]);
        assert_eq!(lex(b"LSHIFT"), vec![Token::Operator(Operator::Lshift)]);
        assert_eq!(lex(b"RSHIFT"), vec![Token::Operator(Operator::Rshift)]);

    }

    #[test]
    fn lex_assignment_arrow() {
        assert_eq!(lex(b"->"), vec![Token::AssignmentArrow]);
    }

    #[test]
    fn lex_simple_instructions() {
        assert_eq!(lex(b"123 -> ax"),
                   vec![Token::Integer(123), Token::AssignmentArrow, Token::Identifier("ax")]);

        assert_eq!(lex(b"yu -> bp"),
                   vec![Token::Identifier("yu"), Token::AssignmentArrow, Token::Identifier("bp")]);
    }

    #[test]
    fn lex_compound_instructions() {
        assert_eq!(lex(b"x LSHIFT 2 -> f"),
                   vec![Token::Identifier("x"),
                        Token::Operator(Operator::Lshift),
                        Token::Integer(2),
                        Token::AssignmentArrow,
                        Token::Identifier("f")]);

        assert_eq!(lex(b"NOT x -> h"),
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
fo OR \
                       fz -> ga
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

        assert_eq!(lex(program.as_bytes()), expected);
    }
}
