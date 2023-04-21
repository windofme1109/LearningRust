
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

use std::error::Error;
// 引入 env 模块
use std::{env, process};

// 引入 fs
use std::fs;

// 导入 库 crate
use minigrep::{self, Config};

// Rust 社区开发了一套为将会逐渐臃肿的二进制程序进行关注点分离的指导性原则：
// 1. 将程序拆分为 main.rs 和 lib.rs ，并将实际的业务逻辑放入 lib.rs
// 2. 当命令行解析逻辑相对简单时，将它留在main.rs 中也无妨
// 3. 当命令行解析逻辑开始变得复杂时，同样需要将它从 main.rs 提取至 lib.rs 中

// main.rs 只做下面这几个事情：
// 1. 调用命令行解析的代码处理参数值
// 2. 准备所有其他的配置
// 3. 调用lib.rs 中的run函数
// 4. 处理run函数可能出现的错误

// 也就是说，maim 只负责启动程序，而真正的业务逻辑放在 lib 中
fn main() {
    // 使用 env 下的方式收集命令行参数收集到一个动态数组中
    // 因为 collect 函数可以返回多种不同的集合，所以我们显式地标注了 args 的类型来获得一个包含字符串的动态数组
    // Rust 无法推断出我们具体的集合类型，所以需要我们手动标注
    let args: Vec<String> = env::args().collect();


    // 关联函数 new 成功的时候返回 Ok 变体，Ok 中的值是 Config 实例
    // 而参数长度不足的时候，返回 Err 变体，Err 中的值是错误信息
    // 这里使用 Result 枚举中的 unwrap_or_else 方法
    // 使用 unwrap_or_else 可以让我们执行一些自定义的且不会产生 panic! 的错误处理策略
    // 当 Result 的值是 Ok 时，这个方法的行为与 unwrap 相同：它会返回 Ok 中的值
    // 但是，当值为 Err 时，这个方法则会调用闭包 （closure）中编写的代码，也就是我们定义出来并通过参数传入 unwrap_or_else 的这个匿名函数
    let config = Config::new(&args).unwrap_or_else(|err| {
        // println!("Problem parsing arguments: {}", err);

        // 区分标准错误（stderr）和标准输出（stdout）

        // 参考资料：https://blog.51cto.com/u_15127685/4395796
        // println! 是用来打印标准输出的，也就是说，正常程序运行的结果，应该使用标准输出打印到屏幕上
        // 而且，标准输出的内容还可以写到文件中
        // 我们使用标准输出 println! 打印错误信息，可能会出现一个问题，就是有时候会将标准输出的内容写入文件
        // 那么此时如果输出的是错误信息，也会写入文件中，这样就会掺杂正确的内容和错误信息

        // 例如，我们可以我们可以在运行程序时使用 > 运算符与文件名 output.txt，这个文件名指定了标准输出重定向的目标
        // 命令：cargo run > output.txt
        // 由于我们没有传入任何参数，所以程序应该会在执行时引发一个错误：
        // Problem parsing arguments: not enough arguments

        // 这里的 > 语法会告知终端将标准输出中的内容写入 output.txt 文件而不是打印到屏幕上
        // 运行程序后屏幕上没有出现我们期待的错误提示信息，这意味着错误提示信息可能被写入了文件中
        // 现在 output.txt 文件中应该会包含以下内容：
        // Problem parsing arguments: not enough arguments

        // 错误提示信息被打印到了标准输出中，将类似的错误提示信息打印到标准错误可能会更加实用，这样可以让文件内容保持整洁，只包含正常的运行结果数据

        // 标准错误将信息打印到屏幕上，所以，对于错误信息，我们应该使用标准错误进行打印
        // 正常的结果使用标准输出打印或者写入到文件中，这样可以保证文件中都是正常的结果，不会出现错误信息
        // 错误只会经由标准错误打印到屏幕上
        // 可以类比于 javascript 中的 console.log 和 console.error
        eprintln!("Problem parsing arguments: {}", err);


        // process::exit函数会立刻中止程序运行，并将我们指定的错误码返回给调用者
        process::exit(1);
    });
    
    // 将主要的运行逻辑放到 run 函数中
    // run 函数返回的是 Result，所以我们必须对 Err 变体进行处理，否则编译器会警告我们：this `Result` may be an `Err` variant, which should be handled
    // 使用 if let 来处理异常，这里不适用 unwrap_or_else 方法的原因是：run函数并不会返回一个需要进行unwrap的值
    // 因为run函数在运行成功时返回的是()，而我们只关注产生错误时的情形，所以没有必要调用 unwrap_or_else 把这个必定是()的值取出来
    // 对于错误的处理，if let 和 unwrap_or_else 是一样的
    if let Err(e) = minigrep::run(config) {
        
        // println!("Application error: {}", e);
        eprintln!("Application error: {}", e);
        process::exit(1)    
    };

    // 使用 Result 处理异常，并结合 unwrap_or_else 方法
    // 当我们再输入 cargo run 的时候，控制台打印的信息如下：
    //     Finished dev [unoptimized + debuginfo] target(s) in 0.44s
    //      Running `target\debug\minigrep.exe`
    // Problem parsing arguments: not enough arguments

    // 打印出了我们期望的信息



    // let (query, filename) = parse_config(&args);

    // let config = parse_config(&args);
    // let config = Config::new(&args);
    // println!("Searching for {}", config.query);
    // // // In file sample.txt
    // println!("In file {}", config.filename);

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



// 将 run 函数移动到 lib.rs 中，通过模块进行调用

// 启动函数，包含了程序运行的主要逻辑
// run 函数的返回值是 Rsult 枚举，当出现错误时，返回的是 Err 变体，其包裹的类型使用了 trait 对象（trait object）Box<dyn Error>（
// Box<dyn Error> 意味着函数会返回一个实现了 Error trait 的类型，但我们并不需要指定具体的类型是什么
// 这意味着我们可以在不同的错误场景下返回不同的错误类型，语句中的dyn关键字所表达的正是这种“动态”（dynamic）的含义。
// fn run(config: Config) -> Result<(), Box<dyn Error>> {
    
//     // let content = fs::read_to_string(config.filename).expect("something went wrong reading the file");

//     // 这里使用了 ? 运算符替代 expect，当 read_to_string 返回的是 Err 变体，? 运算符直接将错误返回值返回给函数的调用者
//     let content = fs::read_to_string(config.filename)?;
//     println!("With text : \n{}", content);
//     // 成功时返回的是 ()，因此这里需要使用 Ok 变体包裹空元祖
//     Ok(())
// }
// 

// 将 Config 结构体移动到 lib.rs 中，通过模块的方式进行调用

// 使用结构体来存储关联性很强的两个字段：query 和 filename
// struct Config {
//     query: String,
//     filename: String
// }
// // 包 含 query 和 filename 字 段 的 结 构 体 Config
// impl Config {
//     // 之前的 parse_config 方法，实际上就返回了一个 Config 的实例
//     // 那么其作用就是一个构造器
//     // 我们完全可以将 parse_config 的功能转移到 Config 结构体的关联函数 new 函数中
//     // 这样可以调用 new 函数来获取 Config 的实例，这是一种更符合 Rust 惯例的方式

//     fn new(args: &[String]) -> Result<Config, &'static str> {


//         // 添加错误处理

//         if args.len() < 3 {
//             // 添加错误提示
//             // 使用 panic! 可以改善提示，但是仍然会暴露一些我们不希望用户看到的信息
//             // 如 我们输入 cargo run，不输入任何参数，输入如下：
//             // warning: `minigrep` (bin "minigrep") generated 1 warning
//             //     Finished dev [unoptimized + debuginfo] target(s) in 0.49s
//             //      Running `target\debug\minigrep.exe`
//             // thread 'main' panicked at 'not enough arguments', src\main.rs:102:13
//             // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
//             // error: process didn't exit successfully: `target\debug\minigrep.exe` (exit code: 101)
//             // 我们只希望暴露和用户输入相关的信息，其他的调试信息是不希望对外暴露的
//             // 我们更倾向于使用panic! 来暴露程序的内部问题而非用法问题
//             // panic!("not enough arguments");

//             // 这里使用 Result 枚举是更好的处理错误的办法
//             // 发生异常时，返回 Err 变体，将错误信息放到 Err 变体中
//             return Err("not enough arguments");
//         }


//         // 这里不再使用 &args[1] 获取对 args 数组中第二个元素的引用
//         // 我们之前返回的是指向 args 中 String 值的字符串切片
//         // 但我们现在定义的 Config 却包含了拥有自身所有权的 String 值
//         // 这是因为 main 函数中的 args 变量是程序参数值的所有者，而 parse_config 函数只是借用了这个值
//         // 如果 Config 试图在运行过程中夺取 args 中某个值的所有权，那么就会违反 Rust 的借用规则
//         // 也就是说，Config 中的字段是持有自身所有权，而如果是借用的 args 数组中的值，后面可能发生所有权归属的问题
//         // 因此，这里使用了 clone 方法，将字符串完整的复制了一份，并赋值给 query 和 filename
//         // 这样 query 和 filename 就拥有一个新的字符串的所有权，而不是借用
//         // 虽然 clone 方法有一定的性能损耗，但是通过 clone 方法可以避免手动管理生命周期从而让代码更加简单直接
//         // 在我们实现命令行这个应用场景中，用少许的性能交换更多的简捷性是非常值得的取舍
//         let query = args[1].clone();
//         let filename = args[2].clone();
//         Ok(Config {
//             query,
//             filename
//         })
//     }
// }

// 将解析参数的功能单独封装成一个函数
// 接收的参数是动态数组
// 返回值是一个元组，元组的类型是字符串切片，第一个参数是 query，第二个是 filename
// fn parse_config(args: &[String]) -> (&str, &str) {
//     let query = &args[1];
//     let filename = &args[2];
//     (query, filename)
// }


// 改进版的 parse_config 函数
//  将 query 和 filename 放在元组中返回，虽然可以满足需求，但是我们发现返回的两个值是彼此相关的，并且都是配置值的一部分
// 单纯地将这两个值放置在元组中并不足以表达出这些意义
// 我们可以将这两个 值放在一个结构体中，并给予每个字段一个有意义的名字
// 这样可以让未来的维护者更加方便地理解不同值之间的关系及其各自的用处
// 使用元组接收数据，本质上还是通过索引访问数据，每个数据并没有语义性，仅仅依赖于元组中的数据的顺序和我们使用什么名字解构数据
// 因此更好的方式是使用结构体，将相关联的数据放到一起，并且每个数据都有名字，使用起来更加方便
// fn parse_config(args: &[String]) -> Config {
//      // 这里不再使用 &args[1] 获取对 args 数组中第二个元素的引用
//      // 我们之前返回的是指向 args 中 String 值的字符串切片
//      // 但我们现在定义的 Config 却包含了拥有自身所有权的 String 值
//      // 这是因为 main 函数中的 args 变量是程序参数值的所有者，而 parse_config 函数只是借用了这个值
//      // 如果 Config 试图在运行过程中夺取 args 中某个值的所有权，那么就会违反 Rust 的借用规则
//      // 也就是说，Config 中的字段是持有自身所有权，而如果是借用的 args 数组中的值，后面可能发生所有权归属的问题
//      // 因此，这里使用了 clone 方法，将字符串完整的复制了一份，并赋值给 query 和 filename
//      // 这样 query 和 filename 就拥有一个新的字符串的所有权，而不是借用
//     let query = args[1].clone();
//     let filename = args[2].clone();
//     // 返回一个 Config 结构体
//     Config { 
//         query, 
//         filename
//     }
// }