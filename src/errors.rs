use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnterminatedString {
        line: usize,
        position: usize,
    },
    UnknownToken {
        line: usize,
        position: usize,
        ch: char,
    },
    IncompleteExpression {
        line: usize,
        position: usize,
    },
    IncompleteList {
        line: usize,
        position: usize,
    },
    ExpectedExpression,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseError {}

#[derive(Debug, PartialEq)]
pub enum RuntimeError {
    // InvalidExpression,
    TypeMismatch { expected: String, found: String },
    NotEnoughArgs { min: usize },
    TooMuchArgs { max: usize },
    UndefinedFunction(String),
    InvalidArgCount { expected: usize, got: usize },
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for RuntimeError {}
