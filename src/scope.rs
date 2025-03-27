use std::collections::HashMap;

use crate::token::Token;

pub struct Scope {
    variables: HashMap<String, Token>,
    // functions: HashMap<String, Token>,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            variables: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, name: String, value: Token) {
        self.variables.insert(name, value);
    }

    pub fn get_variable(&self, name: String) -> Token {
        match self.variables.get(&name) {
            Some(v) => v.clone(),
            None => Token::Nil,
        }
    }
}
