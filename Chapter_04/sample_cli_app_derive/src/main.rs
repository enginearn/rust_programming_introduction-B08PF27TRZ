use clap::Parser;
// use std::fs::File;
// use std::io::{BufRead, BufReader};

#[derive(Parser, Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "John Doe",
    about = "Super awesome simple RPN calculator"
)]
struct Opts {
    // Sets the level of verbosity
    #[clap(short, long)]
    verbose: bool,

    // Formulas written in RPN
    #[clap(name = "FILE")]
    formula_file: Option<String>,

    #[clap(name = "NUMBER")]
    num: Option<i32>,
}

fn main() {
    let opts = Opts::parse();
    match opts.formula_file {
        Some(file) => println!("ile specified: {}", file),
        None => println!("No file specified!"),
    }
    println!("Is verbosity specified?: {}", opts.verbose);
}