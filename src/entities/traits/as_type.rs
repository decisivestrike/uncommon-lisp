use crate::entities::{Datatype, Entity, Primitive, Value};

pub trait AsType {
    fn as_type(&self) -> Datatype;
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

impl AsType for Value {
    fn as_type(&self) -> Datatype {
        match self {
            Value::Primitive(primitive) => primitive.as_type(),
            Value::List(_) => Datatype::List,
            Value::Object => Datatype::Object,
        }
    }
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
