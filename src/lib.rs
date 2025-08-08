// 为了满足 Result<T,E> 的要求，使用了 Ok(()) 返回一个单元类型 ()。
// Box<dyn Error> 是一个特质对象，它表示函数返回一个类型，该类型实现了 Error 特质，这样我们就无需指定具体的错误类型
pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    // 错误转换的关键就是 ?
    let contents = std::fs::read_to_string(config.file_path)?;

    // println!("With text:\n{contents}");
    for line in search(&config.query, &contents) {
        println!("{line}");
    }
    
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    // 遍历迭代每一行
    for line in contents.lines() {
        // 在每一行中查询目标字符串
        if line.contains(query) {
            // 存储匹配到的结果
            results.push(line);
        }
    }

    results
}

// 聚合配置变量为一个结构体，避免配置分散不好维护
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // new 一般不会失败，用在这里不合适，因此改叫 build
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        // 结构体拥有内部字符串的所有权
        // clone 整个程序只执行一次，性能消耗可忽略不计
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// 一般对于初始化，较少使用函数
// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let file_path = args[2].clone();

//     Config { query, file_path }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
