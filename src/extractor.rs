use crate::{errors::RuntimeError, executer::execute, token::Token, utils::ULispType};

pub trait Extractable: Sized {
    fn extract(token: Token) -> Result<Self, RuntimeError>;
}

impl Extractable for f64 {
    fn extract(token: Token) -> Result<Self, RuntimeError> {
        match token {
            Token::Number(value) => Ok(value),
            _ => Err(RuntimeError::TypeMismatch {
                expected: ULispType::Number,
                found: token.as_type(),
            }),
        }
    }
}

impl Extractable for String {
    fn extract(token: Token) -> Result<Self, RuntimeError> {
        token.value()
    }
}

impl Extractable for bool {
    fn extract(token: Token) -> Result<Self, RuntimeError> {
        match token {
            Token::Number(v) => Ok(v != 0.0),
            Token::String(v) => Ok(v.len() > 0),
            Token::Bool(value) => Ok(value),
            Token::Nil => Ok(false),
            _ => Err(RuntimeError::TypeMismatch {
                expected: ULispType::Number,
                found: token.as_type(),
            }),
        }
    }
}

pub fn evaluate<T: Extractable>(token: Token) -> Result<T, RuntimeError> {
    match token {
        Token::Expression(_) => execute(token).and_then(|result| T::extract(result)),
        _ => T::extract(token),
    }
}
