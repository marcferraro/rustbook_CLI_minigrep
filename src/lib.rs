use std::{fs, error::Error};
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // In future refactor, consider consuming args here, unlikely to resuse
        // or find a way to not use clone()
        if args.len() < 3 {
            return Err("not enough arguments")
        }
        
        Ok(
            Config {
                query: args[1].clone(),
                file_path: args[2].clone(),
            }
        )
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    // println!("With text:\n{contents}");
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
            Rust:
            Safe, fast, productive.
            Pick three.";
        
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}