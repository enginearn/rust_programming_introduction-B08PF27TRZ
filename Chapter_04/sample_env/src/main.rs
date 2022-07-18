use std::env;

/*
Usage:
    cargo run -- 1 a xyz 2.0
*/

fn main() {
    cli_args();
}

fn cli_args() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}