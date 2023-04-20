use std::error::Error;
// 引入 env 模块
use std::{env, process};

// 引入 fs
use std::fs;


// todo - 环境变量和命令行参数的优先级
// 某些程序允许用户同时使用命令行参数和环境变量来设置同一个选项
// 在这种情况下，程序需要确定不同配置方式的启用优先级
// 作为练习，你可以同时使用命令行参数和环境变量来配置不区分大小写
// 的选项，并在两种配置方式不一致时决定命令行参数和环境变量的优先级

// 启动函数，包含了程序运行的主要逻辑
// main 函数的返回值是 Rsult 枚举，当出现错误时，返回的是 Err 变体，其包裹的类型使用了 trait 对象（trait object）Box<dyn Error>（
// Box<dyn Error> 意味着函数会返回一个实现了 Error trait 的类型，但我们并不需要指定具体的类型是什么
// 这意味着我们可以在不同的错误场景下返回不同的错误类型，语句中的dyn关键字所表达的正是这种“动态”（dynamic）的含义。
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


        // 处理环境变量的相关函数被放置在标准库的env模块中，需要引入 env 模块
        // 然后使用 env 模块中的 var 函数来检查名为 CASE_INSENSITIVE 的环境变量是否存在，其返回值是一个 Result 枚举
        // 没有设置以 key 为名的环境变量，var 函数返回一个 Err 变体，调用 is_err 函数判断是不是 Err 变体，如果是，则返回 true，否则返回 false
        // 因为我们不关心环境变量的具体值，只关心其存在与否，所以我们直接使用了is_err而不是unwrap、expect或其他曾经接触过的Result的方法
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive
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


    //可以用字符串的 lines 方法逐行遍历字符
    // lines方法会返回一个迭代器。结合 for 循环，就可以获得字符串中每一行的值
    // 另外一种的做法是自己处理，以换行符为分隔符，对字符串进行切分
    for line in content.lines() {
        // 使用字符串的 contains 方法可实现搜索
        // contains 方法用来再字符串中查找是否包含指定的子字符串，包含的话，返回 true，否则返回 false
        if line.contains(query) {
            // 将找到的结果放入动态数组中
            result.push(line);
        }
    }

    // vec![]

    result
}

pub fn search_case_insensitive<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    // 调用 to_lowercase 将字符串转换为小写
    // 注意：query 是一个拥有数据所有权的 String，而不再是一个字符串切片
    // 因为调用 to_lowercase 函数必定会创建新的字符串
    let query = query.to_lowercase();
    let mut result = vec![];

    for line in content.lines() {
        // query 现在是一个字符串，而 contains 方法接收一个字符串切片，所在这需要使用 query 的引用
        // 将 line 转换为小写的字符，然后调用 contains 方法进行判断
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result

}


// 测试驱动开发 （ test-driven development，TDD）的流程如下：
// 1. 编写一个会失败的测试，运行该测试，确保它会如期运行失败
// 2. 编写或修改刚好足够多的代码来让新测试通过
// 3. 在保证测试始终通过的前提下重构刚刚编写的代码
// 4. 返回步骤 1，进行下一轮开发

#[cfg(test)]
mod tests {
    use super::*;


    // one_result 测试函数一开始会运行失败，因为 search 函数不能如期的返回搜索结果的数组
    // 随着我们修改 search 函数，完善它的功能，one_result 就可以通过测试
//     #[test]
//     fn one_result() {
//         let query = "duct";
//         let content = "\
// Rust:
// safe, fast, productive.
// Pick three.";
//
//         assert_eq!(
//             vec!["safe, fast, productive."],
//             search(query, content)
//         )
//     }


    // 使用环境变量
    // 用户可以设置一个环境变量，设置以后就可进行不区分大小写的搜索
    // 配置的变量在整个终端会话中一直有效


    // 将 one_result 改为 case_sensitive
    // 目的是和另外一个测试用例 case_insensitive 进行对比
    // case_sensitive 测试用例表示这是一个区分大小写搜索的测试用例
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct Tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, content)
        )
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