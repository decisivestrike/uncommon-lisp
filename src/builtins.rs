use std::{
    collections::{HashMap, VecDeque},
    io::{Write, stdout},
};

use lazy_static::lazy_static;

use crate::{
    errors::RuntimeError,
    executer::execute,
    extractor::evaluate,
    scope::Scope,
    token::Token,
    utils::{ULispType, get_token_strict, get_value_token, unescape},
};

type BuiltinFunc = fn(VecDeque<Token>, &mut Scope) -> Result<Token, RuntimeError>;

lazy_static! {
    pub static ref FUNCTIONS: HashMap<String, BuiltinFunc> = [
        ("var", create_variable as BuiltinFunc),
        ("func", create_function),
        ("typeof", typeof_),

        // Math
        ("add", add),
        ("sub", sub),
        ("mul", mul),
        ("div", div),

        ("concat", concat),
        ("print", print),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v))
    .collect();
}

pub fn create_variable(
    mut tokens: VecDeque<Token>,
    scope: &mut Scope,
) -> Result<Token, RuntimeError> {
    if tokens.len() != 2 {
        return Err(RuntimeError::InvalidArgCount {
            expected: 2,
            got: tokens.len(),
        });
    }

    let name = get_token_strict(&mut tokens, ULispType::Identifier)?;
    let value = get_value_token(&mut tokens, scope)?;

    scope.set_variable(name.to_string(), value);

    Ok(scope.get_variable(&name.to_string()))
}

pub fn create_function(
    mut tokens: VecDeque<Token>,
    scope: &mut Scope,
) -> Result<Token, RuntimeError> {
    if tokens.len() != 3 {
        return Err(RuntimeError::InvalidArgCount {
            expected: 3,
            got: tokens.len(),
        });
    }

    let name = get_token_strict(&mut tokens, ULispType::Identifier)?;
    let Token::List(args) = get_token_strict(&mut tokens, ULispType::List)? else {
        unreachable!()
    };
    let body = get_token_strict(&mut tokens, ULispType::Expression)?;

    scope.add_function(name.to_string(), args, body);

    return Ok(Token::Nil);
}

pub fn typeof_(mut tokens: VecDeque<Token>, scope: &mut Scope) -> Result<Token, RuntimeError> {
    if tokens.len() != 1 {
        return Err(RuntimeError::InvalidArgCount {
            expected: 1,
            got: tokens.len(),
        });
    }

    let type_ = match tokens.pop_front().unwrap() {
        Token::Identifier(name) => {
            if scope.variables.contains_key(&name) {
                scope.get_variable(&name).as_type()
            } else if scope.functions.contains_key(&name) || FUNCTIONS.contains_key(&name) {
                ULispType::Expression
            } else {
                ULispType::Nil
            }
        }
        t @ Token::Expression(_) => execute(t, scope)?.as_type(),
        t => t.as_type(),
    };

    Ok(Token::String(type_.to_string()))
}

pub fn add(tokens: VecDeque<Token>, scope: &mut Scope) -> Result<Token, RuntimeError> {
    tokens
        .into_iter()
        .try_fold(0.0, |acc, token| {
            let value: f64 = evaluate(token, scope)?;
            Ok(acc + value)
        })
        .map(Token::Number)
}

pub fn sub(tokens: VecDeque<Token>, scope: &mut Scope) -> Result<Token, RuntimeError> {
    let mut tokens = tokens.into_iter();

    if tokens.len() < 2 {
        return Err(RuntimeError::NotEnoughArgs { min: 2 });
    }

    let base: f64 = evaluate(tokens.next().unwrap(), scope)?;

    tokens
        .try_fold(base, |acc, token| {
            let value: f64 = evaluate(token, scope)?;
            Ok(acc - value)
        })
        .map(Token::Number)
}

pub fn mul(tokens: VecDeque<Token>, scope: &mut Scope) -> Result<Token, RuntimeError> {
    tokens
        .into_iter()
        .try_fold(1.0, |acc, token| {
            let value: f64 = evaluate(token, scope)?;
            Ok(acc * value)
        })
        .map(Token::Number)
}

pub fn div(tokens: VecDeque<Token>, scope: &mut Scope) -> Result<Token, RuntimeError> {
    let mut tokens = tokens.into_iter();

    if tokens.len() < 2 {
        return Err(RuntimeError::NotEnoughArgs { min: 2 });
    }

    let base: f64 = evaluate(tokens.next().unwrap(), scope)?;

    tokens
        .try_fold(base, |acc, token| {
            let value: f64 = evaluate(token, scope)?;
            Ok(acc / value)
        })
        .map(Token::Number)
}

pub fn concat(tokens: VecDeque<Token>, scope: &mut Scope) -> Result<Token, RuntimeError> {
    tokens
        .into_iter()
        .try_fold(String::new(), |acc, token| {
            let value: String = evaluate(token, scope)?;
            Ok(acc + &value)
        })
        .map(Token::String)
}

pub fn print(tokens: VecDeque<Token>, scope: &mut Scope) -> Result<Token, RuntimeError> {
    let mut parts = Vec::new();

    for token in tokens {
        let value = match token {
            Token::Identifier(name) => scope.get_variable(&name),
            Token::Expression(_) => execute(token, scope)?,
            _ => token,
        };

        parts.push(value.to_string());
    }

    let output = parts.join(" ");

    print!("{}", unescape(&output));
    stdout().flush().unwrap();

    Ok(Token::Nil)
}
