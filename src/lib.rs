use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough args supplied");
        };
        let query = args[1].clone();
        let file_path = args[2].clone();

        let case_sensitive = std::env::var("CASE_SENSITIVE").is_ok();

        println!("Case sensitive: {}", case_sensitive);

        Ok(Config {
            query,
            file_path,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    let results = if config.case_sensitive {
        search_case_sensitive(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };

    if results.is_empty() {
        println!("Results:")
    } else {
        println!("Results:");
    }
    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query_lower = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query_lower) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_sensitive(query, content)
        )
    }

    #[test]
    fn multiple_results() {
        let query = "is";
        let content = "\
This is the first line.
This is the second line.
And this is the third line.";

        assert_eq!(
            vec![
                "This is the first line.",
                "This is the second line.",
                "And this is the third line."
            ],
            search_case_sensitive(query, content)
        )
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, content)
        )
    }

    #[test]
    fn case_sensitive() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Trust me.";

        assert_eq!(vec![] as Vec<&str>, search_case_sensitive(query, content))
    }
}
