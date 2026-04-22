use std::process;

use colored::Colorize;

use m_grep::{Config, search};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if let Err(err) = run(args) {
        eprintln!("{}: {}", "Error".red().bold(), err.to_string().red());
        process::exit(1);
    }
}

fn run(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::build(args)?;
    let result = search(config)?;

    for line in result {
        println!("{line}");
    }
    Ok(())
}
