use std::{
    collections::HashMap,
    io::{Write, stdout},
};

use once_cell::sync::Lazy;

use crate::{
    errors::RuntimeError,
    scope::{FUNCTIONS, VARIABLES, get_variable, set_function, set_variable},
    token::{Expression, Identifier, List, Token},
    utils::unescape,
};

type BuiltinFunc = fn(List, Option<String>) -> Result<Token, RuntimeError>;

pub static BUILTIN_FUNCTIONS: Lazy<HashMap<String, BuiltinFunc>> = Lazy::new(|| {
    [
        ("var", create_variable as BuiltinFunc),
        ("func", create_function),
        ("typeof", typeof_),
        // Control flow
        ("if", if_then_else),
        // while

        // Comparing
        // ("eq", equal),
        // ("ne", not_equal),
        // ("lt", less_then),
        // ("gt", greater_then),
        // ("le", less_or_equal),
        // ("ge", greater_or_equal),

        // // Math
        ("add", add),
        ("sub", sub),
        ("mul", mul),
        ("div", div),
        // // Other
        ("concat", concat),
        ("print", print),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v))
    .collect()
});

pub fn create_variable(
    mut tokens: List,
    maybe_prefix: Option<String>,
) -> Result<Token, RuntimeError> {
    if tokens.0.len() != 2 {
        return Err(RuntimeError::InvalidArgCount {
            expected: 2,
            got: tokens.0.len(),
        });
    }

    let id: Identifier = tokens.0.pop_front().unwrap().extract(None)?;

    let value = tokens.0.pop_front().unwrap().into_value(maybe_prefix)?;

    set_variable(&id.0, value);

    Ok(get_variable(&id.0))
}

pub fn create_function(mut tokens: List, _: Option<String>) -> Result<Token, RuntimeError> {
    if tokens.0.len() != 3 {
        return Err(RuntimeError::InvalidArgCount {
            expected: 3,
            got: tokens.0.len(),
        });
    }

    let id: Identifier = tokens.0.pop_front().unwrap().extract(None)?;
    let arg_names: List = tokens.0.pop_front().unwrap().extract(None)?;
    let body: Expression = tokens.0.pop_front().unwrap().extract(None)?;

    set_function(&id.0, arg_names, body)?;

    return Ok(Token::Nil);
}

pub fn typeof_(mut tokens: List, maybe_prefix: Option<String>) -> Result<Token, RuntimeError> {
    if tokens.len() != 1 {
        return Err(RuntimeError::InvalidArgCount {
            expected: 1,
            got: tokens.len(),
        });
    }

    let type_ = match tokens.pop_front().unwrap() {
        Token::Identifier(name) => {
            if VARIABLES.read().unwrap().contains_key(&name.0) {
                get_variable(&name).as_type()
            } else if FUNCTIONS.read().unwrap().contains_key(&name.0)
                || BUILTIN_FUNCTIONS.contains_key(&name.0)
            {
                "function".to_string()
            } else {
                return Err(RuntimeError::UndefinedFunction(name.0));
            }
        }
        t @ Token::Expression(_) => t.into_value(maybe_prefix)?.as_type(),
        t => t.as_type(),
    };

    Ok(Token::String(type_))
}

pub fn if_then_else(mut tokens: List, maybe_prefix: Option<String>) -> Result<Token, RuntimeError> {
    if tokens.len() < 2 {
        return Err(RuntimeError::NotEnoughArgs { min: 2 });
    }

    if tokens.len() > 3 {
        return Err(RuntimeError::TooMuchArgs { max: 3 });
    }

    let has_else_block = tokens.len() == 3;
    let condition: bool = tokens.pop_front().unwrap().extract(maybe_prefix.clone())?;

    let then_block = tokens.pop_front().unwrap();
    let else_block = match has_else_block {
        true => tokens.pop_front().unwrap(),
        false => Token::Nil,
    };

    Ok(match condition {
        true => then_block.into_value(maybe_prefix)?,
        false => else_block.into_value(maybe_prefix)?,
    })
}

