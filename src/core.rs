use std::{
    error::Error,
    fs,
    io::{self, Write},
    str::Chars,
};

use crate::parser::{eval, parse};

pub fn repl() -> io::Result<()> {
    let mut input = String::with_capacity(100);

    loop {
        print!("lisp> ");
        io::stdout().flush()?;

        io::stdin().read_line(&mut input)?;

        let trimmed_input = input.trim();
        if trimmed_input == "quit" {
            break;
        }

        match parse(trimmed_input) {
            Ok(ast) => match eval(&ast) {
                Ok(result) => println!("=> {:?}", result),
                Err(e) => println!("Error: {}", e),
            },

            Err(e) => println!("Parse error: {}", e),
        }

        input.clear();
    }

    Ok(())
}

pub fn run_file(path: &str) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(path)?;
    let mut chars = file_content.chars();

    loop {
        match get_expression(&mut chars) {
            Some(expression) => {
                let token = parse(&expression)?;
                eval(&token)?;
            }
            None => break,
        }
    }

    Ok(())
}

fn get_expression(chars: &mut Chars<'_>) -> Option<String> {
    let mut expression = String::new();
    let mut depth = 0;

    if Some('(') != chars.next() {
        return None;
    }

    loop {
        match chars.next() {
            Some(ch) if ch == '(' => {
                depth += 1;
                expression.push(ch);
            }
            Some(ch) if ch == ')' => {
                if depth == 0 {
                    return Some(expression);
                }
                depth -= 1;
                expression.push(ch);
            }
            Some(ch) => expression.push(ch),
            None => return None,
        }
    }
}
