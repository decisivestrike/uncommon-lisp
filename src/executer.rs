use std::collections::HashMap;
use std::sync::Once;

use crate::{builtins, parser::ParseError, token::Token};

pub unsafe fn variables() -> &'static HashMap<String, Token> {
    static ref VARIABLES: Option<HashMap<String, Token>> = None;

    unsafe {
        Once::new().call_once(|| {
            VARIABLES = Some(HashMap::new());
        });
    }

    unsafe { &VARIABLES }
}

pub fn handle_escapes(s: &str) -> String {
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

            let mut tokens = tokens.into_iter();
            let function = tokens.next().unwrap();

            match function {
                Token::Identifier(id) => match id.as_str() {
                    "add" => builtins::add(tokens),
                    "sub" => builtins::sub(tokens),
                    "mul" => builtins::mul(tokens),
                    "div" => builtins::div(tokens),
                    "print" => builtins::print(tokens),
                    _ => todo!(),
                },
                _ => Err(ParseError::UnknownError)?,
            }
        }
        _ => Err(ParseError::UnknownError)?,
    }
}
