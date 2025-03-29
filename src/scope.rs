use std::collections::{HashMap, VecDeque};

use crate::token::Token;

pub struct Scope {
    pub variables: HashMap<String, Token>,
    pub functions: HashMap<String, (VecDeque<Token>, Token)>,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, name: String, value: Token) {
        self.variables.insert(name, value);
    }

    pub fn get_variable(&self, name: &String) -> Token {
        match self.variables.get(name) {
            Some(v) => v.clone(),
            None => Token::Nil,
        }
    }

    pub fn add_function(&mut self, name: String, args: VecDeque<Token>, body: Token) {
        self.functions.insert(name, (args, body));
    }

    pub fn get_function(&mut self, name: &String) -> Option<(VecDeque<Token>, Token)> {
        self.functions.get(name).cloned()
    }
}
