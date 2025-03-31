use std::{
    any::Any,
    collections::{HashMap, VecDeque},
    fmt::{Debug, Display},
};

use crate::{errors::RuntimeError, executer::execute, scope::Scope};
pub mod primitive;

pub enum Datatype {
    Number,
    String,
    Bool,
    Nil,

    List,

    Identifier,
    Expression,
}

// impl Display for Datatype {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let strlit = match self {
//             ULispType::Number => "number",
//             ULispType::String => "string",
//             ULispType::Bool => "bool",
//             ULispType::Nil => "nil",
//             ULispType::List => "list",
//             // ULispType::Object => "object",
//             ULispType::Identifier | ULispType::Expression => unreachable!("wtf?"),
//         };

//         write!(f, "{}", strlit)
//     }
// }

pub trait Entity: Any + Debug {
    fn as_type(&self) -> Datatype;

    fn as_any(&self) -> &dyn Any;
}

pub trait Value: Entity {}

pub trait Raw: Entity {
    fn to_value(self, scope: &mut Scope) -> Result<impl Value, RuntimeError>;
}

#[derive(Debug)]
pub struct List(pub VecDeque<Box<dyn Entity>>);

impl Entity for List {
    fn as_type(&self) -> Datatype {
        Datatype::List
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Value for List {}

// #[derive(Debug)]
// pub struct Object(pub HashMap<String, Box<dyn Value>>);

// impl Entity for Object {
//     fn as_type(&self) -> Datatype {
//         Datatype::Object
//     }
// }

// impl Value for Object {}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Identifier(pub String);

impl Entity for Identifier {
    fn as_type(&self) -> Datatype {
        Datatype::Identifier
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Raw for Identifier {
    fn to_value(self, scope: &mut Scope) -> Result<impl Value, RuntimeError> {
        Ok(scope.get_variable(&self.0))
    }
}

#[derive(Debug)]
pub struct Expression {
    pub fid: Identifier,
    pub args: VecDeque<Box<dyn Entity>>,
    pub line: usize,
    pub pos: usize,
}

impl Entity for Expression {
    fn as_type(&self) -> Datatype {
        Datatype::Expression
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Raw for Expression {
    fn to_value(self, scope: &mut Scope) -> Result<impl Value, RuntimeError> {
        execute(self, scope)
    }
}
