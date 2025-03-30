use std::{collections::VecDeque, iter::Peekable, str::Chars};

use crate::{
    errors::{ParseError, RuntimeError},
    token::{Entity, Expression, Identifier, List, primitive::Primitive},
};

pub struct Parser<'a> {
    chars: Peekable<Chars<'a>>,
    line: usize,
    position: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.trim().chars().peekable(),
            line: 0,
            position: 0,
        }
    }

    pub fn parse(mut self) -> Result<Vec<Expression>, ParseError> {
        let mut expressions = Vec::new();

        loop {
            self.skip_before_expression();
            match self.parse_expression()? {
                Some(e) => expressions.push(e),
                None => break,
            }
        }

        Ok(expressions)
    }

    fn define(&mut self, &ch: &char) -> Result<Option<Box<dyn Entity>>, ParseError> {
        let result: Option<Box<dyn Entity>> = match ch {
            ' ' | '\t' | '\r' => {
                self.chars.next();
                None
            }
            '\n' => {
                self.chars.next();
                self.line += 1;
                self.position = 0;
                None
            }
            '#' => {
                self.chars.find(|&c| c == '\n');
                self.line += 1;
                self.position = 0;
                None
            }
            '(' => self.parse_expression()?,

            '[' => self.parse_list()?,

            // '{' => Some(self.parse_object()),
            '"' => Some(self.parse_string()?),
            '-' | '0'..='9' => Some(self.parse_number()),
            'a'..='z' | 'A'..='Z' | '_' => Some(self.parse_identifier()),
            _ => {
                return Err(ParseError::UnknownToken {
                    line: self.line,
                    position: self.position,
                    ch,
                });
            }
        };

        Ok(result)
    }

    fn skip_before_expression(&mut self) {
        while let Some(&ch) = self.chars.peek() {
            match ch {
                '(' => break,
                '\n' => {
                    self.chars.next();
                    self.line += 1;
                    self.position = 0;
                }
                _ => {
                    self.chars.next();
                    self.position += 1;
                }
            }
        }
    }

    fn parse_expression(&mut self) -> Result<Option<Box<Expression>>, ParseError> {
        let mut expression = Expression::new(self.line, self.position);

        self.chars.next();

        match self.chars.peek() {
            Some('a'..='z' | 'A'..='Z' | '_') => expression.fid = Some(self.parse_identifier()),
            _ => {
                return Err(ParseError::ExpectedIdentifier {
                    line: self.line,
                    position: self.position,
                });
            }
        }

        while let Some(&ch) = self.chars.peek() {
            let privitive_value = match ch {
                ')' => {
                    self.chars.next();
                    return Ok(Some(Expression {
                        items: primitives,
                        line: self.line,
                        pos: self.position,
                    }));
                }
                _ => self.define(&ch)?,
            };

            self.position += 1;

            privitive_value.map(|e| primitives.push_back(e));
        }

        if primitives.len() != 0 {
            return Err(ParseError::IncompleteExpression {
                line: self.line,
                position: self.position,
            });
        }

        Ok(None)
    }

    fn parse_number(&mut self) -> Primitive {
        let mut num_str = String::new();

        loop {
            match self.chars.peek() {
                Some('0'..='9' | '.') => {
                    self.position += 1;
                    num_str.push(self.chars.next().unwrap());
                }
                _ => return Primitive::Number(num_str.parse().unwrap()),
            }
        }
    }

    fn parse_string(&mut self) -> Result<Primitive, ParseError> {
        let mut string = String::new();
        self.chars.next();

        loop {
            self.position += 1;
            match self.chars.peek() {
                Some('"') => {
                    self.chars.next();
                    return Ok(Primitive::String(string));
                }
                None | Some('\n') => {
                    return Err(ParseError::UnterminatedString {
                        line: self.line,
                        position: self.position,
                    });
                }
                Some(_) => string.push(self.chars.next().unwrap()),
            }
        }
    }

    fn parse_identifier(&mut self) -> Box<dyn Entity> {
        let mut identifier = String::new();

        loop {
            match self.chars.peek() {
                Some('a'..='z' | 'A'..='Z' | '_' | '0'..='9') => {
                    self.position += 1;
                    identifier.push(self.chars.next().unwrap())
                }
                _ => {
                    let token: Box<dyn Entity> = match identifier.as_str() {
                        "true" => Box::new(Primitive::Bool(true)),
                        "false" => Box::new(Primitive::Bool(false)),
                        "nil" => Box::new(Primitive::Nil),
                        _ => Box::new(Identifier(identifier)),
                    };

                    return token;
                }
            }
        }
    }

    fn parse_list(&mut self) -> Result<Option<List>, ParseError> {
        self.chars.next();

        let mut list_items = VecDeque::new();

        while let Some(&ch) = self.chars.peek() {
            let token = match ch {
                ']' => {
                    self.chars.next();
                    return Ok(Some(List(list_items)));
                }
                _ => self.define(&ch)?,
            };

            self.position += 1;

            if token.is_some() {
                list_items.push_back(token.unwrap());
            }
        }

        if list.len() != 0 {
            return Err(ParseError::IncompleteList {
                line: self.line,
                position: self.position,
            });
        }

        Ok(None)
    }

    // fn parse_object(&mut self) -> Token {
    //     self.chars.next();

    //     todo!()
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number() {
        let mut parser = Parser::new("123.456");

        let result = Token::Number(123.456);

        assert_eq!(result, parser.parse_number());
    }

    #[test]
    fn string() {
        let mut parser = Parser::new("\"Hello\"");

        let result = Token::String("Hello".to_string());

        assert_eq!(result, parser.parse_string().unwrap());
    }

    #[test]
    fn bool() {
        let mut parser = Parser::new("true");

        let result = Token::Bool(true);

        assert_eq!(result, parser.parse_identifier());
    }

    #[test]
    fn sum_of_two() {
        let mut parser = Parser::new("(sum 1 1)");

        let result = Token::Expression(
            vec![
                Token::Identifier("sum".to_string()),
                Token::Number(1.0),
                Token::Number(1.0),
            ]
            .into(),
        );

        assert_eq!(Ok(Some(result)), parser.parse_expression());
    }

    #[test]
    fn sum_of_sum() {
        let mut parser = Parser::new("(sum (sum 1)(sum 1))");

        let result = Token::Expression(
            vec![
                Token::Identifier("sum".to_string()),
                Token::Expression(
                    vec![Token::Identifier("sum".to_string()), Token::Number(1.0)].into(),
                ),
                Token::Expression(
                    vec![Token::Identifier("sum".to_string()), Token::Number(1.0)].into(),
                ),
            ]
            .into(),
        );

        assert_eq!(Ok(Some(result)), parser.parse_expression());
    }
}
