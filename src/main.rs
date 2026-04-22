use colored::Colorize;
use std::io::Read;
use std::process;

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
    let file_path = &config.file_path;
    // if !std::path::Path::new(&file_path).exists() {
    //     return Err("File not exists!");
    // }

    let mut file = std::fs::File::open(file_path).map_err(|_| "Failed to open file")?;
    let mut file_content = String::new();

    file.read_to_string(&mut file_content)
        .map_err(|_| "Failed to read file")?;

    let result = search(config, file_content)?;

    for line in result {
        println!("{line}");
    }
    Ok(())
}
