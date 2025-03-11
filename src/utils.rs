use crate::{errors::RuntimeError, executer::execute, token::Token};

pub enum Type {
    Number,
    String,
    Bool,
    Nil,

    List,
    Object,

    Identifier,
    Expression,
}

pub fn handle_escapes(s: &str) -> String {
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

// fn evaluate_number(token: Token) -> Result<f64, RuntimeError> {
//     match token {
//         Token::Number(value) => Ok(value),
//         Token::Expression(expr) => execute(token)?.and_then(|result| match result {
//             Token::Number(value) => Ok(value),
//             other => Err(RuntimeError::TypeMismatch {
//                 expected: "Number".to_string(),
//                 found: format!("{:?}", other),
//             }),
//         }),
//         other => Err(RuntimeError::TypeMismatch {
//             expected: "Number".to_string(),
//             found: format!("{:?}", other),
//         }),
//     }
// }

// fn evaluate<T>()
