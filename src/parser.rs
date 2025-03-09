#[derive(Clone, Debug)]
pub enum Token {
    Number(i64),
    String(String),
    Identifier(String),
    List(Vec<Token>),
    Bool(bool),
}

type Expression = Vec<Token>;

pub fn parse(input: &str) -> Result<Token, String> {
    match input {
        _ if input.starts_with('(') && input.ends_with(')') => {
            let inner = &input[1..input.len() - 1];

            let mut values: Vec<Token> = Vec::new();
            let mut current = String::new();
            let mut depth = 0;
            let mut in_quoted_string = false;

            for c in inner.chars() {
                match c {
                    '(' => {
                        depth += 1;
                        current.push(c);
                    }
                    ')' => {
                        depth -= 1;
                        current.push(c);
                    }
                    '"' => {
                        in_quoted_string ^= true;
                        current.push(c);
                    }
                    ' ' if depth == 0 && !in_quoted_string && !current.is_empty() => {
                        values.push(parse(&current)?);
                        current.clear();
                    }
                    _ => current.push(c),
                }
            }

            if !current.is_empty() {
                values.push(parse(&current)?);
            }

            Ok(Token::List(values))
        }

        _ if input.starts_with('"') && input.ends_with('"') => {
            let token = Token::String(input.trim_matches('"').to_string());
            Ok(token)
        }

        _ if input.parse::<f64>().is_ok() => Ok(Token::Number(input.parse().unwrap())),

        "" => Ok(Token::String(String::new())),

        "true" => Ok(Token::Bool(true)),

        "false" => Ok(Token::Bool(false)),

        _ if matches!(input.chars().nth(0).unwrap(), 'a'..'z' | 'A'..'Z' ) => {
            Ok(Token::Identifier(input.to_string()))
        }

        _ => Err(format!("Wtf if that: {}", input)),
    }
}

pub fn eval(token: &Token) -> Result<Token, String> {
    match token {
        Token::Identifier(name) => Err(format!("Unbound variable: {}", name)),

        Token::Number(_) | Token::String(_) | Token::Bool(_) => Ok(token.clone()),

        Token::List(list) => {
            if list.is_empty() {
                return Err("Empty list".to_string());
            }

            match &list[0] {
                Token::Identifier(s) => match s.as_str() {
                    "+" => apply_op(list, |a, b| a + b),
                    "-" => apply_op(list, |a, b| a - b),
                    "*" => apply_op(list, |a, b| a * b),
                    "/" => apply_op(list, |a, b| a / b),
                    _ => Err(format!("Unknown function: {}", s)),
                },

                _ => Err("First element of list must be a function".to_string()),
            }
        }
    }
}

// For numbers only
fn apply_op<F>(args: &[Token], op: F) -> Result<Token, String>
where
    F: Fn(i64, i64) -> i64,
{
    if args.len() < 3 {
        return Err("Too few arguments for operation".to_string());
    }

    let mut result = match eval(&args[1])? {
        Token::Number(n) => n,
        _ => return Err("Argument must be a number".to_string()),
    };

    for arg in &args[2..] {
        match eval(arg)? {
            Token::Number(n) => result = op(result, n),
            _ => return Err("Argument must be a number".to_string()),
        }
    }

    Ok(Token::Number(result))
}
