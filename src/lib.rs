// 为了满足 Result<T,E> 的要求，使用了 Ok(()) 返回一个单元类型 ()。
// Box<dyn Error> 是一个特质对象，它表示函数返回一个类型，该类型实现了 Error 特质，这样我们就无需指定具体的错误类型
pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    // 错误转换的关键就是 ?
    let contents = std::fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        // search(&config.query, &contents)
        search_new(&config.query, &contents)
    };

    // println!("With text:\n{contents}");
    for line in results {
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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // 引入了一个新的方法 to_lowercase，它会将 str 转换成全小写的字符串
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

pub fn search_new<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

// 聚合配置变量为一个结构体，避免配置分散不好维护
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
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

        // is_ok 方法是 Result 提供的，用于检查是否有值，有就返回 true，没有则返回 false
        // let ignore_case = if std::env::var("IGNORE_CASE").is_ok() {
        // 但这么写太粗糙，我决定改成要求值必须为 1 的时候才算开启
        let ignore_case = match std::env::var("IGNORE_CASE") {
            // 优先从环境变量读取 ignore_case
            Ok(val) => val == "1", // 存在环境变量，只看是否为 "1"
            Err(_) => {
                // 否则检查命令行第三个参数
                if args.len() > 3 {
                    args[3].clone() == "true"
                } else {
                    false
                }
            }
        };
        
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }

    // 使用迭代器优化的新方法
    // 数组索引会越界，为了安全性和简洁性，使用 Iterator 自带的 next 方法是一个更好的选择
    pub fn build_new(
        // 特质约束，说明 arg 可以是任何实现了 String 迭代器的类型。
        // 迭代器的所有权已经转移到 build 内，因此可以直接对其进行修改，这里加上了 mut 关键字。
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        // 第一个参数是程序名，由于无需使用，因此这里直接空调用一次
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = std::env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
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
    // fn one_result() {
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
