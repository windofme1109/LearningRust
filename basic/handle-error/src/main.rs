// 向外界传递错误

// 处理错误的一些原则
// 
// 当某个错误可能会导致代码处于损坏状态时，推荐你在代码中使用 panic! 来处理错误
// 在这种情形下，损坏状态意味着设计中的一些假设、保证、约定或不可变性出现了被打破的情形
// 例如，当某些非法的值、自相矛盾的值或不存在的值被传入代码中，且满足下列某个条件时：
// 1. 损坏状态并不包括预期中会偶尔发生的事情
// 2. 随后的代码无法在出现损坏状态后继续正常运行
// 3. 没有合适的方法来将“处于损坏状态”这一信息编码至我们所使用的类型中

// 假如用户在使用你的代码时传入了一些毫无意义的值，最好的办法也许就是调用 panic! 来警告他们代码中出现了bug，
// 以便用户提前在开发过程中发现并解决这些问题
// 类似地，当你调用某些不可控制的外部代码，且这些代码出现了无法修复的非法状态时，也可以直接调用 panic!


// 如果假如错误是可预期的，那么就应该返回一个 Result 而不是调用panic!
// 这样的例子包括解析器接收到错误数据的场景，以及HTTP  请求返回限流状态的场景
// 在这些例子中，返回 Result 作为结果表明失败是一种可预期的状态，调用者必须决定如何处理这些失败

// 当你的代码基于某些值来执行操作时，应该首先验证值的有效性，并在其无效时触发 panic
// 这主要是出于安全性的考虑：尝试基于某些非法值去进行操作可能会暴露代码中的漏洞
// 这也是标准库会在我们尝试进行越界访问时触发panic的主要原因：
// 尝试访问不属于当前数据结构的内存是一个普遍的安全性问题。
// 函数通常都有某种约定 ：
// 它们只在输入数据满足某些特定条件时才能正常运行。在约定被违反时触发panic是合理的
// 因为破坏约定往往预示着调用产生了 bug，而这不是我们希望用户显式去处理的错误类型。
// 事实上，调用者很难用合理的方式对程序进行恢复；
// 调用代码的程序员 需要自行解决这些问题，函数的这些约定，尤其是在违反时会触发panic的那些约定，应该在API文档中被详细注明

// 总结一下，就是用户的输入或者是操作是非预期的，同时我们不知道怎么处理这些非预期的内容，就可以触发 panic!
// 如果我们可以处理这些非预期的内容，那么就应该使用 Result

// 之前的猜数游戏中，要求用户输入一个 1～100 之间的数字
// 并没有对数字进行严格的验证，只是要求数字为正
// 在这个场景中，缺少值检查的后果还没有那么严重：最终产生的 Too hight 或 Too low 输出依然是正确的
// 
// 还可以由另外一种形式的值检测：
// 将玩家的输入解析为i32（而不仅仅只是u32）来允许玩家输入负数，并接着检查数字是否处于1～100之间
// 如果不在这个区间，就触发程序 panic，并提示用户，这样可以程序后面使用的数字都是在 1 - 100 之间的

// 实际上，将值限定在某个范围是很常见的需求，不同的函数都可以用到
// 因此我们可以将上述逻辑抽象为一个结构体，
// 结构体中保存输入的数字，只有输入符合范围的数字才能生成相关的实例

pub struct  Guess {
    
    value: i32,
}

impl Guess {
    // 关联函数创建 Guess 实例
    pub fn new(val: i32) -> Guess {
        if val < 1 || val > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        //  只有值在 1 - 100 之间才会创建 Guess 实例
        Guess { value:val}
    }

    // 定义一个 getter 函数，作用是读取相应字段内的数据并返回
    // 因为 Guess 中的 value 字段是私有的，所以我们有必要提供这类公共方法用于访问数据
    // 而之所以将 value 字段设置为私有的，是因为我们不允许使用 Guess 结构体的代码随意修改 value 中的值
    pub fn value(&self) -> i32 {
        self.value
    }
}

//当我们在使用 Guess 这个结构体的时候，使用 Guess::new 函数来创建新的 Guess 实例
// 这就确保了所有Guess实例中的 value 都可以在 Guess::new 函数中进行有效性检查


use std::error::Error;
// use std::f32::consts::E;
use std::fs::File;
use std::fs;
use std::io;
use std::io::Read;
// use std::io::File;





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