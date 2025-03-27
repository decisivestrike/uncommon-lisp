use std::{
    io::{Write, stdout},
    vec::IntoIter,
};

use crate::{
    errors::RuntimeError,
    executer::execute,
    extractor::evaluate,
    scope::Scope,
    token::Token,
    utils::{ULispType, handle_escapes},
};

// TODO: Add func arg guard

pub fn add(mut tokens: IntoIter<Token>, scope: &mut Scope) -> Result<Token, RuntimeError> {
    tokens
        .try_fold(0.0, |acc, token| {
            let value: f64 = evaluate(token, scope)?;
            Ok(acc + value)
        })
        .map(Token::Number)
}

pub fn sub(mut tokens: IntoIter<Token>, scope: &mut Scope) -> Result<Token, RuntimeError> {
    let base: f64 = evaluate(tokens.next().unwrap(), scope)?;

    tokens
        .try_fold(base, |acc, token| {
            let value: f64 = evaluate(token, scope)?;
            Ok(acc - value)
        })
        .map(Token::Number)
}

pub fn mul(mut tokens: IntoIter<Token>, scope: &mut Scope) -> Result<Token, RuntimeError> {
    tokens
        .try_fold(1.0, |acc, token| {
            let value: f64 = evaluate(token, scope)?;
            Ok(acc * value)
        })
        .map(Token::Number)
}

pub fn div(mut tokens: IntoIter<Token>, scope: &mut Scope) -> Result<Token, RuntimeError> {
    let base: f64 = evaluate(tokens.next().unwrap(), scope)?;

    tokens
        .try_fold(base, |acc, token| {
            let value: f64 = evaluate(token, scope)?;
            Ok(acc / value)
        })
        .map(Token::Number)
}

pub fn concat(mut tokens: IntoIter<Token>, scope: &mut Scope) -> Result<Token, RuntimeError> {
    tokens
        .try_fold(String::new(), |acc, token| {
            let value: String = evaluate(token, scope)?;
            Ok(acc + &value)
        })
        .map(Token::String)
}

pub fn set_variable(mut tokens: IntoIter<Token>, scope: &mut Scope) -> Result<Token, RuntimeError> {
    if tokens.len() < 2 {
        return Err(RuntimeError::NotEnoughArgs);
    }

    if tokens.len() > 2 {
        return Err(RuntimeError::TooMuchArgs);
    }

    let name = tokens.next().unwrap();

    if !matches!(name, Token::Identifier(_)) {
        return Err(RuntimeError::TypeMismatch {
            expected: ULispType::Identifier,
            found: name.as_type(),
        });
    }

    let value = tokens.next().unwrap();

    scope.set_variable(name.to_string(), value);

    Ok(scope.get_variable(name.to_string()))
}

pub fn print(mut tokens: IntoIter<Token>, scope: &mut Scope) -> Result<Token, RuntimeError> {
    while let Some(token) = tokens.next() {
        let value = match token {
            Token::Identifier(name) => scope.get_variable(name),
            Token::Expression(_) => execute(token, scope)?,
            _ => token,
        };

        print!("{} ", handle_escapes(&value.to_string()));
    }

    stdout().flush().unwrap();

    Ok(Token::Nil)
}
