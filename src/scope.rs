use std::collections::HashMap;

use crate::{
    entities::{AsType, Datatype, Expression, List, Primitive, ToEntity, Value},
    errors::RuntimeError,
};

#[derive(Debug, Clone)]
pub struct UserFunction {
    pub body: Expression,
    pub arg_names: List,
}

impl UserFunction {
    pub fn call(mut self, args: List, scope: &mut Scope) -> Result<Value, RuntimeError> {
        // TODO: add recursive parsing support
        for (name, value) in self.arg_names.iter().zip(args.iter()) {
            while let Some(i) = self.body.args.iter().position(|t| *t == *name) {
                if self.body.args[i].as_type() == Datatype::Expression {
                    self.body.args[i] = self.body.args[i]
                        .clone()
                        .to_expression()?
                        .replace_all(args.clone(), self.arg_names.clone())?
                        .to_entity()
                }

                self.body.args[i] = value.clone();
            }
        }

        self.body.execute(scope)
    }
}

pub struct Scope {
    pub variables: HashMap<String, Value>,
    pub functions: HashMap<String, UserFunction>,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn get_variable(&self, name: &String) -> Value {
        match self.variables.get(name) {
            Some(v) => v.clone(),
            None => Primitive::Nil.to_value(),
        }
    }

    pub fn add_function(&mut self, name: String, arg_names: List, body: Expression) {
        self.functions
            .insert(name, UserFunction { body, arg_names });
    }

    pub fn get_function(&mut self, name: &String) -> Option<UserFunction> {
        self.functions.get(name).cloned()
    }
}
