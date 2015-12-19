use std::io;

#[derive(Debug, PartialEq)]
pub enum ErrorCode {
    UppercaseLetterInWireName,
    InvalidNumberLiteral,
}

#[derive(Debug)]
pub enum Error {
    SyntaxError(ErrorCode, usize, usize),
    IoError(io::Error),
}

pub type ParseResult<T> = Result<T, Error>;
