use minigrep::Config;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    // 对 build 返回的 `Result` 进行处理
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        std::process::exit(1)
    });
    // print 没有意义，使用测试替代
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        std::process::exit(1);
    }
}