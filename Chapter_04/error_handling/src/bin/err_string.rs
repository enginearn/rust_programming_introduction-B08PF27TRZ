fn get_int_from_file() -> Result<i32, String> {
    let path = "number.txt";

    let num_str =
        std::fs::read_to_string(path).map_err(|e| format!("failed to open the file...{}", e))?;

    num_str // 最初は`num_str`は`&str`型
        .trim() // 文字列の前後の空白を取り除く。型は`&str`型のまま。
        .parse::<i32>() // `&str`型を`i32`型に変換する。型は`Result<i32, ParseIntError>`型。
        .map(|t| t * 2) // `parse()`の結果が`Ok(t)`の場合は、`t`を2倍して返す。型は`i32`型。
        .map_err(|e| e.to_string()) // `parse()`の結果が`Err(e)`の場合は、`e`を文字列に変換して返す。型は`String`型。
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e),
    }
}
