// 向外界传递错误

use std::error::Error;
use std::f32::consts::E;
use std::fs::File;
use std::fs;
use std::io;
use std::io::Read;
use std::io::File;

// fn main() {
//     // ? 运算符只能被用于返回 Result 的函数中
//     // 这里面有两层含义
//     // 1. ? 操作符只能跟在返回 Result 枚举的函数后面。如 File::open 函数
//     // 使用 ? 操作符的函数必须返回 Result 枚举，如 read_username_from_file 函数
//     // 因为 ? 运算符的功能类似于 match 表达式
//     // 所以它只能被用于那些拥有 Result 返回类型的函数
//     // 在match表达式中，return Err(e) 部分产生的返回类型是 Result
//     // 所以函数的返回类型也必须是 Result，才能与此处的 return 兼容
//
//     // main 函数默认返回 ()，所以这里使用 ? 操作符，会报错：error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option`
//     // let f = File::open("hello.txt")?;
//
//     // 在那些没有返回上述类型的函数里，一旦调用的其他函数返回了 Result<T, E>
//     // 就需要使用 match 或 Result<T, E> 自身的方法来对 Result<T, E>进行恰当的处理
//     // 当然你也可以选择在合适的条件下将函数的返回类型修改为Result<T, E>
// }

// 特殊情况下，还可以改写 main 函数的返回值类型

// 这里的 Box<dyn Error> 被称作 trait 对象
// 现在我们可以简单地将 Box<dyn Error> 理解为“任何可能的错误类型”
// 在拥有这种返回类型的main函数中使用？运算符是合法的。
fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}

//定义一个读取函数
// 返回值是 Result 枚举
// Ok 变体的类型是 String，Err 变体的类型是 io::Error
// fn read_username_from_file() -> Result<String, io::Error> {
//
//     // 读取文件
//     let f = File::open("hello.txt");
//
//     // 根据 open 的返回值进行匹配
//     let mut f = match f {
//         // OK 直接返回文件的句柄，并赋值给 f
//         Ok(file) => file,
//         // Err 直接终止函数执行，并返回 一个 Err 变体，类型是 Error 结构体
//         Err(e) => {
//             return Err(e);
//         }
//     };
//
//     // 读取文件成功
//     // 定义一个字符串
//     let mut s = String::new();
//
//     // 文件读取成功，f 是文件句柄
//     // 调用 read_to_string 方法，并传入 s 的可变引用
//     // read_to_string 读取文件中的内容，并将内容追加到 s 上
//     // read_to_string 也会有成功和失败的情况，因此返回一个 Result 的变体
//     // 由于这里是函数的最后一个表达式，所以我们不再需要显式地添加 return
//     match f.read_to_string(&mut s) {
//         // Ok 直接返回 OK 变体，作为函数的返回值，其类型是拿到文件内容的字符串 s
//         Ok(_) => OK(s),
//         // Err 直接返回 Err 变体，作为函数的返回值，其类型是 Error 结构体
//         Err(e) => Err(e)
//     }
//
//
// }

// 调用这个函数的用户需要处理包含了用户名的 Ok 值，或者包含了io::Error实例的 Err 值
// 我们这里不关心用户怎么处理，只是单纯的将错误和成功的结构向外传递出去

// 传播错误的模式在Rust编程中非常常见，所以Rust专门提供了一个问号运算符（?）来简化它的语法

fn read_username_from_file() -> Result<String, io::Error> {

    // 通过将 ? 放置于Result值之后，其作用与使用 match 表达式来处理Result 时是一样的
    // 假如这个 Result 的值是Ok，那么包含在 Ok 中的值就会作为这个表达式的结果返回并继续执行程序。
    // 假如值是 Err，那么这个值就会作为整个程序的结果返回，如同使用了return一样将错误传播给调用者
    let mut f = File::open("hello.txt")?;

    let mut  s = String::new();

    // 这里的 ? 的应用规则同上
    f.read_to_string(&mut s)?;

    // 最后需要返回一个 Ok 的变体，表明最终成功的状态
    Ok(s)
}

// match表达式与 ? 运算符的一个区别：
// 被 ? 运算符所接收的错误值会隐式地被 from 函数处理，这个函数定义于标准库的 From trait 中
// 用于在错误类型之间进行转换。当 ? 运算符调用 from 函数时
// 它就开始尝试将传入的错误类型转换为当前函数的返回错误类型
// 当一个函数拥有不同的失败原因，却使用了统一的错误返回类型来同时进行表达时，这个功能会十分有用
// 只要每个错误类型都实现了转换为返回错误类型的 from 函数
// ? 运算符就会自动帮我们处理所有的转换过程

// 使用 ? 精简代码
fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut s = String::new();

    // 这里没有创建临时变量 f，而是直接将 read_to_string 链接至 File::open ("hello.txt)? 所产生的结果处来进行调用
    // 我们依然在 read_to_string 调用的尾部保留了 ?，
    // 并依然会在 File::open 和 read_to_string 都运行成功时，返回一个包含了用户名s的Ok值
    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)

    // 上面的链式调用，当 File::open 成功的时候，返回 OK 变体中的文件句柄，因此可以继续链式调用 read_to_string
    // 当 read_to_string 成功时，将读取的字符串追加到 s 上，最后返回 Ok 变体
    // File::open 和 read_to_string 有一个失败的时候，直接返回 Err 变体
}

// 更加精简版本
fn read_username_from_file_3() -> Result<String, io::Error> {
    // Rust提供了一个函数 fs::read_to_string，用于打开文件
    // 创建一个新String，并将文件中的内容读入这个 String，接着返回给调用者
    // 其返回值依旧是一个 Result 的枚举
    fs::read_to_string("hello.txt")
}