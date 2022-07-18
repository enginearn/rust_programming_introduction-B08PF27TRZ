use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);

        for line in reader.lines() {
            let line = line.unwrap();
            /*
            ここに計算ロジックが入る
            */
            println!("{}", line);
        }
    } else {
        println!("No file is specified...");
    }
} // end of main()
//     match opts.formula_file {
//         Some(file) => println!("ile specified: {}", file),
//         None => println!("No file specified!"),
//     }
//     println!("Is verbosity specified?: {}", opts.verbose);
// }