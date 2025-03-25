use std::collections::HashMap;

use crate::{builtins, errors::RuntimeError, token::Token, utils::ULispType};
use lazy_static::lazy_static;

lazy_static! {
    static ref VARIABLES: HashMap<&'static str, Token> = HashMap::new();
}

pub fn execute(token: Token) -> Result<Token, RuntimeError> {
    match token {
        Token::Expression(tokens) => {
            if tokens.len() == 0 {
                return Ok(Token::Nil);
            }

            let mut tokens = tokens.into_iter();

            let func_id = tokens.next().unwrap();

            if func_id.as_type() != ULispType::Identifier {
                return Err(RuntimeError::InvalidExpression);
            }

            match func_id {
                Token::Identifier(id) => match id.as_str() {
                    "add" => builtins::add(tokens),
                    "sub" => builtins::sub(tokens),
                    "mul" => builtins::mul(tokens),
                    "div" => builtins::div(tokens),
                    "print" => builtins::print(tokens),
                    // TODO: Find func in hashmap
                    _ => panic!("Undefined function"),
                },

                _ => Err(RuntimeError::TypeMismatch {
                    expected: ULispType::Expression,
                    found: func_id.as_type(),
                }),
            }
        }
        _ => Err(RuntimeError::InvalidExpression),
    }
}

trait Extractable: Sized {
    fn extract(token: Token) -> Result<Self, RuntimeError>;
    fn type_name() -> ULispType;
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

    fn type_name() -> ULispType {
        ULispType::Number
    }
}

impl Extractable for String {
    fn extract(token: Token) -> Result<Self, RuntimeError> {
        match token {
            Token::String(value) => Ok(value),
            _ => Err(RuntimeError::TypeMismatch {
                expected: ULispType::String,
                found: token.as_type(),
            }),
        }
    }

    fn type_name() -> ULispType {
        ULispType::String
    }
}

impl Extractable for bool {
    fn extract(token: Token) -> Result<Self, RuntimeError> {
        match token {
            Token::Bool(value) => Ok(value),
            _ => Err(RuntimeError::TypeMismatch {
                expected: ULispType::Number,
                found: token.as_type(),
            }),
        }
    }

    fn type_name() -> ULispType {
        ULispType::Bool
    }
}

pub fn evaluate<T: Extractable>(token: Token) -> Result<T, RuntimeError> {
    match token {
        Token::Expression(_) => execute(token).and_then(|result| T::extract(result)),
        _ => T::extract(token),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_two() {
        let expression = Token::Expression(vec![
            Token::Identifier("add".to_string()),
            Token::Number(1.0),
            Token::Number(1.0),
        ]);

        let expected = Token::Number(2.0);

        assert_eq!(execute(expression).unwrap(), expected);
    }
}
