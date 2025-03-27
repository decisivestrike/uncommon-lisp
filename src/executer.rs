use crate::{builtins, errors::RuntimeError, scope::Scope, token::Token, utils::ULispType};

pub fn execute(token: Token, scope: &mut Scope) -> Result<Token, RuntimeError> {
    match token {
        Token::Expression(mut tokens) => {
            if tokens.len() == 0 {
                return Ok(Token::Nil);
            }

            let func_id = tokens.pop_front().unwrap();

            match func_id {
                Token::Identifier(id) => match id.as_str() {
                    "add" => builtins::add(tokens, scope),
                    "sub" => builtins::sub(tokens, scope),
                    "mul" => builtins::mul(tokens, scope),
                    "div" => builtins::div(tokens, scope),

                    "set" => builtins::set_variable(tokens, scope),
                    "type" => builtins::get_type(tokens, scope),

                    "concat" => builtins::concat(tokens, scope),
                    "print" => builtins::print(tokens, scope),
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
    use std::vec;

    use super::*;

    #[test]
    fn sum_of_two() {
        let expression = Token::Expression(
            vec![
                Token::Identifier("add".to_string()),
                Token::Number(1.0),
                Token::Number(1.0),
            ]
            .into(),
        );

        let expected = Token::Number(2.0);

        assert_eq!(execute(expression, &mut Scope::new()).unwrap(), expected);
    }

    #[test]
    fn greeting() {
        let set_variable = Token::Expression(
            vec![
                Token::Identifier("set".to_string()),
                Token::Identifier("name".to_string()),
                Token::String("Denis".to_string()),
            ]
            .into(),
        );

        let concat = Token::Expression(
            vec![
                Token::Identifier("concat".to_string()),
                Token::String("Hello ".to_string()),
                Token::Identifier("name".to_string()),
            ]
            .into(),
        );

        let mut scope = Scope::new();

        assert_eq!(
            execute(set_variable, &mut scope).unwrap(),
            Token::String("Denis".to_string())
        );

        assert_eq!(
            execute(concat, &mut scope).unwrap(),
            Token::String("Hello Denis".to_string())
        );
    }

    #[test]
    fn sum_of_sum() {
        let expression = Token::Expression(
            vec![
                Token::Identifier("add".to_string()),
                Token::Expression(
                    vec![Token::Identifier("add".to_string()), Token::Number(1.0)].into(),
                ),
                Token::Expression(
                    vec![Token::Identifier("add".to_string()), Token::Number(1.0)].into(),
                ),
            ]
            .into(),
        );

        assert_eq!(
            execute(expression, &mut Scope::new()).unwrap(),
            Token::Number(2.0)
        );
    }

    #[test]
    fn set_in_set() {
        let expression = Token::Expression(
            vec![
                Token::Identifier("set".to_string()),
                Token::Identifier("anotherName".to_string()),
                Token::Expression(
                    vec![
                        Token::Identifier("set".to_string()),
                        Token::Identifier("name".to_string()),
                        Token::String("Denis".to_string()),
                    ]
                    .into(),
                ),
            ]
            .into(),
        );

        assert_eq!(
            execute(expression, &mut Scope::new()).unwrap(),
            Token::String("Denis".to_string())
        );
    }
}
