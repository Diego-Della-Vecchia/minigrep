use std::env;
use std::error::Error;
use std::fs;
use std::process;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough args supplied");
        };
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing input arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {:} in {:}", config.query, config.file_path);

    if let Err(err) = run(config) {
        eprintln!("A problemm occured while searching: {err}");
    };
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    println!("With text : \n{content}");
    Ok(())
}
