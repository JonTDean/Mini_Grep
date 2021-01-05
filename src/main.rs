
use std::{env, process};
// use ansi_term::{Color};

fn main() {
    // // Collect any arguments supplied to the shell.
    // let args: Vec<String> = env::args().collect();

    // Store the arguments in a Struct of Strings.
    // and collect any arguments supplied to the shell.
    let config = mini_grep::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);

        process::exit(1);
    });

    mini_grep::header_text(&config);

    // System Error Check.
    if let Err(err) = mini_grep::run(config) {
        eprintln!("Application Error: {}", err);

        process::exit(1);
    }

    println!("-------end-------");
}

#[cfg(test)]
#[path = "./test.rs"]
mod tests;