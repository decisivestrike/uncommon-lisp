use std::{iter::Peekable, str::Chars};

use crate::{
    entities::{Entity, Expression, Identifier, List, Primitive, traits::ToEntity},
    errors::ParseError,
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

    fn define(&mut self, &ch: &char) -> Result<Option<Entity>, ParseError> {
        let result: Option<Entity> = match ch {
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
            '(' => self.parse_expression()?.map(|e| e.to_entity()),

            '[' => Some(self.parse_list()?.to_entity()),

            // '{' => Some(self.parse_object()),
            '"' => Some(self.parse_string()?.to_entity()),

            '-' | '0'..='9' => Some(self.parse_number().to_entity()),

            'a'..='z' | 'A'..='Z' | '_' => Some(self.parse_identifier_or_keyword()),

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

    fn parse_expression(&mut self) -> Result<Option<Expression>, ParseError> {
        self.chars.next();

        let fid: Identifier = match self.chars.peek() {
            Some(ch) => match self.define(ch)? {
                Some(id) => id,
                None => {
                    return Err(ParseError::ExpectedIdentifier {
                        line: self.line,
                        position: self.position,
                    });
                }
            },

            None => {
                return Err(ParseError::IncompleteExpression {
                    line: self.line,
                    position: self.position,
                });
            }
        };

        let mut args = List::new();

        while let Some(&ch) = self.chars.peek() {
            let maybe_entity = match ch {
                ')' => {
                    self.chars.next();
                    return Ok(Some(Expression {
                        fid,
                        args,
                        line: self.line,
                        pos: self.position,
                    }));
                }
                _ => self.define(&ch)?,
            };

            // self.position += 1;

            maybe_entity.map(|e| args.push_back(e));
        }

        if args.len() != 0 {
            return Err(ParseError::IncompleteExpression {
                line: self.line,
                position: self.position,
            });
        }

        Ok(None)
    }

    fn parse_number(&mut self) -> f64 {
        let mut numeric_string = String::new();

        // TODO: Update condition

        loop {
            match self.chars.peek() {
                Some('0'..='9' | '.') => {
                    self.position += 1;
                    numeric_string.push(self.chars.next().unwrap());
                }
                _ => return numeric_string.parse().unwrap(),
            }
        }
    }

    fn parse_string(&mut self) -> Result<String, ParseError> {
        let mut string = String::new();
        self.chars.next();

        loop {
            self.position += 1;
            match self.chars.peek() {
                Some('"') => {
                    self.chars.next();
                    return Ok(string);
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

    fn parse_identifier_or_keyword(&mut self) -> Entity {
        let mut identifier = String::new();

        loop {
            match self.chars.peek() {
                Some('a'..='z' | 'A'..='Z' | '_' | '0'..='9') => {
                    self.position += 1;
                    identifier.push(self.chars.next().unwrap())
                }
                _ => {
                    return match identifier.as_str() {
                        "true" => true.to_entity(),
                        "false" => false.to_entity(),
                        "nil" => Primitive::Nil.to_entity(),
                        _ => Identifier(identifier).to_entity(),
                    };
                }
            }
        }
    }

    fn parse_list(&mut self) -> Result<List, ParseError> {
        self.chars.next();

        let mut list = List::new();

        while let Some(&ch) = self.chars.peek() {
            let maybe_entity = match ch {
                ']' => {
                    self.chars.next();
                    return Ok(list);
                }
                _ => self.define(&ch)?,
            };

            self.position += 1;

            maybe_entity.map(|e| list.push_back(e));
        }

        Err(ParseError::IncompleteList {
            line: self.line,
            position: self.position,
        })
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

        let result = 123.456;

        assert_eq!(result, parser.parse_number());
    }

    #[test]
    fn string() {
        let mut parser = Parser::new("\"Hello\"");

        let result = "Hello".to_string();

        assert_eq!(result, parser.parse_string().unwrap());
    }

    #[test]
    fn bool() {
        let mut parser = Parser::new("true");

        let result = Primitive::Bool(true).to_entity();

        assert_eq!(result, parser.parse_identifier_or_keyword());
    }

    #[test]
    fn sum_of_two() {
        let mut parser = Parser::new("(sum 1 1)");

        let result = Expression {
            fid: Identifier("sum".to_string()),
            args: List::from([Primitive::Number(1.0), Primitive::Number(1.0)]),
            ..Default::default()
        };

        assert_eq!(Ok(Some(result)), parser.parse_expression());
    }

    #[test]
    fn sum_of_sum() {
        let mut parser = Parser::new("(sum (sum 1)(sum 1))");

        let result = Expression {
            fid: Identifier::new("sum"),
            args: List::from([
                Expression {
                    fid: Identifier::new("sum"),
                    args: List::from([Primitive::Number(1.0)]),
                    ..Default::default()
                },
                Expression {
                    fid: Identifier::new("sum"),
                    args: List::from([Primitive::Number(1.0)]),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        };

        assert_eq!(Ok(Some(result)), parser.parse_expression());
    }
}
