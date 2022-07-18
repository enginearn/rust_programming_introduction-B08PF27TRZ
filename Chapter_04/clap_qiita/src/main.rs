use clap::{App, Arg};

/*
Rustで手軽にCLIツールを作れるclapを軽く紹介する
https://qiita.com/Tadahiro_Yamamura/items/4ae32347fb4be07ea642

USAGE:
..\target\debug\clap_qiita.exe --help
sampleCliApp 1.0.0
Tadahiro_Yamamura
sampleCliApp is a sample cli program

USAGE:
    clap_qiita.exe [OPTIONS] --count <count> <message>

ARGS:
    <message>

OPTIONS:
    -c, --count <count>
    -h, --help             Print help information
    -n, --name <name>      [default: World]
    -V, --version          Print version information

EXAMPLE:
..\target\debug\clap_qiita.exe -n "me!"-c 1 "Who are you?"
me! Who are you?
*/

fn main() {
    // 任意のオプション(デフォルト値を指定可能)
    let optional_opt: Arg = Arg::new("name")
        .short('n')
        .long("name")
        .default_value("World");

        // 必須オプション
    let required_opt: Arg = Arg::new("count")
        .short('c')
        .long("count")
        .takes_value(true)
        .required(true);

    // 位置引数
    let positional_arg: Arg = Arg::new("message")
        .required(true);

    // 上記で定義したオプションを使用して、コマンドAppを作成する
    let app: App = App::new("sampleCliApp")
        .version("1.0.0")
        .author("Tadahiro_Yamamura")
        .about("sampleCliApp is a sample cli program")
        .arg(optional_opt)
        .arg(required_opt)
        .arg(positional_arg);

    match app.try_get_matches() {
        Ok(m) => {
            // オプションの値を取得する
            let name = m.value_of("name").unwrap();
            let count = m.value_of("count").unwrap().parse::<i32>().unwrap();
            let message = m.value_of("message").unwrap();
            for _ in 0..count {
                println!("{} {}", name, message);
            }
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}