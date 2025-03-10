use std::io::{Write, stdout};

use crate::{parser::ParseError, token::Token};

fn handle_escapes(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('t') => result.push('\t'),
                Some('\\') => result.push('\\'),
                Some(c) => result.push(c),
                None => break,
            }
        } else {
            result.push(c);
        }
    }

    result
}

pub fn execute(token: Token) -> Result<Token, ParseError> {
    match token {
        Token::Expression(tokens) => {
            if tokens.len() == 0 {
                return Ok(Token::Nil);
            }

            let mut iter = tokens.into_iter();

            let function = iter.next().unwrap();

            match function {
                Token::Identifier(id) => match id.as_str() {
                    "sum" => {
                        let mut result = 0.0;

                        while let Some(token) = iter.next() {
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
                    "mul" => {
                        let mut result = 1.0;

                        while let Some(token) = iter.next() {
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
                    "print" => {
                        while let Some(token) = iter.next() {
                            let value = token.value()?;

                            println!("{} ", handle_escapes(&value));
                        }

                        stdout().flush().unwrap();

                        Ok(Token::Nil)
                    }
                    _ => todo!(),
                },
                _ => Err(ParseError::UnknownError),
            }
        }
        _ => Err(ParseError::UnknownError),
    }
}
