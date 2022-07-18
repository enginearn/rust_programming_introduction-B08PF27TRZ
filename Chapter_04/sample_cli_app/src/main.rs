use clap::{arg, App};

/*
USAGE:
    cargo run -- input.txt
    File specified: input.txt
    Is verbosity specified?: false

    cargo run -- -V
    My RPN program 1.0.0

    ..\target\debug\sample_cli_app.exe -h
    My RPN program 1.0.0
    Your name
    Super awesome sample RPN calculator

    USAGE:
        sample_cli_app.exe [OPTIONS] [FILE]

    ARGS:
        <FILE>    Formulas written in RPN

    OPTIONS:
        -h, --help       Print help information
        -v, --verbose    Sets the level of verbosity
        -V, --version    Print version information
*/

fn main() {
    let matches = App::new("My RPN program")
        .version("1.0.0")
        .author("Your name")
        .about("Super awesome sample RPN calculator")
        .arg(arg!([FILE] "Formulas written in RPN").required(false))
        .arg(arg!(-v --verbose ... "Sets the level of verbosity").required(false))
        .get_matches();

    match matches.value_of("FILE") {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified."),
    }
    let verbose = matches.is_present("verbose");
    println!("Is verbosity specified?: {}", verbose);
}

// fn main() {
//     let matches = App::new("My RPN program")
//         .version("1.0.0")
//         .author("Your name")
//         .about("Super awesome sample RPN calculator")
//         .arg(
//             Arg::new("formula_file")
//                 .about("Formulas written in RPN")
//                 .value_name("FILE")
//                 .index(1)
//                 .required(false),
//         )
//         .arg(
//             Arg::new("verbose")
//                 .about("Sets the level of verbosity")
//                 .short("v")
//                 .long("verbose")
//                 .required(false),
//         )
//         .get_matches();

//     match matches.value_of("formula_file") {
//         Some(file) => println!("File specified: {}", file),
//         None => println!("No file specified"),
//     }
//     let verbose = matches.is_present("verbose");
//     println!("Is verbosity specified? {}", verbose);
// }