// General
// pub fn compare(mut tokens: List, op: &str) -> Result<Token, RuntimeError> {
//     if tokens.len() < 2 {
//         return Err(RuntimeError::NotEnoughArgs { min: 2 });
//     }

//     let action: fn(&Token, &Token) -> bool = match op {
//         "==" => |a, b| a == b,
//         "!=" => |a, b| a != b,
//         "<" => |a, b| a < b,
//         ">" => |a, b| a > b,
//         "<=" => |a, b| a <= b,
//         ">=" => |a, b| a >= b,
//         _ => unreachable!(),
//     };

//     let base = tokens.pop_front().unwrap();

//     for t in tokens {
//         if !action(&base, &t) {
//             return Ok(Token::Bool(false));
//         }
//     }

//     Ok(Token::Bool(true))
// }

// pub fn equal(tokens: List) -> Result<Token, RuntimeError> {
//     compare(tokens, "==")
// }

// pub fn not_equal(tokens: List) -> Result<Token, RuntimeError> {
//     compare(tokens, "!=")
// }

// pub fn less_then(tokens: List) -> Result<Token, RuntimeError> {
//     compare(tokens, "<")
// }

// pub fn greater_then(tokens: List) -> Result<Token, RuntimeError> {
//     compare(tokens, ">")
// }

// pub fn less_or_equal(tokens: List) -> Result<Token, RuntimeError> {
//     compare(tokens, "<=")
// }

// pub fn greater_or_equal(tokens: List) -> Result<Token, RuntimeError> {
//     compare(tokens, "=>")
// }

pub fn add(tokens: List, maybe_prefix: Option<String>) -> Result<Token, RuntimeError> {
    tokens
        .0
        .into_iter()
        .try_fold(0.0, |acc, token| {
            let value: f64 = token.extract(maybe_prefix.clone())?;
            Ok(acc + value)
        })
        .map(Token::Number)
}

pub fn sub(tokens: List, maybe_prefix: Option<String>) -> Result<Token, RuntimeError> {
    let mut tokens = tokens.0.into_iter();

    if tokens.len() < 2 {
        return Err(RuntimeError::NotEnoughArgs { min: 2 });
    }

    let base: f64 = tokens.next().unwrap().extract(maybe_prefix.clone())?;

    tokens
        .try_fold(base, |acc, token| {
            let value: f64 = token.extract(maybe_prefix.clone())?;
            Ok(acc - value)
        })
        .map(Token::Number)
}

pub fn mul(tokens: List, maybe_prefix: Option<String>) -> Result<Token, RuntimeError> {
    tokens
        .0
        .into_iter()
        .try_fold(1.0, |acc, token| {
            let value: f64 = token.extract(maybe_prefix.clone())?;
            Ok(acc * value)
        })
        .map(Token::Number)
}

pub fn div(tokens: List, maybe_prefix: Option<String>) -> Result<Token, RuntimeError> {
    let mut tokens = tokens.0.into_iter();

    if tokens.len() < 2 {
        return Err(RuntimeError::NotEnoughArgs { min: 2 });
    }

    let base: f64 = tokens.next().unwrap().extract(maybe_prefix.clone())?;

    tokens
        .try_fold(base, |acc, token| {
            let value: f64 = token.extract(maybe_prefix.clone())?;
            Ok(acc / value)
        })
        .map(Token::Number)
}

pub fn concat(tokens: List, maybe_prefix: Option<String>) -> Result<Token, RuntimeError> {
    tokens
        .0
        .into_iter()
        .try_fold(String::new(), |acc, token| {
            let value: String = token.extract(maybe_prefix.clone())?;
            Ok(acc + &value)
        })
        .map(Token::String)
}

pub fn print(tokens: List, maybe_prefix: Option<String>) -> Result<Token, RuntimeError> {
    let mut parts = Vec::new();

    for token in tokens.0 {
        let value = token.into_value(maybe_prefix.clone())?;

        parts.push(value.to_string());
    }

    let output = parts.join(" ");

    print!("{}", unescape(&output));
    stdout().flush().unwrap();

    Ok(Token::Nil)
}
