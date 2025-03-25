use crate::{builtins, errors::RuntimeError, token::Token, utils::ULispType};

pub fn execute(token: Token) -> Result<Token, RuntimeError> {
    match token {
        Token::Expression(tokens) => {
            if tokens.len() == 0 {
                return Ok(Token::Nil);
            }

            let mut tokens = tokens.into_iter();
            let func_id = tokens.next().unwrap();

            if func_id.as_type() != ULispType::Identifier {
                return Err(RuntimeError::InvalidExpression);
            }

            match func_id {
                Token::Identifier(id) => match id.as_str() {
                    "add" => builtins::add(tokens),
                    "sub" => builtins::sub(tokens),
                    "mul" => builtins::mul(tokens),
                    "div" => builtins::div(tokens),

                    "concat" => builtins::concat(tokens),

                    "print" => builtins::print(tokens),
                    // TODO: Find func in hashmap
                    _ => Err(RuntimeError::UndefinedFunction(id)),
                },

                _ => Err(RuntimeError::TypeMismatch {
                    expected: ULispType::Expression,
                    found: func_id.as_type(),
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
