use std::error::Error;
// 引入 env 模块
use std::{env, process};

// 引入 fs
use std::fs;


// 启动函数，包含了程序运行的主要逻辑
// main 函数的返回值是 Rsult 枚举，当出现错误时，返回的是 Err 变体，其包裹的类型使用了 trait 对象（trait object）Box<dyn Error>（
// Box<dyn Error> 意味着函数会返回一个实现了 Error trait 的类型，但我们并不需要指定具体的类型是什么
// 这意味着我们可以在不同的错误场景下返回不同的错误类型，语句中的dyn关键字所表达的正是这种“动态”（dynamic）的含义。
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    
    // let content = fs::read_to_string(config.filename).expect("something went wrong reading the file");

    // 这里使用了 ? 运算符替代 expect，当 read_to_string 返回的是 Err 变体，? 运算符直接将错误返回值返回给函数的调用者
    let content = fs::read_to_string(config.filename)?;
    // println!("With text : \n{}", content);
    // 成功时返回的是 ()，因此这里需要使用 Ok 变体包裹空元祖

    for line in search(&config.query, &content) {
        println!("{}", line);
    }

    Ok(())
}

// 使用结构体来存储关联性很强的两个字段：query 和 filename
pub struct Config {
    query: String,
    filename: String
}
// 包 含 query 和 filename 字 段 的 结 构 体 Config
impl Config {
    // 之前的 parse_config 方法，实际上就返回了一个 Config 的实例
    // 那么其作用就是一个构造器
    // 我们完全可以将 parse_config 的功能转移到 Config 结构体的关联函数 new 函数中
    // 这样可以调用 new 函数来获取 Config 的实例，这是一种更符合 Rust 惯例的方式

    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        // 添加错误处理
        if args.len() < 3 {
            // 添加错误提示
            // 使用 panic! 可以改善提示，但是仍然会暴露一些我们不希望用户看到的信息
            // 如 我们输入 cargo run，不输入任何参数，输入如下：
            // warning: `minigrep` (bin "minigrep") generated 1 warning
            //     Finished dev [unoptimized + debuginfo] target(s) in 0.49s
            //      Running `target\debug\minigrep.exe`
            // thread 'main' panicked at 'not enough arguments', src\main.rs:102:13
            // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
            // error: process didn't exit successfully: `target\debug\minigrep.exe` (exit code: 101)
            // 我们只希望暴露和用户输入相关的信息，其他的调试信息是不希望对外暴露的
            // 我们更倾向于使用panic! 来暴露程序的内部问题而非用法问题
            // panic!("not enough arguments");

            // 这里使用 Result 枚举是更好的处理错误的办法
            // 发生异常时，返回 Err 变体，将错误信息放到 Err 变体中
            return Err("not enough arguments");
        }


        // 这里不再使用 &args[1] 获取对 args 数组中第二个元素的引用
        // 我们之前返回的是指向 args 中 String 值的字符串切片
        // 但我们现在定义的 Config 却包含了拥有自身所有权的 String 值
        // 这是因为 main 函数中的 args 变量是程序参数值的所有者，而 parse_config 函数只是借用了这个值
        // 如果 Config 试图在运行过程中夺取 args 中某个值的所有权，那么就会违反 Rust 的借用规则
        // 也就是说，Config 中的字段是持有自身所有权，而如果是借用的 args 数组中的值，后面可能发生所有权归属的问题
        // 因此，这里使用了 clone 方法，将字符串完整的复制了一份，并赋值给 query 和 filename
        // 这样 query 和 filename 就拥有一个新的字符串的所有权，而不是借用
        // 虽然 clone 方法有一定的性能损耗，但是通过 clone 方法可以避免手动管理生命周期从而让代码更加简单直接
        // 在我们实现命令行这个应用场景中，用少许的性能交换更多的简捷性是非常值得的取舍
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config {
            query,
            filename
        })
    }
}

// search 函数的作用是从 content 字符串中搜索出包含 query 关键词的句子，并将其放到动态数组中返回

// search 函数的签名中需要一个显式生命周期 'a，它被用来和 contents 参数与返回值一起使用
// 生命周期参数指定了哪一个参数的生命周期会和返回值的生命周期产生关联
// 我们指定返回的动态数组应当包含从 contents 参数（而不是query参数）中取得的字符串切片
// 返回值的生命周期应该和 contents 的生命周期相同
// 而且由于这里是两个参数，且没有 self 参数，所以 Rust 无法推断出相关的生命周期
pub fn search<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {

    let mut result = vec![];
    // 搜索指定内容的步骤是：
    // 1. 遍历内容的每一行
    // 2. 检查当前行是否包含搜索字符串
    // 3. 如果包含，则将其添加到返回值列表中
    // 4. 如果不包含，则忽略
    // 5. 返回匹配到的结果列表

    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    // vec![]

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, content)
        )
    }
}