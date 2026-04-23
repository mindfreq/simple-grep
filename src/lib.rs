use colored::Colorize;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: Vec<String>) -> Result<Self, &'static str> {
        args.remove(0); // Remove program path
        let ignore_case = Self::ignore_case(&mut args);

        if args.len() < 2 {
            return Err("Argument not enough!");
        }

        let (query, file_path) = (args.remove(0), args.remove(0));
        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }

    fn ignore_case(args: &mut Vec<String>) -> bool {
        for (i, arg) in args.iter().enumerate() {
            if arg.contains("-i") {
                args.remove(i);
                return true;
            }
        }
        false
    }
}

pub fn search(config: Config, file_content: String) -> Result<Vec<String>, &'static str> {
    let query = config.query;

    let mut str_result: Vec<String> = Vec::new();

    if config.ignore_case {
        for line in file_content.lines() {
            let search_lower = query.to_lowercase();
            let line_lower = line.to_lowercase();

            if line_lower.contains(&search_lower) {
                let highlighted = line_lower
                    .find(&search_lower)
                    .map(|i| {
                        line[..i].to_string()
                            + &line[i..i + query.len()].green().to_string()
                            + &line[i + query.len()..]
                    })
                    .unwrap_or(line.to_string());

                str_result.push(highlighted);
            }
        }
    } else {
        for line in file_content.lines() {
            if line.contains(&query) {
                let highlighted = line.replace(&query, &query.green().to_string());
                str_result.push(highlighted);
            }
        }
    }
    Ok(str_result)
}

#[cfg(test)]
mod tests {

    use super::*;

    fn run_search(args: Vec<String>) -> Vec<String> {
        let config = Config::build(args).unwrap();

        let file_content = std::fs::read_to_string(&config.file_path)
            .expect("Failed to read file");

        let result = search(config, file_content).unwrap();
        assert!(!result.is_empty(), "Expected matches but found none!");
        result
    }

    #[test]
    fn test_search() {
        let args: Vec<String> = vec!["target/debug/m-grep", "Who", "poem.txt"]
            .iter().map(|&x| x.into()).collect();
        
        let result = run_search(args);

        for line in result {
            println!("{line}");
            assert_eq!(line, format!("I'm nobody! {} are you?", "Who".green()));
        }
    }

    #[test]
    fn test_search_ignorecase() {
        let args: Vec<String> = vec!["target/debug/m-grep", "who", "poem.txt", "-i"]
            .iter().map(|&x| x.into()).collect();
        
        let result = run_search(args);

        for line in result {
            assert_eq!(line, format!("I'm nobody! {} are you?", "Who".green()));
        }
    }
}
