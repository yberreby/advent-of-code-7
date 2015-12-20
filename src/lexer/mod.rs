use std::iter::Iterator;

mod utils;
use self::utils::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Integer(u16),
    Identifier(String),
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


pub fn lex(input: &[u8]) -> Vec<Token> {
    let mut lexer = Lexer::new(input);
    lexer.lex()
}

impl<'input> Lexer<'input> {
    pub fn new(input: &[u8]) -> Lexer {
        Lexer {
            buffer: input,
            idx: 0,
        }
    }

    fn lex(&mut self) -> Vec<Token> {
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

    fn read_token(&mut self) -> Option<Token> {
        // Ignore leading spaces.
        while let Some(b' ') = self.current_char() {
            self.bump();
        }

        let start_idx = self.idx;

        match self.current_char() {
            Some(b'\n') => return Some(Token::Newline),
            Some(b'0'...b'9') => {
                while let Some(b'0'...b'9') = self.current_char() {
                    self.bump();
                }

                let num_buf = &self.buffer[start_idx..self.idx];
                let num_str = ::std::str::from_utf8(num_buf).unwrap();

                // It is safe to unwrap here, because we just checked the buffer was all
                // ASCII digits.
                let num = num_str.parse()
                                 .unwrap();
                return Some(Token::Integer(num));
            }
            Some(c) => panic!(),
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
    fn lex_numbers() {
        assert_eq!(lex(b"1"), vec![Token::Integer(1)]);
        assert_eq!(lex(b"123"), vec![Token::Integer(123)]);
        // u16::MAX
        assert_eq!(lex(b"65535"), vec![Token::Integer(65535)]);
    }
}
