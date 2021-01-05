use std::{error::Error, fs, env};
use ansi_term::Colour::{Green, Red, Cyan};

pub fn header_text(config: &Config) {
    println!("====mini-grep====");
    println!("Querying term: {}", Green.paint(&config.query));
    println!("In File: {}", Red.paint(&config.filename));
    println!("------begin------");
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        // Skips the first index which is the Binary flag.
        args.next();

        // Query starts at [1] because [0] is the Binary Path
        // Filename is at [2] but I might switch filename
        // and query indices in order to allow multi parsing.
        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        }; 

        let filename = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a filename"),
        }; 

        // CMD: 
        // Use the command line setter command 
        // // NOTE THIS IS PERMA SETTING THE VALUE OF THE ENV VARIABLE
        // // i.e. -> 
        // // // set CASE_INSENSITIVE=1 
        // // // cargo run monday schedule.txt

        // // To unset the ENV variable set it with an empty value: 
        // // set CASE_INSENSITIVE=
        
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Takes the filename, and returns the
    // contents of the file as a Result<String>
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents) 
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    // Iterate through the contents
    // and Read the contents
    for line in results {
        println!("{}", Cyan.paint(line));
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // Primitive Design Anti-Pattern
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }

    // results

    // Iterator Adapter Pattern
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }

    results
}

