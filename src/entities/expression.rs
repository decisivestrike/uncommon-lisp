use crate::builtins;

use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Expression {
    pub fid: Identifier,
    pub args: List,
    pub line: usize,
    pub pos: usize,
}

impl Expression {
    pub fn execute(&self, scope: &mut Scope) -> Result<Value, RuntimeError> {
        let func_id = self.fid.as_str();

        match builtins::FUNCTIONS.get(func_id) {
            Some(func) => func(self.args, scope),

            None => match scope.get_function(&func_id.to_string()) {
                Some(f) => f.call(self.args, scope),
                None => Err(RuntimeError::UndefinedFunction(func_id.to_string())),
            },
        }
    }
}

impl ToEntity for Expression {
    fn to_entity(self) -> Entity {
        Entity::Expression(self)
    }
}
