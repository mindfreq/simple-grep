use colored::Colorize;
use std::process;

use s_grep::{Config, search};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if let Err(err) = run(args) {
        eprintln!("{}: {}", "Error".red().bold(), err.to_string().red());
        process::exit(1);
    }
}

fn run(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::build(args)?;
    let file_path = &config.file_path;

    if !std::path::Path::new(&file_path).exists() {
        return Err("File not exists!".into());
    }

    let file_content = std::fs::read_to_string(file_path)
        .expect("Failed to read file");

    let result = search(config, file_content)?;

    for line in result {
        println!("{line}");
    }
    Ok(())
}
