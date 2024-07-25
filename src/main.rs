use anyhow::{Context, Result};
use std::fs::read_to_string;
use std::io::{self, BufRead, Write};

use clap::Parser;
mod error;
mod scanner;
mod token;
#[derive(Parser, Debug)]

struct Args {
    path: Option<String>,
}
fn main() -> Result<()> {
    let args = Args::parse();
    if let Some(path) = args.path {
        return run_file(&path);
    }

    run_prompt()?;

    Ok(())
}

fn run_file(path: &str) -> Result<()> {
    read_to_string(path)?;

    Ok(())
}

fn run_prompt() -> Result<()> {
    let stdin = io::stdin();

    print!("> ");
    io::stdout().flush()?;
    loop {
        let line = stdin.lock().lines().next().context("No more lines")??;
        run(&line)?;

        if line.is_empty() {
            return Ok(());
        }

        print!("{} > ", line);
        io::stdout().flush()?;
    }
}

fn run(source: &str) -> Result<()> {
    Ok(())
}
