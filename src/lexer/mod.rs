use std::iter::Iterator;

#[cfg(test)]
mod tests;


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

impl<'input> Iterator for Lexer<'input> {
    type Item = Token<'input>;

    fn next(&mut self) -> Option<Token<'input>> {
        self.read_token()
    }
}

pub struct Lexer<'input> {
    buffer: &'input [u8],
    idx: usize,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input [u8]) -> Lexer<'input> {
        Lexer {
            buffer: input,
            idx: 0,
        }
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
            // EOF
            None => return None,
        }
    }
}
