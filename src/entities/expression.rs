use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Expression {
    pub fid: Identifier,
    pub args: VecDeque<Entity>,
    pub line: usize,
    pub pos: usize,
}

impl Expression {
    pub fn execute(&self, scope: &mut Scope) -> Result<Value, RuntimeError> {
        todo!()
    }
}

impl ToEntity for Expression {
    fn to_entity(self) -> Entity {
        Entity::Expression(self)
    }
}


pub fn execute(e: Expression, scope: &mut Scope) -> Result<Primitive, RuntimeError> {

    if let Some(function)

    match builtins::FUNCTIONS.get(name.as_str()) {
        Some(func) => func(tokens, scope),
        None => match scope.get_function(&name) {
            Some((arg_names, body)) => execute(custom_func_call(arg_names, tokens, body), scope),
            None => Err(RuntimeError::UndefinedFunction(name)),
        },
    }
}

