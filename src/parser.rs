use std::{error::Error, fmt::Display, iter::Peekable, str::Chars};

use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnterminatedString,
    UnknownToken,
    UnknownError,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseError {}

pub fn parse_expression(chars: &mut Peekable<Chars<'_>>) -> Result<Token, ParseError> {
    let mut tokens = Vec::new();

    chars.next();

    while let Some(ch) = chars.peek() {
        let token = match ch {
            ' ' | '\t' | '\r' | '\n' => {
                chars.next();
                continue;
            }
            ')' => break,
            '(' => parse_expression(chars)?,
            '[' => parse_list(chars),
            '{' => parse_object(chars),
            '"' => parse_string(chars)?,
            '-' | '0'..='9' => parse_number(chars),
            'a'..='z' | 'A'..='Z' | '_' => parse_identifier(chars),
            _ => return Err(ParseError::UnknownToken),
        };

        tokens.push(token);
    }

    Ok(Token::Expression(tokens))
}

fn parse_number(chars: &mut Peekable<Chars<'_>>) -> Token {
    let mut num_str = String::new();

    loop {
        match chars.peek() {
            Some('0'..='9' | '.') => num_str.push(chars.next().unwrap()),
            _ => return Token::Number(num_str.parse().unwrap()),
        }
    }
}

fn parse_string(chars: &mut Peekable<Chars<'_>>) -> Result<Token, ParseError> {
    let mut string = String::new();

    chars.next();

    loop {
        match chars.peek() {
            Some('"') => {
                chars.next();
                return Ok(Token::String(string));
            }
            None | Some('\n') => {
                return Err(ParseError::UnterminatedString);
            }
            Some(_) => string.push(chars.next().unwrap()),
        }
    }
}

fn parse_identifier(chars: &mut Peekable<Chars<'_>>) -> Token {
    let mut identifier = String::new();

    loop {
        match chars.peek() {
            Some('a'..='z' | 'A'..='Z' | '_' | '0'..='9') => identifier.push(chars.next().unwrap()),
            _ => {
                let token = match identifier.as_str() {
                    "true" => Token::Bool(true),
                    "false" => Token::Bool(false),
                    "nil" => Token::Nil,
                    _ => Token::Identifier(identifier),
                };

                return token;
            }
        }
    }
}

fn parse_list(chars: &mut Peekable<Chars<'_>>) -> Token {
    todo!()
}

fn parse_object(chars: &mut Peekable<Chars<'_>>) -> Token {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string() {
        let mut iter = "\"Hello\"".chars().peekable();

        let result = Ok(Token::String("Hello".to_string()));

        assert_eq!(result, parse_string(&mut iter));
    }

    #[test]
    fn sum_of_two() {
        let mut iter = "(sum 1 1)".chars().peekable();

        let result = Ok(Token::Expression(vec![
            Token::Identifier("sum".to_string()),
            Token::Number(1.0),
            Token::Number(1.0),
        ]));

        assert_eq!(result, parse_expression(&mut iter));
    }
}
