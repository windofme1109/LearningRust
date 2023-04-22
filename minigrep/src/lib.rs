use std::error::Error;
// 引入 env 模块
use std::{env, process};

use std::env::Args;

// 引入 fs
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    // let content = fs::read_to_string(config.filename).expect("something went wrong reading the file");

    // 这里使用了 ? 运算符替代 expect，当 read_to_string 返回的是 Err 变体，? 运算符直接将错误返回值返回给函数的调用者
    let content = fs::read_to_string(config.filename)?;
    // println!("With text : \n{}", content);

    let result = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };

    for line in result {
        println!("{}", line);
    }
// 成功时返回的是 ()，因此这里需要使用 Ok 变体包裹空元祖
    Ok(())
}

// 使用结构体来存储关联性很强的两个字段：query 和 filename
pub struct Config {
    query: String,
    filename: String,
    // 搜索时是否忽略大小写，为 true，表示忽略大小写，为 false，表示识别大小写
    case_sensitive: bool,
}
// 包含 query 和 filename 字段的结构体 Config
impl Config {
    // 之前的 parse_config 方法，实际上就返回了一个 Config 的实例
    // 那么其作用就是一个构造器
    // 我们完全可以将 parse_config 的功能转移到 Config 结构体的关联函数 new 函数中
    // 这样可以调用 new 函数来获取 Config 的实例，这是一种更符合 Rust 惯例的方式

    // pub fn new(args: &[String]) -> Result<Config, &'static str> {

    // 之前的 new 函数，接收的是字符串数组，这个字符串数组是 env::args 返回的迭代器，使用 collect 方法转换过来的
    // 实际上，我们可以直接使用 env::args() 返回的迭代器
    // env::args 函数的标准库文档表明，它会返回一个类型为 std::env::Args 的迭代器
    // 据此，我们将 new 函数签名中的 args 参数的类型从 &[String] 类型改为了 std::env::Args 类型
    // 由于我们获得了 args 的所有权并会在函数体中通过迭代来改变它，所以我们需要在 args 参数前指定 mut 关键字来使其可变
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {

        // 添加错误处理
        // if args.len() < 3 {
        //     return Err("not enough arguments");
        // }

        // 使用 clone 是因为 new 函数并不持有 args 参数内元素的所有权
        // 我们获得的仅仅是一个 String 序列的切片
        // 为了返回 Config 实例的所有权，我们必须要克隆 Config 的 query 字段和 filename 字段中的值
        // 只有这样，Config 才能拥有这些值的所有权


        // 学习了迭代器之后，我们现在可以在 new 函数中直接使用迭代器作为参数来获取其所有权而无须再借用切片
        // 我们还可以使用迭代器附带的功能来进行长度检查和索引
        // 这将使 Config::new 函数的责任范围更加明确，因为我们通过迭代器将读取具体值的工作分离了出去


        // let query = args[1].clone();
        // let filename = args[2].clone();


        // args 迭代器的第一个元素是二进制程序名，我们不需要，所以这里直接调用 next 方法，跳过第一个元素
        args.next();


        // 再次调用 next 来取得用于 Config 中 query 字段的值
        // 如果 next 返回一个 Some 变体，我们就会使用 match 来提取这个值
        // 如果它返回的是 None，则表明用户没有提供足够的参数，返回一个 Err 变体
        let query = match args.next() {
            Some(v) => v,
            None => return Err("Didn't get a query string")
        };

        let filename = match args.next() {
            Some(v) => v,
            None => return Err("Didn't get a file name")
        };


        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive
        })
    }
}

pub fn search<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {



    // 使用迭代器适配器优化查找过程
    // 函数式编程风格倾向于在程序中最小化可变状态的数量来使代码更加清晰
    // 消除可变状态也使我们可以在未来通过并行化来提升搜索效率，因为我们不再需要考虑并发访问 results 动态数组时的安全问题了
    content.lines()
        .filter(|x| {x.contains(query)})
        .collect()


    // let mut result = vec![];

    // for line in content.lines() {
    //
    //     if line.contains(query) {
    //         // 将找到的结果放入动态数组中
    //         result.push(line);
    //     }
    // }
    // result
}

pub fn search_case_insensitive<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {

    let query = query.to_lowercase();

    // 使用迭代器适配器优化查找过程
    content.lines()
        .filter(|line| {line.contains(query)})
        .collect()

    // let mut result = vec![];
    //
    // for line in content.lines() {
    //
    //     if line.to_lowercase().contains(&query) {
    //         result.push(line);
    //     }
    // }
    //
    // result

}
