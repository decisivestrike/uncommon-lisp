use std::io::{Write, stdout};

use crate::{parser::ParseError, token::Token};

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

                        while let Some(Token::Number(value)) = iter.next() {
                            result += value;
                        }

                        Ok(Token::Number(result))
                    }
                    "print" => {
                        while let Some(token) = iter.next() {
                            print!("{} ", token.value());
                        }

                        stdout().flush().unwrap();

                        Ok(Token::Nil)
                    }
                    "println" => {
                        while let Some(token) = iter.next() {
                            print!("{} ", token.value());
                        }

                        print!("\n");

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
