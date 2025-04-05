use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
    sync::RwLock,
};

use once_cell::sync::Lazy;

use crate::{
    errors::RuntimeError,
    token::{Expression, Identifier, List, Token},
};

pub static VARIABLES: Lazy<RwLock<HashMap<String, Token>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub static FUNCTIONS: Lazy<RwLock<HashMap<String, Function>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

#[derive(Debug, Clone)]
pub struct Function {
    arg_names: Vec<String>,
    body: Expression,
}

impl Function {
    pub fn new(body: Expression, arg_names: List) -> Result<Self, RuntimeError> {
        let arg_names: Vec<String> = {
            let mut v = vec![];

            for token in arg_names.0 {
                v.push(token.extract::<Identifier>(None)?.0);
            }

            v
        };

        Ok(Self { arg_names, body })
    }

    pub fn call(self, name: String, args: List) -> Result<Token, RuntimeError> {
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        let prefix = hasher.finish().to_string();

        self.arg_names
            .iter()
            .zip(args.0.into_iter())
            .for_each(|(name, value)| {
                set_variable(&(prefix.clone() + &name), value);
            });

        self.body.execute(Some(prefix))
    }
}

pub fn get_variable(name: &String) -> Token {
    let variables = VARIABLES.read().unwrap();

    match variables.get(name) {
        Some(v) => v.clone(),
        None => Token::Nil,
    }
}

pub fn set_variable(name: &String, value: Token) {
    let mut variables = VARIABLES.write().unwrap();

    variables.insert(name.clone(), value);
}

pub fn get_function(name: &String) -> Option<Function> {
    let functions = FUNCTIONS.read().unwrap();

    functions.get(name).cloned()
}

pub fn set_function(name: &String, arg_names: List, body: Expression) -> Result<(), RuntimeError> {
    let mut functions = FUNCTIONS.write().unwrap();

    functions.insert(name.clone(), Function::new(body, arg_names)?);

    Ok(())
}
