use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnterminatedString { line: u64, position: u64 },
    UnknownToken { line: u64, position: u64, ch: char },
    IncompleteExpression { line: u64, position: u64 },
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseError {}

#[derive(Debug, PartialEq)]
pub enum RuntimeError {
    InvalidExpression,
    TypeMismatch { expected: String },
    TooMuchArgs,
    NotEnoughArgs,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for RuntimeError {}
