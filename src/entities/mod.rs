use std::{
    collections::VecDeque,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

pub use expression::Expression;
pub use identifier::Identifier;
pub use list::List;
pub use primitive::Primitive;
pub use value::Value;

use crate::{errors::RuntimeError, scope::Scope};

mod expression;
mod identifier;
mod list;
mod primitive;
mod value;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Entity {
    Value(Value),
    Identifier(Identifier),
    Expression(Expression),
}

impl Entity {
    pub fn to_value(self, scope: &mut Scope) -> Result<Value, RuntimeError> {
        match self {
            Entity::Value(v) => Ok(v),
            Entity::Identifier(id) => Ok(id.value_from(scope)),
            Entity::Expression(e) => e.execute(scope),
        }
    }

    pub fn to_id(self) -> Result<Identifier, RuntimeError> {
        match self {
            Entity::Identifier(id) => Ok(id),
            _ => Err(RuntimeError::TypeMismatch {
                expected: Datatype::Identifier,
                found: self.as_type(),
            }),
        }
    }

    pub fn to_expression(self) -> Result<Expression, RuntimeError> {
        match self {
            Entity::Expression(e) => Ok(e),
            _ => Err(RuntimeError::TypeMismatch {
                expected: Datatype::Expression,
                found: self.as_type(),
            }),
        }
    }

    // pub fn into(self) -> Result<Self, RuntimeError> {
    //     if self.as_type() != Self
    // }
}

impl AsType for Entity {
    fn as_type(&self) -> Datatype {
        match self {
            Entity::Value(value) => value.as_type(),
            Entity::Identifier(_) => Datatype::Identifier,
            Entity::Expression(_) => Datatype::Expression,
        }
    }
}

pub trait ToEntity {
    fn to_entity(self) -> Entity;
}

impl ToEntity for f64 {
    fn to_entity(self) -> Entity {
        Entity::Value(Value::Primitive(Primitive::Number(self)))
    }
}

impl ToEntity for String {
    fn to_entity(self) -> Entity {
        Entity::Value(Value::Primitive(Primitive::String(self)))
    }
}

impl ToEntity for bool {
    fn to_entity(self) -> Entity {
        Entity::Value(Value::Primitive(Primitive::Bool(self)))
    }
}

pub trait AsType {
    fn as_type(&self) -> Datatype;
}

impl AsType for Primitive {
    fn as_type(&self) -> Datatype {
        match self {
            Primitive::Number(_) => Datatype::Number,
            Primitive::String(_) => Datatype::String,
            Primitive::Bool(_) => Datatype::Bool,
            Primitive::Nil => Datatype::Nil,
        }
    }
}

impl AsType for Value {
    fn as_type(&self) -> Datatype {
        match self {
            Value::Primitive(primitive) => primitive.as_type(),
            Value::List(_) => Datatype::List,
            Value::Object => Datatype::Object,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Datatype {
    Number,
    String,
    Bool,
    Nil,

    List,
    Object,

    Identifier,
    Expression,

    Primitive,
    Any, // Special
}
