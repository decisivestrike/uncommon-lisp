use std::io::{self, Write};
use std::{error::Error, fs, iter::Peekable, str::Chars};

use crate::{executer::execute, parser::parse_expression};

pub fn repl() -> Result<(), Box<dyn Error>> {
    loop {
        print!("ul> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let trimmed_input = input.trim();
        if trimmed_input == "quit" {
            break;
        }

        let mut chars = input.chars().peekable();

        while let Some(_) = chars.peek() {
            match get_expression(&mut chars) {
                Some(expression) => {
                    let expression = parse_expression(&mut expression.chars().peekable())?;
                    println!("{}", execute(expression)?.value()?);
                }
                None => break,
            }
        }
    }

    Ok(())
}

pub fn tokenize_file(path: &str) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(path)?;
    let mut chars = file_content.chars().peekable();

    while let Some(expression_literal) = get_expression(&mut chars) {
        let token = parse_expression(&mut expression_literal.chars().peekable())?;
        println!("{}", token);
    }

    Ok(())
}

pub fn run_file(path: &str) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(path)?;
    let mut chars = file_content.chars().peekable();

    while let Some(expression_literal) = get_expression(&mut chars) {
        let expression = parse_expression(&mut expression_literal.chars().peekable())?;
        execute(expression)?;
    }

    Ok(())
}

fn get_expression(chars: &mut Peekable<Chars<'_>>) -> Option<String> {
    let mut expression = String::new();
    let mut depth = 1;

    while chars.peek() != Some(&'(') {
        if chars.next() == None {
            return None;
        }
    }

    expression.push(chars.next().unwrap());

    while let Some(ch) = chars.next() {
        match ch {
            '(' => {
                expression.push(ch);
                depth += 1;
            }
            ')' => {
                expression.push(ch);
                depth -= 1;
                if depth == 0 {
                    return Some(expression);
                }
            }
            _ => expression.push(ch),
        }
    }

    // TODO: add incomplete expression check

    None
}
