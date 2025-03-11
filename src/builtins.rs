#[allow(dead_code)]
use std::{
    io::{Write, stdout},
    vec::IntoIter,
};

use crate::{errors::RuntimeError, executer::execute, token::Token, utils::handle_escapes};

pub fn add(mut tokens: IntoIter<Token>) -> Result<Token, RuntimeError> {
    let mut result = 0.0;

    while let Some(token) = tokens.next() {
        let value = match token {
            Token::Expression(_) => match execute(token)? {
                Token::Number(value) => value,
                _ => Err(RuntimeError::TypeMismatch {
                    expected: "Number".to_string(),
                })?,
            },
            Token::Number(number) => number,
            _ => Err(RuntimeError::TypeMismatch {
                expected: "Number".to_string(),
            })?,
        };

        result += value;
    }

    Ok(Token::Number(result))
}

pub fn sub(mut tokens: IntoIter<Token>) -> Result<Token, RuntimeError> {
    let mut base = match tokens.next() {
        Some(Token::Number(value)) => value,
        _ => Err(RuntimeError::TypeMismatch {
            expected: "Number".to_string(),
        })?,
    };

    while let Some(token) = tokens.next() {
        let value = match token {
            Token::Expression(_) => match execute(token)? {
                Token::Number(value) => value,
                _ => Err(RuntimeError::TypeMismatch {
                    expected: "Number".to_string(),
                })?,
            },
            Token::Number(number) => number,
            _ => Err(RuntimeError::TypeMismatch {
                expected: "Number".to_string(),
            })?,
        };

        base -= value;
    }

    Ok(Token::Number(base))
}

pub fn mul(mut tokens: IntoIter<Token>) -> Result<Token, RuntimeError> {
    let mut result = 1.0;

    while let Some(token) = tokens.next() {
        let value = match token {
            Token::Expression(_) => match execute(token)? {
                Token::Number(value) => value,
                _ => Err(RuntimeError::TypeMismatch {
                    expected: "Number".to_string(),
                })?,
            },
            Token::Number(number) => number,
            _ => Err(RuntimeError::TypeMismatch {
                expected: "Number".to_string(),
            })?,
        };

        result *= value;
    }

    Ok(Token::Number(result))
}

pub fn div(mut tokens: IntoIter<Token>) -> Result<Token, RuntimeError> {
    let mut base = match tokens.next() {
        Some(Token::Number(value)) => value,
        _ => Err(RuntimeError::TypeMismatch {
            expected: "Number".to_string(),
        })?,
    };

    while let Some(token) = tokens.next() {
        let value = match token {
            Token::Expression(_) => match execute(token)? {
                Token::Number(value) => value,
                _ => Err(RuntimeError::TypeMismatch {
                    expected: "Number".to_string(),
                })?,
            },
            Token::Number(number) => number,
            _ => Err(RuntimeError::TypeMismatch {
                expected: "Number".to_string(),
            })?,
        };

        base /= value;
    }

    Ok(Token::Number(base))
}

pub fn set_variable(mut tokens: IntoIter<Token>) {
    todo!()
}

pub fn while_loop(mut tokens: IntoIter<Token>) -> Result<Token, RuntimeError> {
    todo!()
}

pub fn print(mut tokens: IntoIter<Token>) -> Result<Token, RuntimeError> {
    while let Some(token) = tokens.next() {
        let value = token.value()?;

        println!("{} ", handle_escapes(&value));
    }

    stdout().flush().unwrap();

    Ok(Token::Nil)
}
