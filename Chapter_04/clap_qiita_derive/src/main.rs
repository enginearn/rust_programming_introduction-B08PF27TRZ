use clap::Parser;

/*
Rustで手軽にCLIツールを作れるclapを軽く紹介する
https://qiita.com/Tadahiro_Yamamura/items/4ae32347fb4be07ea642

USAGE:
    cargo run -- -h
    clap_qiita_derive 1.0.0
    me
    sample_cli_app_derive is a sample cli program with Parser

    USAGE:
        clap_qiita_derive.exe [OPTIONS] <MESSAGE>

    ARGS:
        <MESSAGE>

    OPTIONS:
        -c, --count <COUNT>    [default: 1]
        -h, --help             Print help information
        -n, --name <NAME>
        -V, --version          Print version information

    cargo run -- -c 2 "Hi there!"
    Args: Hi there!
    Args: Hi there!
*/

#[derive(Parser)]
#[clap(
    name = "clap_qiita_derive",
    version = "1.0.0",
    author = "Tadahiro_Yamamura",
    about = "clap_qiita_derive is a sample cli program with Parser"
)]

struct AppArg {
    // 任意のオプション
    #[clap(short, long)]
    name: Option<String>,
    // #[clap(short, long, default_value = "World")]
    // name: String,

    // 必須オプション
    #[clap(short = 'c', long = "count", value_parser, default_value_t = 1)]
    count: u8,

    // 位置引数
    #[clap(value_parser)]
    message: String,
}

fn main() {
    let arg: AppArg = AppArg::parse();
    for _ in 0..arg.count {
        println!(
            "{}: {}",
            arg.name.clone().unwrap_or(String::from("Args")),
            arg.message
        );
    }
}