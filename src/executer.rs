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
                    _ => panic!("Undefined function"),
                },
                _ => Err(RuntimeError::TypeMismatch {
                    expected: "Identifier".to_string(),
                }),
            }
        }
        _ => Err(RuntimeError::InvalidExpression),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_two() {
        let expression = Token::Expression(vec![
            Token::Identifier("add".to_string()),
            Token::Number(1.0),
            Token::Number(1.0),
        ]);

        let expected = Token::Number(2.0);

        assert_eq!(execute(expression).unwrap(), expected);
    }
}
