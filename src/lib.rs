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
    println!("With text:\n{contents}");
    Ok(())
}