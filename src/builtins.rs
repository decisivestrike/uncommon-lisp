use std::{
    collections::HashMap,
    io::{Write, stdout},
};

use lazy_static::lazy_static;

use crate::{
    entities::{Identifier, List, Primitive, Value},
    errors::RuntimeError,
    extractor::evaluate,
    scope::Scope,
    utils::{get_token_strict, unescape},
};

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct BuiltinFunc(fn(List, &mut Scope) -> Result<Value, RuntimeError>);

lazy_static! {
    pub static ref FUNCTIONS: HashMap<String, BuiltinFunc> = [
        ("var", create_variable as fn(List, &mut Scope) -> Result<Value, RuntimeError>),
        ("func", create_function),
        ("typeof", typeof_),

        // Control flow
        ("if", if_then_else),
        // while

        // Comparing
        ("eq", equal),
        ("ne", not_equal),

        ("lt", less_then),
        ("gt", greater_then),

        ("le", less_or_equal),
        ("ge", greater_or_equal),

        // Math
        ("add", add),
        ("sub", sub),
        ("mul", mul),
        ("div", div),

        // Other
        ("concat", concat),
        ("print", print),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), BuiltinFunc(v)))
    .collect();
}

pub fn create_variable(mut args: List, scope: &mut Scope) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::InvalidArgCount {
            expected: 2,
            got: args.len(),
        });
    }

    let name: Identifier = evaluate(args.pop_front().unwrap(), scope)?;
    let value: Value = args.pop_front().unwrap().to_value(scope)?;

    scope.set_variable(name.to_string(), value);

    Ok(scope.get_variable(&name.to_string()))
}

pub fn create_function(mut tokens: List, scope: &mut Scope) -> Result<Value, RuntimeError> {
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

    return Ok(Primitive::Nil.to_value());
}

pub fn typeof_(mut tokens: List, scope: &mut Scope) -> Result<Value, RuntimeError> {
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

pub fn if_then_else(mut tokens: List, scope: &mut Scope) -> Result<Value, RuntimeError> {
    if tokens.len() < 2 {
        return Err(RuntimeError::NotEnoughArgs { min: 2 });
    }

    if tokens.len() > 3 {
        return Err(RuntimeError::TooMuchArgs { max: 3 });
    }

    let has_else_block = tokens.len() == 3;
    let condition: bool = evaluate(tokens.pop_front().unwrap(), scope)?;

    let then_block = tokens.pop_front().unwrap();
    let else_block = match has_else_block {
        true => tokens.pop_front().unwrap(),
        false => Token::Nil,
    };

    Ok(match condition {
        true => then_block.to_primitive(scope)?,
        false => else_block.to_primitive(scope)?,
    })
}

// General
pub fn compare(mut tokens: List, op: &str) -> Result<Value, RuntimeError> {
    if tokens.len() < 2 {
        return Err(RuntimeError::NotEnoughArgs { min: 2 });
    }

    let action: fn(&Token, &Token) -> bool = match op {
        "==" => |a, b| a == b,
        "!=" => |a, b| a != b,
        "<" => |a, b| a < b,
        ">" => |a, b| a > b,
        "<=" => |a, b| a <= b,
        ">=" => |a, b| a >= b,
        _ => unreachable!(),
    };

    let base = tokens.pop_front().unwrap();

    for t in tokens {
        if !action(&base, &t) {
            return Ok(Token::Bool(false));
        }
    }

    Ok(Token::Bool(true))
}

pub fn equal(tokens: List, _: &mut Scope) -> Result<Token, RuntimeError> {
    compare(tokens, "==")
}

pub fn not_equal(tokens: List, _: &mut Scope) -> Result<Token, RuntimeError> {
    compare(tokens, "!=")
}

pub fn less_then(tokens: List, _: &mut Scope) -> Result<Token, RuntimeError> {
    compare(tokens, "<")
}

pub fn greater_then(tokens: List, _: &mut Scope) -> Result<Token, RuntimeError> {
    compare(tokens, ">")
}

pub fn less_or_equal(tokens: List, _: &mut Scope) -> Result<Token, RuntimeError> {
    compare(tokens, "<=")
}

pub fn greater_or_equal(tokens: List, _: &mut Scope) -> Result<Token, RuntimeError> {
    compare(tokens, "=>")
}

pub fn add(tokens: List, scope: &mut Scope) -> Result<Value, RuntimeError> {
    tokens
        .into_iter()
        .try_fold(0.0, |acc, token| {
            let value: f64 = evaluate(token, scope)?;
            Ok(acc + value)
        })
        .map(|v| Primitive::Number(v).to_value())
}

pub fn sub(tokens: List, scope: &mut Scope) -> Result<Value, RuntimeError> {
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
        .map(|v| Primitive::Number(v).to_value())
}

pub fn mul(tokens: List, scope: &mut Scope) -> Result<Value, RuntimeError> {
    tokens
        .into_iter()
        .try_fold(1.0, |acc, token| {
            let value: f64 = evaluate(token, scope)?;
            Ok(acc * value)
        })
        .map(|v| Primitive::Number(v).to_value())
}

pub fn div(tokens: List, scope: &mut Scope) -> Result<Value, RuntimeError> {
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

pub fn concat(tokens: List, scope: &mut Scope) -> Result<Value, RuntimeError> {
    tokens
        .into_iter()
        .try_fold(String::new(), |acc, token| {
            let value: String = evaluate(token, scope)?;
            Ok(acc + &value)
        })
        .map(|v| Primitive::String(v).to_value())
}

pub fn print(tokens: List, scope: &mut Scope) -> Result<Value, RuntimeError> {
    let mut parts = Vec::new();

    for token in tokens.0 {
        let value = token.to_value(scope)?;

        parts.push(value.to_string());
    }

    let output = parts.join(" ");

    print!("{}", unescape(&output));
    stdout().flush().unwrap();

    Ok(Primitive::Nil.to_value())
}
