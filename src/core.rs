use std::io::{self, Write};
use std::{error::Error, fs};

use crate::executer::execute;
use crate::parser::Parser;

pub fn repl() -> io::Result<()> {
    loop {
        print!("ul> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.trim() == "quit" {
            break;
        }

        match Parser::new(&input).parse() {
            Ok(expressions) => {
                for e in expressions {
                    match execute(e) {
                        Ok(result) => println!("{}", result.to_string()),
                        Err(error) => println!("RuntimeError: {}", error),
                    }
                }
            }
            Err(error) => println!("ParseError: {}", error),
        }
    }

    Ok(())
}

pub fn tokenize_file(path: &str) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(path)?;

    Parser::new(&file_content)
        .parse()?
        .into_iter()
        .for_each(|token| println!("{:?}", token));

    Ok(())
}

pub fn run_file(path: &str) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(path)?;
    let expressions = Parser::new(&file_content).parse()?;

    for e in expressions {
        execute(e)?;
    }

    Ok(())
}
