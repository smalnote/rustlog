use std::{
    env,
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
    process,
};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Failed to parse arguments: {}", err);
        process::exit(1);
    });

    run(config).unwrap_or_else(|err| {
        eprintln!("Application error: {}", err);
        process::exit(1);
    });
}

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = args.next().ok_or("Didn't get a query string")?;

        let file_path = args.next().ok_or("Didn't get a file path")?;

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = File::open(&config.file_path)?;
    let reader = BufReader::new(file);

    let found_lines = if config.ignore_case {
        search_case_insensitive(&config.query, reader)
    } else {
        search(&config.query, reader)
    };

    for line in found_lines? {
        println!("{}", line)
    }

    Ok(())
}

fn search(query: &str, reader: impl BufRead) -> io::Result<Vec<String>> {
    let mut found = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.contains(query) {
            found.push(line);
        }
    }

    Ok(found)
}

fn search_case_insensitive(query: &str, reader: impl BufRead) -> io::Result<Vec<String>> {
    let query = query.to_lowercase();
    let mut found = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.to_lowercase().contains(&query) {
            found.push(line);
        }
    }

    Ok(found)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents.as_bytes()).unwrap()
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents.as_bytes()).unwrap()
        );
    }
}
