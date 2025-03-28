use std::{collections::VecDeque, iter::Peekable, str::Chars};

use crate::{errors::ParseError, token::Token};

pub struct Parser<'a> {
    chars: Peekable<Chars<'a>>,
    line: u64,
    position: u64,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.trim().chars().peekable(),
            line: 0,
            position: 0,
        }
    }

    pub fn parse(mut self) -> Result<Vec<Token>, ParseError> {
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

    fn define(&mut self, &ch: &char) -> Result<Option<Token>, ParseError> {
        match ch {
            ' ' | '\t' | '\r' => {
                self.chars.next();
                Ok(None)
            }
            '\n' => {
                self.chars.next();
                self.line += 1;
                self.position = 0;
                Ok(None)
            }
            '#' => {
                self.chars.find(|&c| c == '\n');
                self.line += 1;
                self.position = 0;
                Ok(None)
            }

            '(' => self.parse_expression(),
            '[' => self.parse_list(),
            '{' => Ok(Some(self.parse_object())),
            '"' => Ok(Some(self.parse_string()?)),
            '-' | '0'..='9' => Ok(Some(self.parse_number())),
            'a'..='z' | 'A'..='Z' | '_' => Ok(Some(self.parse_identifier())),
            _ => {
                return Err(ParseError::UnknownToken {
                    line: self.line,
                    position: self.position,
                    ch,
                });
            }
        }
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

    fn parse_expression(&mut self) -> Result<Option<Token>, ParseError> {
        let mut tokens = VecDeque::new();

        self.chars.next();

        while let Some(&ch) = self.chars.peek() {
            let token = match ch {
                ')' => {
                    self.chars.next();
                    return Ok(Some(Token::Expression(tokens)));
                }
                _ => self.define(&ch)?,
            };

            self.position += 1;

            if token.is_some() {
                tokens.push_back(token.unwrap());
            }
        }

        if tokens.len() != 0 {
            return Err(ParseError::IncompleteExpression {
                line: self.line,
                position: self.position,
            });
        }

        Ok(None)
    }

    fn parse_number(&mut self) -> Token {
        let mut num_str = String::new();

        loop {
            match self.chars.peek() {
                Some('0'..='9' | '.') => {
                    self.position += 1;
                    num_str.push(self.chars.next().unwrap());
                }
                _ => return Token::Number(num_str.parse().unwrap()),
            }
        }
    }

    fn parse_string(&mut self) -> Result<Token, ParseError> {
        let mut string = String::new();
        self.chars.next();

        loop {
            self.position += 1;
            match self.chars.peek() {
                Some('"') => {
                    self.chars.next();
                    return Ok(Token::String(string));
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

    fn parse_identifier(&mut self) -> Token {
        let mut identifier = String::new();

        loop {
            match self.chars.peek() {
                Some('a'..='z' | 'A'..='Z' | '_' | '0'..='9') => {
                    self.position += 1;
                    identifier.push(self.chars.next().unwrap())
                }
                _ => {
                    let token = match identifier.as_str() {
                        "true" => Token::Bool(true),
                        "false" => Token::Bool(false),
                        "nil" => Token::Nil,
                        _ => Token::Identifier(identifier),
                    };

                    return token;
                }
            }
        }
    }

    fn parse_list(&mut self) -> Result<Option<Token>, ParseError> {
        self.chars.next();

        let mut list = VecDeque::new();

        while let Some(&ch) = self.chars.peek() {
            let token = match ch {
                ']' => {
                    self.chars.next();
                    return Ok(Some(Token::List(list)));
                }
                _ => self.define(&ch)?,
            };

            self.position += 1;

            if token.is_some() {
                list.push_back(token.unwrap());
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

    fn parse_object(&mut self) -> Token {
        self.chars.next();

        todo!()
    }
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
