use crate::entities::{Entity, Expression, Identifier, List, Primitive, Value};

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

impl ToEntity for Primitive {
    fn to_entity(self) -> Entity {
        Entity::Value(Value::Primitive(self))
    }
}

impl ToEntity for List {
    fn to_entity(self) -> Entity {
        Entity::Value(Value::List(self))
    }
}

impl ToEntity for Identifier {
    fn to_entity(self) -> Entity {
        Entity::Identifier(self)
    }
}

impl ToEntity for Expression {
    fn to_entity(self) -> Entity {
        Entity::Expression(self)
    }
}
