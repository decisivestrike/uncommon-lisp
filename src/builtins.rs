use std::{
    collections::HashMap,
    io::{Write, stdout},
};

use lazy_static::lazy_static;

use crate::{
    entities::{
        Datatype, Entity, List, Primitive, Value,
        traits::{AsType, ToEntity},
    },
    errors::RuntimeError,
    scope::Scope,
    utils::unescape,
};

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct BuiltinFunc(pub fn(List, &mut Scope) -> Result<Value, RuntimeError>);

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

    let id = args.pop_front().unwrap().to_id()?;
    let value = args.pop_front().unwrap().to_value(scope)?;

    scope.set_variable(id.0.clone(), value);

    Ok(scope.get_variable(&id))
}

pub fn create_function(mut args: List, scope: &mut Scope) -> Result<Value, RuntimeError> {
    if args.len() != 3 {
        return Err(RuntimeError::InvalidArgCount {
            expected: 3,
            got: args.len(),
        });
    }

    let name = args.get().to_id()?;
    let list_names = args.get().to_value(scope)?.to_list()?;
    let body = args.get().to_expression()?;

    scope.add_function(name.to_string(), list_names, body);

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
        Entity::Identifier(id) => {
            if scope.variables.contains_key(&id.0) {
                scope.get_variable(&id.0).as_type()
            } else if scope.functions.contains_key(&id.0) || FUNCTIONS.contains_key(&id.0) {
                Datatype::Expression
            } else {
                Datatype::Nil
            }
        }
        Entity::Expression(e) => e.execute(scope)?.as_type(),
        t => t.as_type(),
    };

    Ok(Primitive::String(format!("{:#?}", type_)).to_value())
}

pub fn if_then_else(mut tokens: List, scope: &mut Scope) -> Result<Value, RuntimeError> {
    if tokens.len() < 2 {
        return Err(RuntimeError::NotEnoughArgs { min: 2 });
    }

    if tokens.len() > 3 {
        return Err(RuntimeError::TooMuchArgs { max: 3 });
    }

    let has_else_block = tokens.len() == 3;

    let condition = tokens
        .pop_front()
        .unwrap()
        .to_value(scope)?
        .to_primitive()?
        .as_bool();

    let then_block = tokens.pop_front().unwrap();

    let else_block = match has_else_block {
        true => tokens.pop_front().unwrap(),
        false => Primitive::Nil.to_entity(),
    };

    Ok(match condition {
        true => then_block.to_value(scope)?,
        false => else_block.to_value(scope)?,
    })
}

// General
pub fn compare(mut tokens: List, op: &str) -> Result<Value, RuntimeError> {
    if tokens.len() < 2 {
        return Err(RuntimeError::NotEnoughArgs { min: 2 });
    }

    let action: fn(&Entity, &Entity) -> bool = match op {
        "==" => |a, b| a == b,
        "!=" => |a, b| a != b,
        "<" => |a, b| a < b,
        ">" => |a, b| a > b,
        "<=" => |a, b| a <= b,
        ">=" => |a, b| a >= b,
        _ => unreachable!(),
    };

    let base = tokens.pop_front().unwrap();

    for t in tokens.0.into_iter() {
        if !action(&base, &t) {
            return Ok(Primitive::Bool(false).to_value());
        }
    }

    Ok(Primitive::Bool(true).to_value())
}

pub fn equal(tokens: List, _: &mut Scope) -> Result<Value, RuntimeError> {
    compare(tokens, "==")
}

pub fn not_equal(tokens: List, _: &mut Scope) -> Result<Value, RuntimeError> {
    compare(tokens, "!=")
}

pub fn less_then(tokens: List, _: &mut Scope) -> Result<Value, RuntimeError> {
    compare(tokens, "<")
}

pub fn greater_then(tokens: List, _: &mut Scope) -> Result<Value, RuntimeError> {
    compare(tokens, ">")
}

pub fn less_or_equal(tokens: List, _: &mut Scope) -> Result<Value, RuntimeError> {
    compare(tokens, "<=")
}

pub fn greater_or_equal(tokens: List, _: &mut Scope) -> Result<Value, RuntimeError> {
    compare(tokens, "=>")
}

pub fn add(args: List, scope: &mut Scope) -> Result<Value, RuntimeError> {
    args.0
        .into_iter()
        .try_fold(0.0, |acc, arg| {
            let value: f64 = arg.to_value(scope)?.to_primitive()?.as_number()?;
            Ok(acc + value)
        })
        .map(|v| Primitive::Number(v).to_value())
}

pub fn sub(tokens: List, scope: &mut Scope) -> Result<Value, RuntimeError> {
    let mut tokens = tokens.0.into_iter();

    if tokens.len() < 2 {
        return Err(RuntimeError::NotEnoughArgs { min: 2 });
    }

    let base: f64 = tokens
        .next()
        .unwrap()
        .to_value(scope)?
        .to_primitive()?
        .as_number()?;

    tokens
        .try_fold(base, |acc, token| {
            let value: f64 = token.to_value(scope)?.to_primitive()?.as_number()?;
            Ok(acc - value)
        })
        .map(|v| Primitive::Number(v).to_value())
}

pub fn mul(tokens: List, scope: &mut Scope) -> Result<Value, RuntimeError> {
    tokens
        .0
        .into_iter()
        .try_fold(1.0, |acc, token| {
            let value: f64 = token.to_value(scope)?.to_primitive()?.as_number()?;
            Ok(acc * value)
        })
        .map(|v| Primitive::Number(v).to_value())
}

pub fn div(tokens: List, scope: &mut Scope) -> Result<Value, RuntimeError> {
    let mut tokens = tokens.0.into_iter();

    if tokens.len() < 2 {
        return Err(RuntimeError::NotEnoughArgs { min: 2 });
    }

    let base: f64 = tokens
        .next()
        .unwrap()
        .to_value(scope)?
        .to_primitive()?
        .as_number()?;

    tokens
        .try_fold(base, |acc, token| {
            let value: f64 = token.to_value(scope)?.to_primitive()?.as_number()?;
            Ok(acc / value)
        })
        .map(|v| Primitive::Number(v).to_value())
}

pub fn concat(tokens: List, scope: &mut Scope) -> Result<Value, RuntimeError> {
    tokens
        .0
        .into_iter()
        .try_fold(String::new(), |acc, token| {
            let value: String = token.to_value(scope)?.to_primitive()?.as_string();
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
