use std::io::{Write, stdout};

use crate::{
    errors::RuntimeError,
    executer::execute,
    extractor::evaluate,
    scope::Scope,
    token::Token,
    utils::{ULispType, handle_escapes},
};

pub fn add<I>(tokens: I, scope: &mut Scope) -> Result<Token, RuntimeError>
where
    I: IntoIterator<Item = Token>,
{
    tokens
        .into_iter()
        .try_fold(0.0, |acc, token| {
            let value: f64 = evaluate(token, scope)?;
            Ok(acc + value)
        })
        .map(Token::Number)
}

pub fn sub<I>(tokens: I, scope: &mut Scope) -> Result<Token, RuntimeError>
where
    I: IntoIterator<Item = Token>,
    I::IntoIter: ExactSizeIterator,
{
    let mut tokens = tokens.into_iter();

    if tokens.len() < 2 {
        return Err(RuntimeError::NotEnoughArgs);
    }

    let base: f64 = evaluate(tokens.next().unwrap(), scope)?;

    tokens
        .try_fold(base, |acc, token| {
            let value: f64 = evaluate(token, scope)?;
            Ok(acc - value)
        })
        .map(Token::Number)
}

pub fn mul<I>(tokens: I, scope: &mut Scope) -> Result<Token, RuntimeError>
where
    I: IntoIterator<Item = Token>,
{
    tokens
        .into_iter()
        .try_fold(1.0, |acc, token| {
            let value: f64 = evaluate(token, scope)?;
            Ok(acc * value)
        })
        .map(Token::Number)
}

pub fn div<I>(tokens: I, scope: &mut Scope) -> Result<Token, RuntimeError>
where
    I: IntoIterator<Item = Token>,
    I::IntoIter: ExactSizeIterator,
{
    let mut tokens = tokens.into_iter();

    if tokens.len() < 2 {
        return Err(RuntimeError::NotEnoughArgs);
    }

    let base: f64 = evaluate(tokens.next().unwrap(), scope)?;

    tokens
        .try_fold(base, |acc, token| {
            let value: f64 = evaluate(token, scope)?;
            Ok(acc / value)
        })
        .map(Token::Number)
}

pub fn concat<I>(tokens: I, scope: &mut Scope) -> Result<Token, RuntimeError>
where
    I: IntoIterator<Item = Token>,
    I::IntoIter: ExactSizeIterator,
{
    tokens
        .into_iter()
        .try_fold(String::new(), |acc, token| {
            let value: String = evaluate(token, scope)?;
            Ok(acc + &value)
        })
        .map(Token::String)
}

pub fn set_variable<I>(tokens: I, scope: &mut Scope) -> Result<Token, RuntimeError>
where
    I: IntoIterator<Item = Token>,
    I::IntoIter: ExactSizeIterator,
{
    let mut tokens = tokens.into_iter();

    if tokens.len() < 2 {
        return Err(RuntimeError::NotEnoughArgs);
    }

    if tokens.len() > 2 {
        return Err(RuntimeError::TooMuchArgs);
    }

    let name = match tokens.next().unwrap() {
        t @ Token::Identifier(_) => t,
        t => Err(RuntimeError::TypeMismatch {
            expected: ULispType::Identifier,
            found: t.as_type(),
        })?,
    };

    let value = match tokens.next().unwrap() {
        Token::Identifier(name) => scope.get_variable(name),
        t @ Token::Expression(_) => execute(t, scope)?,
        t => t,
    };

    scope.set_variable(name.to_string(), value);

    Ok(scope.get_variable(name.to_string()))
}

pub fn get_type<I>(tokens: I, scope: &mut Scope) -> Result<Token, RuntimeError>
where
    I: IntoIterator<Item = Token>,
    I::IntoIter: ExactSizeIterator,
{
    let mut tokens = tokens.into_iter();

    if tokens.len() == 0 {
        return Err(RuntimeError::NotEnoughArgs);
    }

    if tokens.len() > 1 {
        return Err(RuntimeError::TooMuchArgs);
    }

    let value = match tokens.next().unwrap() {
        Token::Identifier(name) => scope.get_variable(name),
        t @ Token::Expression(_) => execute(t, scope)?,
        t => t,
    };

    Ok(Token::String(value.as_type().to_string()))
}

pub fn print<I>(tokens: I, scope: &mut Scope) -> Result<Token, RuntimeError>
where
    I: IntoIterator<Item = Token>,
{
    let mut tokens = tokens.into_iter();

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
