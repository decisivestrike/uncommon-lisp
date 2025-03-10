use std::{
    io::{Write, stdout},
    vec::IntoIter,
};

use crate::{
    executer::{execute, handle_escapes},
    parser::ParseError,
    token::Token,
};

pub fn add(mut tokens: IntoIter<Token>) -> Result<Token, ParseError> {
    let mut result = 0.0;

    while let Some(token) = tokens.next() {
        let value = match token {
            Token::Expression(_) => match execute(token)? {
                Token::Number(value) => value,
                _ => Err(ParseError::UnknownError)?,
            },
            Token::Number(number) => number,
            _ => Err(ParseError::UnknownError)?,
        };

        result += value;
    }

    Ok(Token::Number(result))
}

pub fn sub(mut tokens: IntoIter<Token>) -> Result<Token, ParseError> {
    let mut base = match tokens.next() {
        Some(Token::Number(value)) => value,
        _ => Err(ParseError::UnknownError)?,
    };

    while let Some(token) = tokens.next() {
        let value = match token {
            Token::Expression(_) => match execute(token)? {
                Token::Number(value) => value,
                _ => Err(ParseError::UnknownError)?,
            },
            Token::Number(number) => number,
            _ => Err(ParseError::UnknownError)?,
        };

        base -= value;
    }

    Ok(Token::Number(base))
}

pub fn mul(mut tokens: IntoIter<Token>) -> Result<Token, ParseError> {
    let mut result = 1.0;

    while let Some(token) = tokens.next() {
        let value = match token {
            Token::Expression(_) => match execute(token)? {
                Token::Number(value) => value,
                _ => Err(ParseError::UnknownError)?,
            },
            Token::Number(number) => number,
            _ => Err(ParseError::UnknownError)?,
        };

        result *= value;
    }

    Ok(Token::Number(result))
}

pub fn div(mut tokens: IntoIter<Token>) -> Result<Token, ParseError> {
    let mut base = match tokens.next() {
        Some(Token::Number(value)) => value,
        _ => Err(ParseError::UnknownError)?,
    };

    while let Some(token) = tokens.next() {
        let value = match token {
            Token::Expression(_) => match execute(token)? {
                Token::Number(value) => value,
                _ => Err(ParseError::UnknownError)?,
            },
            Token::Number(number) => number,
            _ => Err(ParseError::UnknownError)?,
        };

        base /= value;
    }

    Ok(Token::Number(base))
}

pub fn set(mut tokens: IntoIter<Token>) {}

pub fn while_loop(mut tokens: IntoIter<Token>) -> Result<Token, ParseError> {
    todo!()
}

pub fn print(mut tokens: IntoIter<Token>) -> Result<Token, ParseError> {
    while let Some(token) = tokens.next() {
        let value = token.value()?;

        println!("{} ", handle_escapes(&value));
    }

    stdout().flush().unwrap();

    Ok(Token::Nil)
}
