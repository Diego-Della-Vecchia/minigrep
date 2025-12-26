use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args.get(1).expect("Please supply a query");
    let file_path = &args.get(2).expect("Please supply a file path");

    let file_content = fs::read_to_string(file_path).expect("File path needs to be valid");

    println!("With text : \n{file_content}");
}
