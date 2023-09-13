use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for: \"{}\"", config.query);
    println!("In file {}", config.file_path);

    run(config);
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
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

fn run(config: Config) {
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file.");

    println!("With text:\n{contents}");
}