use std::{
    collections::VecDeque,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

pub use expression::Expression;
pub use identifier::Identifier;
pub use list::List;
pub use primitive::Primitive;

use crate::{errors::RuntimeError, scope::Scope};

mod expression;
mod identifier;
mod list;
mod primitive;

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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Primitive(Primitive),
    List(List),
    Object,
}

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
}
