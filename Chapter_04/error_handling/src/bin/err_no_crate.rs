fn get_int_from_file() -> Result<i32, MyError> {
    let path = "number.txt";
    let num_str = std::fs::read_to_string(path).map_err(|e| MyError::Io(e))?;

    num_str
        .trim()
        .parse::<i32>()
        .map(|t| t * 2)
        .map_err(|e| MyError::Num(e))
}

enum MyError {
    Io(std::io::Error),
    Num(std::num::ParseIntError),
}

use std::fmt;
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Io(cause) => write!(f, "I/O error: {}", cause),
            MyError::Num(cause) => write!(f, "Parse error: {}", cause),
        }
    }
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => match e {
            MyError::Io(cause) => println!("I/O Error: {}", cause),
            MyError::Num(cause) => println!("Parse Error: {}", cause),
        },
    }
}

/*
今回は内部で発生したエラーを`MyError`へマッピングするのに、
`.map_err(|e| MyError::Io(e))`という書き方をしていますが、
`std::io::Error`が発生する箇所が多数の場合、`MyError`への
一意なマッピングをたくさん書かないといけない場合、`From`トレイトを
実装することで、`map_err`を少し簡潔に書ける。
*/

// impl From<std::io::Error> for MyError {
//     fn from(cause: std::io::Error) -> Self {
//         Self::Io(cause)
//     }
// }

// let num_str = std::fs::read_to_string(path).map_err(MyError::from)?;