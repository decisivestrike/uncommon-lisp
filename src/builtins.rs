use std::{
    collections::HashMap,
    io::{Write, stdout},
    vec::IntoIter,
};

use lazy_static::lazy_static;

use crate::{
    errors::RuntimeError,
    extractor::evaluate,
    token::Token,
    utils::{ULispType, handle_escapes},
};

// TODO: Add func arg guard

type ULispFunc = fn(IntoIter<Token>) -> Result<Token, RuntimeError>;

pub fn add(mut tokens: IntoIter<Token>) -> Result<Token, RuntimeError> {
    tokens
        .try_fold(0.0, |acc, token| {
            let value: f64 = evaluate(token)?;
            Ok(acc + value)
        })
        .map(Token::Number)
}

pub fn sub(mut tokens: IntoIter<Token>) -> Result<Token, RuntimeError> {
    let base: f64 = evaluate(tokens.next().unwrap())?;

    tokens
        .try_fold(base, |acc, token| {
            let value: f64 = evaluate(token)?;
            Ok(acc - value)
        })
        .map(Token::Number)
}

pub fn mul(mut tokens: IntoIter<Token>) -> Result<Token, RuntimeError> {
    tokens
        .try_fold(1.0, |acc, token| {
            let value: f64 = evaluate(token)?;
            Ok(acc * value)
        })
        .map(Token::Number)
}

pub fn div(mut tokens: IntoIter<Token>) -> Result<Token, RuntimeError> {
    let base: f64 = evaluate(tokens.next().unwrap())?;

    tokens
        .try_fold(base, |acc, token| {
            let value: f64 = evaluate(token)?;
            Ok(acc / value)
        })
        .map(Token::Number)
}

pub fn concat(mut tokens: IntoIter<Token>) -> Result<Token, RuntimeError> {
    tokens
        .try_fold(String::new(), |acc, token| {
            let value: String = evaluate(token)?;
            Ok(acc + &value)
        })
        .map(Token::String)
}

pub fn set_variable(mut tokens: IntoIter<Token>) -> Result<Token, RuntimeError> {
    if tokens.len() < 2 {
        return Err(RuntimeError::NotEnoughArgs);
    }

    let name = tokens.next().unwrap();

    if !matches!(name, Token::Identifier(_)) {
        return Err(RuntimeError::TypeMismatch {
            expected: ULispType::Identifier,
            found: name.as_type(),
        });
    }

    let value = tokens.next().unwrap();

    // Set name value

    todo!()
}

pub fn while_loop(mut tokens: IntoIter<Token>) -> Result<Token, RuntimeError> {
    todo!()
}

pub fn print(mut tokens: IntoIter<Token>) -> Result<Token, RuntimeError> {
    while let Some(token) = tokens.next() {
        let value = token.value()?;

        print!("{} ", handle_escapes(&value));
    }

    stdout().flush().unwrap();

    Ok(Token::Nil)
}
