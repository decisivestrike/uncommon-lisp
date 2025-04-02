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
        let args = self.args.clone();

        match builtins::FUNCTIONS.get(func_id) {
            Some(func) => func.0(args, scope),

            None => match scope.get_function(&func_id.to_string()) {
                Some(f) => f.call(args, scope),
                None => Err(RuntimeError::UndefinedFunction(func_id.to_string())),
            },
        }
    }

    pub fn replace_all(&mut self, args: List, arg_names: List) -> Result<Expression, RuntimeError> {
        for (name, value) in arg_names.iter().zip(args.iter()) {
            while let Some(i) = self.args.iter().position(|t| *t == *name) {
                if self.args[i].as_type() == Datatype::Expression {
                    self.args[i] = self.args[i]
                        .clone()
                        .to_expression()?
                        .replace_all(args.clone(), arg_names.clone())?
                        .to_entity()
                }

                self.args[i] = value.clone();
            }
        }

        Ok(self.clone())
    }
}

impl ToEntity for Expression {
    fn to_entity(self) -> Entity {
        Entity::Expression(self)
    }
}
