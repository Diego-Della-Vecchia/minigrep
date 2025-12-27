use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args = env::args();

    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing input arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {:} in {:}", config.query, config.file_path);

    if let Err(err) = minigrep::run(config) {
        eprintln!("A problemm occured while searching: {err}");
        process::exit(1);
    };
}
