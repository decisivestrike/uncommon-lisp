use core::{repl, run_file};
use std::{env, error::Error};

mod core;
mod parser;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();

    match args.len() {
        1 => repl()?,
        2 => run_file(&args[1])?,
        _ => Err("Using: ul <filename>")?,
    }

    Ok(())
}
