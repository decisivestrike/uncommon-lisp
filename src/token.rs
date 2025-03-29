use std::{collections::VecDeque, fmt::Display};

use crate::utils::ULispType;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,

    List(VecDeque<Token>),
    Object(VecDeque<(Token, Token)>),

    Identifier(String),
    Expression(VecDeque<Token>),
}

impl Token {
    pub fn as_type(&self) -> ULispType {
        match self {
            Token::Number(_) => ULispType::Number,
            Token::String(_) => ULispType::String,
            Token::Bool(_) => ULispType::Bool,
            Token::Nil => ULispType::Nil,
            Token::List(_) => ULispType::List,
            Token::Object(_) => ULispType::Object,
            Token::Identifier(_) => ULispType::Identifier,
            Token::Expression(_) => ULispType::Expression,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Self::Expression(tokens) => format!(
                "({})",
                tokens
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            Self::Number(v) => v.to_string(),
            Self::String(v) | Self::Identifier(v) => v.to_string(),
            Self::Bool(v) => v.to_string(),
            Self::Nil => "Nil".to_string(),
            Self::List(items) => {
                let mut result = Vec::new();

                for item in items.into_iter() {
                    result.push(item.to_string());
                }

                result.join(" ").to_string()
            }
            Self::Object(fields) => {
                let mut result = Vec::new();

                for (key, value) in fields {
                    result.push(format!("{}:{}\n", key.to_string(), value.to_string()));
                }

                result.join(" ").to_string()
            }
        };

        write!(f, "{}", output)
    }
}
