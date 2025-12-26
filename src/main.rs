use std::env;
use std::fs;

struct Config {
    query: String,
    file_path: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    let file_content = fs::read_to_string(config.file_path).expect("File path needs to be valid");

    println!("With text : \n{file_content}");
}

fn parse_config(args: &[String]) -> Config {
    let query = &args.get(1).expect("Please supply a query").clone();
    let file_path = &args.get(2).expect("Please supply a file path").clone();
    Config {
        query: String::from(query),
        file_path: String::from(file_path),
    }
}
