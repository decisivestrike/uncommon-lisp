use std::io::{self, Write};
use std::{error::Error, fs, iter::Peekable, str::Chars};

use crate::{executer::execute, parser::parse_expression};

pub fn repl() -> Result<(), Box<dyn Error>> {
    loop {
        print!("lisp> ");
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
                    println!("{}", execute(expression)?.value());
                }
                None => break,
            }
        }
    }

    Ok(())
}

pub fn run_file(path: &str) -> Result<(), Box<dyn Error>> {
    // let file_content = fs::read_to_string(path)?;
    // let mut chars = file_content.chars().peekable();

    // loop {
    //     match get_expression(&mut chars) {
    //         Some(expression) => {
    //             let expression = parse_expression(&mut expression.chars().peekable())?;
    //             println!(execute(expression));
    //         }
    //         None => break,
    //     }
    // }

    // Ok(())

    todo!()
}

pub fn tokenize_file(path: &str) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(path)?;
    let mut chars = file_content.chars().peekable();

    loop {
        match get_expression(&mut chars) {
            Some(expression) => {
                let token = parse_expression(&mut expression.chars().peekable())?;
                println!("{}", token);
            }
            None => break,
        }
    }

    Ok(())
}

fn get_expression(chars: &mut Peekable<Chars<'_>>) -> Option<String> {
    let mut expression = String::new();
    let mut depth = 0;

    if Some(&'(') != chars.peek() {
        return None;
    }

    expression.push(chars.next().unwrap());

    loop {
        match chars.next() {
            Some(ch) if ch == '(' => {
                expression.push(ch);
                depth += 1;
            }
            Some(ch) if ch == ')' => {
                expression.push(ch);
                if depth == 0 {
                    return Some(expression);
                }
                depth -= 1;
            }
            Some(ch) => expression.push(ch),
            None => return None,
        }
    }
}
