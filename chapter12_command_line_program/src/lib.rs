use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // This is the idiomatic way to construct new objects.
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough input arguments.");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        // If env variable is set, case_sensitive should evaluate to false.
        // We therefore check if Result is Err to get this inverse behavior.
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

// If Ok, returns the unit type.
// Else, returns the trait object Box<dyn Error> which may be any type that implements the Error
// trait.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let res = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        ci_search(&config.query, &contents)
    };

    for line in res {
        println!("{}", line);
    }

    // Idiomatic way to indicate that we call run for its side effects only.
    Ok(())
}

// The search result is borrowed from the contents string slice. We therefore need to connect their
// lifetimes or the borrow checker will complain.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }
    res
}

pub fn ci_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut res = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\nRust:\nsafe, fast, productive.\nPick three.\nDuct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\nRust:\nsafe, fast, productive.\nPick three.\nTrust me.";

        assert_eq!(vec!["Rust:", "Trust me."], ci_search(query, contents));
    }
}
