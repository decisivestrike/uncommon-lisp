use std::collections::HashMap;

use crate::{builtins, errors::RuntimeError, token::Token};
use lazy_static::lazy_static;

lazy_static! {
    static ref VARIABLES: HashMap<&'static str, Token> = HashMap::new();
}

pub fn execute(token: Token) -> Result<Token, RuntimeError> {
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
                _ => Err(RuntimeError::TypeMismatch {
                    expected: "Identifier".to_string(),
                })?,
            }
        }
        _ => Err(RuntimeError::InvalidExpression),
    }
}
