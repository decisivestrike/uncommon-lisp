use crate::{
    errors::RuntimeError,
    token::{Expression, Identifier, List, Token},
};

pub trait Extractable: Sized {
    fn extract(token: Token, maybe_prefix: Option<String>) -> Result<Self, RuntimeError>;
}

impl Extractable for f64 {
    fn extract(token: Token, maybe_prefix: Option<String>) -> Result<Self, RuntimeError> {
        match token {
            Token::Number(value) => Ok(value),
            Token::Identifier(_) | Token::Expression(_) => {
                Self::extract(token.into_value(maybe_prefix.clone())?, maybe_prefix)
            }
            _ => Err(RuntimeError::TypeMismatch {
                expected: "number".to_string(),
                found: token.as_type().to_string(),
            }),
        }
    }
}

impl Extractable for String {
    fn extract(token: Token, maybe_prefix: Option<String>) -> Result<Self, RuntimeError> {
        match token {
            Token::Identifier(_) | Token::Expression(_) => {
                Self::extract(token.into_value(maybe_prefix.clone())?, maybe_prefix)
            }
            _ => Ok(token.to_string()),
        }
    }
}

impl Extractable for bool {
    fn extract(token: Token, maybe_prefix: Option<String>) -> Result<Self, RuntimeError> {
        Ok(match token {
            Token::Number(v) => v != 0.0,
            Token::String(v) => v.len() > 0,
            Token::Bool(value) => value,
            Token::Nil => false,
            Token::List(list) => list.len() > 0,
            Token::Identifier(_) | Token::Expression(_) => {
                Self::extract(token.into_value(maybe_prefix.clone())?, maybe_prefix)?
            }
        })
    }
}

impl Extractable for Identifier {
    fn extract(token: Token, _: Option<String>) -> Result<Self, RuntimeError> {
        match token {
            Token::Identifier(id) => Ok(id),
            _ => Err(RuntimeError::TypeMismatch {
                expected: "identifier".to_string(),
                found: token.as_type(),
            }),
        }
    }
}

impl Extractable for List {
    fn extract(token: Token, _: Option<String>) -> Result<Self, RuntimeError> {
        match token {
            Token::List(id) => Ok(id),
            _ => Err(RuntimeError::TypeMismatch {
                expected: "list".to_string(),
                found: token.as_type(),
            }),
        }
    }
}

impl Extractable for Expression {
    fn extract(token: Token, _: Option<String>) -> Result<Self, RuntimeError> {
        match token {
            Token::Expression(e) => Ok(e),
            _ => Err(RuntimeError::TypeMismatch {
                expected: "expression".to_string(),
                found: token.as_type(),
            }),
        }
    }
}
