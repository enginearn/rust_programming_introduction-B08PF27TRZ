use anyhow::{bail, ensure, Context, Result};

use clap::Parser;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::PathBuf;

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
    formula_file: Option<PathBuf>,

    // #[clap(name = "NUMBER")]
    // num: Option<i32>,
}

struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> Result<i32> {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> Result<i32> {
        let mut stack = Vec::new();
        let mut pos = 0;

        while let Some(token) = tokens.pop() {
            pos += 1;

            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let y = stack.pop().context(format!("invalid syntax at {}", pos))?;
                let x = stack.pop().context(format!("invalid syntax at {}", pos))?;

                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    // "^" => x ^ y,
                    _ => bail!("invalid token at {}", pos),
                };
                stack.push(res);
            }

            // `-v`オプションが指定されている場合は、この時点でのトークンとスタックの状態を出力
            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }

        ensure!(stack.len() == 1, "invalid syntax");

        Ok(stack[0])

        // if stack.len() == 1 {
        //     stack[0]
        // } else {
        //     panic!("invalid syntax...")
        // }
    }
}

fn run<R: BufRead>(reader: R, verbose: bool) -> Result<()> {
    let calc = RpnCalculator::new(verbose);

    for line in reader.lines() {
        // let line = line.unwrap();
        let line = line?;
        match calc.eval(&line) {
            Ok(answer) => println!("{}", answer),
            Err(e) => eprintln!("{}", e),
        }
        // let answer = calc.eval(&line);
        // println!("Your input: {}", line);
        // println!("The answer is: {}", answer);
    }

    Ok(())
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        run(reader, opts.verbose)
    } else {
        // ファイルを指定しなかった場合は、stdinから読み込む
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose)
    }
} // end of main()

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        // assert_eq!(2 * 2, 4);
        let calc = RpnCalculator::new(false);
        assert_eq!(calc.eval("5").unwrap(), 5);
        assert_eq!(calc.eval("50").unwrap(), 50);
        assert_eq!(calc.eval("-50").unwrap(), -50);

        assert_eq!(calc.eval("2 3 +").unwrap(), 5);
        assert_eq!(calc.eval("3 4 *").unwrap(), 12);
        assert_eq!(calc.eval("5 6 -").unwrap(), -1);
        assert_eq!(calc.eval("7 8 /").unwrap(), 0);
        assert_eq!(calc.eval("9 10 %").unwrap(), 9);
    }

    #[test]
    fn test_ng() {
        let calc = RpnCalculator::new(false);
        // calc.eval("2 3 ^");
        assert!(calc.eval("").is_err());
        assert!(calc.eval("1 1 1 +").is_err());
        assert!(calc.eval("+ 1 1").is_err());
    }
}