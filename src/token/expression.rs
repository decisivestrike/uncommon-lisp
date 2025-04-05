use std::fmt::Display;

use crate::{builtins::BUILTIN_FUNCTIONS, errors::RuntimeError, scope::get_function};

use super::{Identifier, List, Token};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Expression {
    pub fid: Identifier,
    pub args: List,
    pub line: usize,
    pub pos: usize,
}

impl Expression {
    pub fn from_iterable<T>(iterable: T, line: usize, pos: usize) -> Self
    where
        T: IntoIterator<Item = Token>,
    {
        let mut iter = iterable.into_iter();

        Self {
            fid: iter.next().unwrap().extract(None).unwrap(),
            args: List::from_iterable(iter),
            line,
            pos,
        }
    }

    pub fn execute(&self, maybe_prefix: Option<String>) -> Result<Token, RuntimeError> {
        let func_id = self.fid.clone();
        let args = self.args.clone();

        match BUILTIN_FUNCTIONS.get(func_id.0.as_str()) {
            Some(func) => func(args, maybe_prefix),

            None => match get_function(&func_id.0) {
                Some(f) => f.call(func_id.0, args),
                None => Err(RuntimeError::UndefinedFunction(func_id.to_string())),
            },
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_two() {
        let expression = Expression::from_iterable(
            [
                Token::Identifier(Identifier("add".to_string())),
                Token::Number(1.0),
                Token::Number(1.0),
            ],
            0,
            0,
        );

        let expected = Token::Number(2.0);

        assert_eq!(expression.execute(None).unwrap(), expected);
    }

    #[test]
    fn greeting() {
        let set_variable = Expression::from_iterable(
            [
                Token::Identifier(Identifier("var".to_string())),
                Token::Identifier(Identifier("name".to_string())),
                Token::String("Denis".to_string()),
            ],
            0,
            0,
        );

        let concat = Expression::from_iterable(
            [
                Token::Identifier(Identifier("concat".to_string())),
                Token::String("Hello ".to_string()),
                Token::Identifier(Identifier("name".to_string())),
            ],
            0,
            0,
        );

        assert_eq!(
            set_variable.execute(None).unwrap(),
            Token::String("Denis".to_string())
        );

        assert_eq!(
            concat.execute(None).unwrap(),
            Token::String("Hello Denis".to_string())
        );
    }

    #[test]
    fn sum_of_sum() {
        let expression = Expression::from_iterable(
            [
                Token::Identifier(Identifier("add".to_string())),
                Token::Expression(Expression::from_iterable(
                    [
                        Token::Identifier(Identifier("add".to_string())),
                        Token::Number(1.0),
                    ],
                    0,
                    0,
                )),
                Token::Expression(Expression::from_iterable(
                    [
                        Token::Identifier(Identifier("add".to_string())),
                        Token::Number(1.0),
                    ],
                    0,
                    0,
                )),
            ],
            0,
            0,
        );

        assert_eq!(expression.execute(None).unwrap(), Token::Number(2.0));
    }

    // #[test]
    // fn var_in_var() {
    //     let expression = Token::Expression(
    //         vec![
    //             Token::Identifier("var".to_string()),
    //             Token::Identifier("anotherName".to_string()),
    //             Token::Expression(
    //                 vec![
    //                     Token::Identifier("var".to_string()),
    //                     Token::Identifier("name".to_string()),
    //                     Token::String("Denis".to_string()),
    //                 ]
    //                 .into(),
    //             ),
    //         ]
    //         .into(),
    //     );

    //     assert_eq!(
    //         execute(expression, &mut Scope::new()).unwrap(),
    //         Token::String("Denis".to_string())
    //     );
    // }

    // #[test]
    // fn get_two() {
    //     let mut scope = Scope::new();

    //     let create_func = Token::Expression(
    //         vec![
    //             Token::Identifier("func".to_string()),
    //             Token::Identifier("getTwo".to_string()),
    //             Token::List(vec![].into()),
    //             Token::Expression(
    //                 vec![
    //                     Token::Identifier("add".to_string()),
    //                     Token::Number(1.0),
    //                     Token::Number(1.0),
    //                 ]
    //                 .into(),
    //             ),
    //         ]
    //         .into(),
    //     );

    //     _ = execute(create_func, &mut scope);

    //     let func_call = Token::Expression(vec![Token::Identifier("getTwo".to_string())].into());

    //     assert_eq!(execute(func_call, &mut scope).unwrap(), Token::Number(2.0));
    // }
}
