
// 使用 rust 实现一个命令行工具

// 我们要实现的命令行工具是：grep（globally search a regular expression and print，全局正则搜索与输出）
// 而它最简单的使用场景就是在特定文件中搜索指定的字符串
// 为此，grep 会接收一个文件名和一个字符串作为参数，然后在执行时读取文件来搜索包含指定字符串的行，并最终将这些行打印输出

// 基本要求：
// 1. 接收命令行参数
// 2. 解析命令行参数
// 3. 读取文件
// 4. 在文件中搜索
// 5. 将搜索结果打印出来
// 6. 错误处理

// 1. 接收命令行参数：让minigrep接收两个命令行参数：文件名和用于搜索的字符串（keyword）
// 我们希望通过依次输入 cargorun、用于搜索的字符串及文件路径的命令行来运行程序：
// cargo run searchstring example-filename.txt

// 为了使 minigrep 可以读取传递给它的命令行参数值，我们需要使用 Rust 标准库提供的 std::env::args 函数
// 这个函数会返回一个传递给 minigrep 的命令行参数迭代器（iterator），然后将迭代器产生的值，转换为集合（collection），例如：动态数组

// 引入 env 模块
use std::env;

// 引入 fs
use std::fs;


fn main() {
    // 使用 env 下的方式收集命令行参数收集到一个动态数组中
    // 因为 collect 函数可以返回多种不同的集合，所以我们显式地标注了 args 的类型来获得一个包含字符串的动态数组
    // Rust 无法推断出我们具体的集合类型，所以需要我们手动标注
    let args: Vec<String> = env::args().collect();
    // let (query, filename) = parse_config(&args);

    // let config = parse_config(&args);
    let config = Config::new(&args);
    println!("Searching for {}", config.query);
    // // In file sample.txt
    println!("In file {}", config.filename);

    // 我们输入的命令式： cargo run needle haystack
    // 打印结果是：["target/debug/minigrep", "needle", "haystack"]
    // 动态数组中的第一个值是 "target/debug/minigrep"，也就是当前执行的二进制文件名称
    // 这和C语言处理参数列表时的行为是一致的，程序可以通过这个参数在运行时获得自己的名称
    // 这一功能可以让我们方便地在输出信息中打印程序名称，或者根据程序名称的不同而改变行为等
    // println!("{:?}", args);

    // // 存储收到的参数
    // // args 第一个元素是执行的二进制文件名，所以，我们需要的参数从第二个元素开始
    // // 需要使用引用类型
    // let query = &args[1];
    // let filename = &args[2];

    // 输入的命令是：cargo run test sample.txt
    // Searching for test
    // println!("Searching for {}", query);
    // // In file sample.txt
    // println!("In file {}", filename);

    // // fs::read_to_string 接收一个路径作为参数，如果是相对路径，相对的是项目的根目录：/minigrep/
    // // 打开对应文件并使用 Result<String> 类型返回文件的内容
    // let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    // println!("With text : \n{}", content);



}

// 使用结构体来存储关联性很强的两个字段：query 和 filename
struct Config {
    query: String,
    filename: String
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();
        Config { 
            query, 
            filename
        }
    }
}

// fn parse_config(args: &[String]) -> (&str, &str) {
//     let query = &args[1];
//     let filename = &args[2];
//     (query, filename)
// }

// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let filename = args[2].clone();
//     Config { 
//         query, 
//         filename
//     }
// }