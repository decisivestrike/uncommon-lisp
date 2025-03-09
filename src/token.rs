use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,

    List(Vec<Token>),
    Object(Vec<(Token, Token)>),

    Identifier(String),
    Expression(Vec<Token>),
}

impl Token {
    pub fn value(self) -> String {
        match self {
            Self::Number(value) => value.to_string(),
            Self::String(value) | Self::Identifier(value) => value,
            Self::Bool(value) => value.to_string(),
            Self::Nil => "Nil".to_string(),
            _ => todo!(),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
