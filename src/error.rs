use std::io;

#[derive(Debug)]
pub enum ParseErrorCode {

}

#[derive(Debug)]
pub enum ParseError {
    SyntaxError(ParseErrorCode, usize, usize),
    IoError(io::Error),
}

pub type ParseResult<T> = Result<T, ParseError>;
