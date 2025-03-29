use std::collections::VecDeque;

use crate::{builtins, errors::RuntimeError, scope::Scope, token::Token, utils::ULispType};

pub fn execute(token: Token, scope: &mut Scope) -> Result<Token, RuntimeError> {
    match token {
        Token::Expression(mut tokens) => {
            if tokens.len() == 0 {
                return Ok(Token::Nil);
            }

            match tokens.pop_front().unwrap() {
                Token::Identifier(name) => match builtins::FUNCTIONS.get(name.as_str()) {
                    Some(func) => func(tokens, scope),
                    None => match scope.get_function(&name) {
                        Some((arg_names, body)) => {
                            execute(custom_func_call(arg_names, tokens, body), scope)
                        }
                        None => Err(RuntimeError::UndefinedFunction(name)),
                    },
                },

                t => Err(RuntimeError::TypeMismatch {
                    expected: ULispType::Expression,
                    found: t.as_type(),
                }),
            }
        }
        _ => Err(RuntimeError::InvalidExpression),
    }
}

fn custom_func_call(arg_names: VecDeque<Token>, args: VecDeque<Token>, body: Token) -> Token {
    let Token::Expression(mut expression_parts) = body else {
        unreachable!()
    };

    for (name, value) in arg_names.into_iter().zip(args.into_iter()) {
        while let Some(i) = expression_parts.iter().position(|t| *t == name) {
            expression_parts[i] = value.clone();
        }
    }

    Token::Expression(expression_parts)
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
