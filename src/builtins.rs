use std::io::{Write, stdout};

use crate::{
    errors::RuntimeError,
    executer::execute,
    extractor::evaluate,
    scope::Scope,
    token::Token,
    utils::{ULispType, unescape},
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
        Token::Identifier(name) => scope.get_variable(&name),
        t @ Token::Expression(_) => execute(t, scope)?,
        t => t,
    };

    scope.set_variable(name.to_string(), value);

    Ok(scope.get_variable(&name.to_string()))
}

pub fn typeof_<I>(tokens: I, scope: &mut Scope) -> Result<Token, RuntimeError>
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
        Token::Identifier(name) => {
            if scope.variables.contains_key(&name) {
                scope.get_variable(&name).as_type()
            } else if scope.functions.contains_key(&name) {
                ULispType::Function
            } else {
                Token::Nil.as_type()
            }
        }
        t @ Token::Expression(_) => execute(t, scope)?.as_type(),
        t => t.as_type(),
    };

    Ok(Token::String(value.to_string()))
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

pub fn print<I>(tokens: I, scope: &mut Scope) -> Result<Token, RuntimeError>
where
    I: IntoIterator<Item = Token>,
{
    let mut tokens = tokens.into_iter();
    let mut parts = Vec::new();

    while let Some(token) = tokens.next() {
        let value = match token {
            Token::Identifier(name) => scope.get_variable(&name),
            Token::Expression(_) => execute(token, scope)?,
            _ => token,
        };

        parts.push(value.to_string());
    }

    let output = parts.join(" ");

    print!("{}", unescape(&output));
    stdout().flush().unwrap();

    Ok(Token::Nil)
}
