use crate::{
    errors::RuntimeError, executer::execute, scope::Scope, token::Token, utils::ULispType,
};

pub trait Extractable: Sized {
    fn extract(token: Token, scope: &mut Scope) -> Result<Self, RuntimeError>;
}

impl Extractable for f64 {
    fn extract(token: Token, scope: &mut Scope) -> Result<Self, RuntimeError> {
        match token {
            Token::Number(value) => Ok(value),
            Token::Identifier(name) => Self::extract(scope.get_variable(&name), scope),
            Token::Expression(_) => Self::extract(execute(token, scope)?, scope),
            _ => Err(RuntimeError::TypeMismatch {
                expected: ULispType::Number,
                found: token.as_type(),
            }),
        }
    }
}

impl Extractable for String {
    fn extract(token: Token, scope: &mut Scope) -> Result<Self, RuntimeError> {
        match token {
            Token::Identifier(name) => Self::extract(scope.get_variable(&name), scope),
            Token::Expression(_) => Self::extract(execute(token, scope)?, scope),
            _ => Ok(token.to_string()),
        }
    }
}

impl Extractable for bool {
    fn extract(token: Token, scope: &mut Scope) -> Result<Self, RuntimeError> {
        match token {
            Token::Number(v) => Ok(v != 0.0),
            Token::String(v) => Ok(v.len() > 0),
            Token::Bool(value) => Ok(value),
            Token::Nil => Ok(false),
            Token::Identifier(name) => Self::extract(scope.get_variable(&name), scope),
            Token::Expression(_) => Self::extract(execute(token, scope)?, scope),
            _ => Err(RuntimeError::TypeMismatch {
                expected: ULispType::Number,
                found: token.as_type(),
            }),
        }
    }
}

pub fn evaluate<T: Extractable>(token: Token, scope: &mut Scope) -> Result<T, RuntimeError> {
    match token {
        Token::Expression(_) => execute(token, scope).and_then(|result| T::extract(result, scope)),
        _ => T::extract(token, scope),
    }
}
