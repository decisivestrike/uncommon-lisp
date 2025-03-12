use std::{
    io::{Write, stdout},
    vec::IntoIter,
};

use crate::{errors::RuntimeError, executer::execute, token::Token, utils::handle_escapes};

pub fn add(mut tokens: IntoIter<Token>) -> Result<Token, RuntimeError> {
    let mut result = 0.0;

    while let Some(token) = tokens.next() {
        let value = match token {
            Token::Expression(_) => match execute(token)? {
                Token::Number(value) => value,
                _ => Err(RuntimeError::TypeMismatch {
                    expected: "Number".to_string(),
                })?,
            },
            Token::Number(number) => number,
            _ => Err(RuntimeError::TypeMismatch {
                expected: "Number".to_string(),
            })?,
        };

        result += value;
    }

    Ok(Token::Number(result))
}

pub fn new_add<I>(tokens: I) -> Result<Token, RuntimeError>
where
    I: IntoIterator<Item = Token>,
{
    tokens
        .into_iter()
        .try_fold(0.0, |acc, token| {
            let value = evaluate<f64>(token)?;
            Ok(acc + value)
        })
        .map(Token::Number)
}

pub fn new_sub<I>(mut tokens: I) -> Result<Token, RuntimeError>
where
    I: IntoIterator<Item = Token>,
{
    let mut iter = tokens.into_iter();
    let base = iter
        .next()
        .ok_or_else(|| RuntimeError::TypeMismatch {
            expected: "Number".to_string(),
        })
        .and_then(evaluate_number)?;

    iter.try_fold(base, |acc, token| {
        let value = evaluate_number(token)?;
        Ok(acc - value)
    })
    .map(Token::Number)
}

pub fn sub(mut tokens: IntoIter<Token>) -> Result<Token, RuntimeError> {
    let mut base = match tokens.next() {
        Some(Token::Number(value)) => value,
        _ => Err(RuntimeError::TypeMismatch {
            expected: "Number".to_string(),
        })?,
    };

    while let Some(token) = tokens.next() {
        let value = match token {
            Token::Expression(_) => match execute(token)? {
                Token::Number(value) => value,
                _ => Err(RuntimeError::TypeMismatch {
                    expected: "Number".to_string(),
                })?,
            },
            Token::Number(number) => number,
            _ => Err(RuntimeError::TypeMismatch {
                expected: "Number".to_string(),
            })?,
        };

        base -= value;
    }

    Ok(Token::Number(base))
}

pub fn mul(mut tokens: IntoIter<Token>) -> Result<Token, RuntimeError> {
    let mut result = 1.0;

    while let Some(token) = tokens.next() {
        let value = match token {
            Token::Expression(_) => match execute(token)? {
                Token::Number(value) => value,
                _ => Err(RuntimeError::TypeMismatch {
                    expected: "Number".to_string(),
                })?,
            },
            Token::Number(number) => number,
            _ => Err(RuntimeError::TypeMismatch {
                expected: "Number".to_string(),
            })?,
        };

        result *= value;
    }

    Ok(Token::Number(result))
}

pub fn div(mut tokens: IntoIter<Token>) -> Result<Token, RuntimeError> {
    let mut base = match tokens.next() {
        Some(Token::Number(value)) => value,
        _ => Err(RuntimeError::TypeMismatch {
            expected: "Number".to_string(),
        })?,
    };

    while let Some(token) = tokens.next() {
        let value = match token {
            Token::Expression(_) => match execute(token)? {
                Token::Number(value) => value,
                _ => Err(RuntimeError::TypeMismatch {
                    expected: "Number".to_string(),
                })?,
            },
            Token::Number(number) => number,
            _ => Err(RuntimeError::TypeMismatch {
                expected: "Number".to_string(),
            })?,
        };

        base /= value;
    }

    Ok(Token::Number(base))
}

pub fn set_variable(mut tokens: IntoIter<Token>) -> Result<Token, RuntimeError> {
    if tokens.len() < 2 {
        return Err(RuntimeError::NotEnoughArgs);
    }

    let name = tokens.next().unwrap();

    if !matches!(name, Token::Identifier(_)) {
        return Err(RuntimeError::TypeMismatch {
            expected: "Identifier".to_string(),
        });
    }

    let value = tokens.next().unwrap();

    // Set name value

    todo!()
}

pub fn while_loop(mut tokens: IntoIter<Token>) -> Result<Token, RuntimeError> {
    todo!()
}

pub fn print(mut tokens: IntoIter<Token>) -> Result<Token, RuntimeError> {
    while let Some(token) = tokens.next() {
        let value = token.value()?;

        println!("{} ", handle_escapes(&value));
    }

    stdout().flush().unwrap();

    Ok(Token::Nil)
}

// Трейт для типов, которые можно извлечь из Token
trait Extractable: Sized {
    fn extract(token: &Token) -> Option<Self>;
    fn type_name() -> &'static str;
}

// Реализации Extractable для различных типов
impl Extractable for f64 {
    fn extract(token: &Token) -> Option<Self> {
        if let Token::Number(value) = token {
            Some(*value)
        } else {
            None
        }
    }

    fn type_name() -> &'static str {
        "Number"
    }
}

impl Extractable for String {
    fn extract(token: &Token) -> Option<Self> {
        if let Token::String(value) = token {
            Some(value.clone())
        } else {
            None
        }
    }

    fn type_name() -> &'static str {
        "String"
    }
}

impl Extractable for bool {
    fn extract(token: &Token) -> Option<Self> {
        if let Token::Boolean(value) = token {
            Some(*value)
        } else {
            None
        }
    }

    fn type_name() -> &'static str {
        "Boolean"
    }
}

fn execute(token: Token) -> Result<Token, RuntimeError> {
    // Реализация execute...
    unimplemented!()
}

fn evaluate<T: Extractable>(token: Token) -> Result<T, RuntimeError> {
    match token {
        Token::Expression(expr) => {
            if expr.is_empty() {
                return Err(RuntimeError::EmptyExpression);
            }
            execute(Token::Expression(expr)).and_then(|result| evaluate_inner(&result))
        }
        _ => evaluate_inner(&token),
    }
}

fn evaluate_inner<T: Extractable>(token: &Token) -> Result<T, RuntimeError> {
    T::extract(token).ok_or_else(|| RuntimeError::TypeMismatch {
        expected: T::type_name().to_string(),
        found: format!("{:?}", token),
    })
}

// Пример использования
fn add(tokens: Vec<Token>) -> Result<Token, RuntimeError> {
    tokens
        .into_iter()
        .try_fold(0.0, |acc, token| {
            let value: f64 = evaluate(token)?;
            Ok(acc + value)
        })
        .map(Token::Number)
}

fn concat(tokens: Vec<Token>) -> Result<Token, RuntimeError> {
    tokens
        .into_iter()
        .try_fold(String::new(), |acc, token| {
            let value: String = evaluate(token)?;
            Ok(acc + &value)
        })
        .map(Token::String)
}
