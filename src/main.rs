#[allow(dead_code)]
#[allow(unused_variables)]
use core::{repl, run_file, tokenize_file};
use std::{env, error::Error};

mod builtins;
mod core;
mod entities;
mod errors;
mod parser;
mod scope;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();

    match args.len() {
        1 => repl()?,
        2 => run_file(&args[1])?,
        3 if &args[1] == "tokenize" => tokenize_file(&args[2])?,
        _ => Err("Using: ul <filename>")?,
    }

    Ok(())
}
