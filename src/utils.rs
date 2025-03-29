use std::{collections::VecDeque, fmt::Display};

use crate::{errors::RuntimeError, executer::execute, scope::Scope, token::Token};

#[derive(Debug, PartialEq)]
pub enum ULispType {
    Number,
    String,
    Bool,
    Nil,

    List,
    // Object,
    Identifier,
    Expression,
}

impl Display for ULispType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let strlit = match self {
            ULispType::Number => "number",
            ULispType::String => "string",
            ULispType::Bool => "bool",
            ULispType::Nil => "nil",
            ULispType::List => "list",
            // ULispType::Object => "object",
            ULispType::Identifier | ULispType::Expression => unreachable!("wtf?"),
        };

        write!(f, "{}", strlit)
    }
}

pub fn unescape(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('t') => result.push('\t'),
                Some('\\') => result.push('\\'),
                Some(c) => result.push(c),
                None => break,
            }
        } else {
            result.push(c);
        }
    }

    result
}

// pub fn check_types<A, T>(args: A, types: T) -> Result<(), RuntimeError>
// where
//     A: IntoIterator<Item = Token> + Clone,
//     T: IntoIterator<Item = ULispType>,
// {
//     for (arg, t) in args.clone().into_iter().zip(types.into_iter()) {
//         if arg.as_type() != t {
//             return Err(RuntimeError::TypeMismatch {
//                 expected: arg.as_type(),
//                 found: t,
//             });
//         }
//     }

//     Ok(())
// }

pub fn get_token_strict(
    tokens: &mut VecDeque<Token>,
    type_: ULispType,
) -> Result<Token, RuntimeError> {
    match tokens.pop_front().unwrap() {
        token if token.as_type() == type_ => Ok(token),
        token => Err(RuntimeError::TypeMismatch {
            expected: type_,
            found: token.as_type(),
        }),
    }
}

pub fn get_value_token(
    tokens: &mut VecDeque<Token>,
    scope: &mut Scope,
) -> Result<Token, RuntimeError> {
    match tokens.pop_front().unwrap() {
        Token::Identifier(name) => Ok(scope.get_variable(&name)),
        t @ Token::Expression(_) => execute(t, scope),
        t => Ok(t),
    }
}
