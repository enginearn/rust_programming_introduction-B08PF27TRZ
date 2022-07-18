use anyhow::{Context, Result, bail, ensure};

fn get_int_from_file() -> Result<i32> {
    let path = "number.txt";

    let num_str = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read string from {}", path))?;

    if num_str.len() >= 10 {
        bail!("number is too large!!");
    }

    ensure!(num_str.starts_with("1"), "first digit is not 1.");

    num_str
        .trim()
        .parse::<i32>()
        .map(|x| x * 2)
        .context("failed to parse string...")
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{:?}", e),
        // Err(e) => println!("{}", e), // 文字列のみを出力する場合は、こちらを使用する
    }
}