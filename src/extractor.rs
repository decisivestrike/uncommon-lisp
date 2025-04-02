use crate::{errors::RuntimeError, scope::Scope, token::Token};

pub trait Extractable: Sized {
    fn extract(token: Token, scope: &mut Scope) -> Result<Self, RuntimeError>;
}

impl Extractable for f64 {
    fn extract(token: Token, scope: &mut Scope) -> Result<Self, RuntimeError> {
        match token {
            Token::Number(value) => Ok(value),
            Token::Identifier(_) | Token::Expression(_) => {
                Self::extract(token.to_value(scope)?, scope)
            }
            _ => Err(RuntimeError::TypeMismatch {
                expected: "number".to_string(),
                found: token.as_type().to_string(),
            }),
        }
    }
}

impl Extractable for String {
    fn extract(token: Token, scope: &mut Scope) -> Result<Self, RuntimeError> {
        match token {
            Token::Identifier(_) | Token::Expression(_) => {
                Self::extract(token.to_value(scope)?, scope)
            }
            _ => Ok(token.to_string()),
        }
    }
}

impl Extractable for bool {
    fn extract(token: Token, scope: &mut Scope) -> Result<Self, RuntimeError> {
        Ok(match token {
            Token::Number(v) => v != 0.0,
            Token::String(v) => v.len() > 0,
            Token::Bool(value) => value,
            Token::Nil => false,
            Token::List(list) => list.len() > 0,
            Token::Identifier(_) | Token::Expression(_) => {
                Self::extract(token.to_value(scope)?, scope)?
            }
        })
    }
}

// Identifier
