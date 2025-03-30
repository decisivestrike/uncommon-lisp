use super::{Datatype, Entity, Value};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Primitive {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl Entity for Primitive {
    fn as_type(&self) -> Datatype {
        match self {
            Self::Number(_) => Datatype::Number,
            Self::String(_) => Datatype::String,
            Self::Bool(_) => Datatype::Bool,
            Self::Nil => Datatype::Nil,
        }
    }
}

impl Value for Primitive {}
