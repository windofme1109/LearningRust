use std::fs::File;
use std::io::ErrorKind;

fn main() {

    // 在Rust中，我们将错误分为两大类：可恢复错误与不可恢复错误
    // 1. 可恢复错误
    //    比如文件未找到等，一般需要将它们报告给用户并再次尝试进行操作，客回复错误需要我们自行处理
    // 2. 不可恢复错误
    //     不可恢复错误往往就是bug的另一种说法，比如尝试访问超出数组结尾的位置等


    // 其他大部分的编程语言都没有刻意地区分这两种错误，而是通过异常之类的机制来统一处理它们。
    // 虽然Rust没有类似的异常机制，但它提供了用于可恢复错误的类型 Result<T, E>
    // 以及在程序出现不可恢复错误时中止运行的panic! 宏

    // Rust 中，没有 try...catch 机制

    // 程序会在 panic! 宏执行时打印出一段错误提示信息
    // 展开并清理当前的调用栈然后退出程序
    // 这种情况大部分都发生在某个错误被检测到，但程序员却不知该如何处理的时候

    // panic 中的栈展开与终止
    // 当 panic 发生时，程序会默认开始栈展开。这意味着Rust会沿着调用栈的反向顺序遍历所有调用函数
    // 并依次清理这些函数中的数据。但是为了支持这种遍历和清理操作，我们需要在二进制中存储许多额外信息
    // 除了展开，我们还可以选择立即终止程序，它会直接结束程序且不进行任何清理工作
    // 程序所使用过的内存只能由操作系统来进行回收
    // 假如项目需要使最终二进制包尽可能小，
    // 那么你可以通过在Cargo.toml 文件中的[profile]区域添加 panic ='abort' 来将 panic 的默认行为从展开切换为终止。例如，如果你想
    // 要在发布模式中使用终止模式，那么可以在配置文件中加入
    // [profile.release]
    // panic='abort'

    // 手动调用 panic!
    // panic!("crash and burn");

    // 产生回溯信息


    let v = vec![1, 2, 3];
    // 越界访问数组，程序会发生 panic
    // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src\main-handle-error:36:5
    // 会打印出出错的位置，但是并没有回溯信息
    // 如果想要展示回溯信息，那么需要使用环境变量：RUST_BACKTRACE=1
    // 如果使用的是 windows 的 powershell，命令是：$env:RUST_BACKTRACE=1;cargo run
    // $env:RUST_BACKTRACE=1 用来设置环境变量，分号用来连接两个命令
    // Linux 或者 MacOs 下，命令是：RUST_BACKTRACE=1 && cargo run

    // v[99];

    // 在Rust中使用回溯的方式与在其他语言中的使用方式类似：从头开始查看回溯列表，直至定位到自己所编写代码的文件
    // 而这也正是产生问题的地方。从定位到文件的那一行往上是我们代码所调用的代码，往下则是调用了我们代码的代码
    // 这些调用中可能会包含Rust核心库、标准库，以及你所使用的第三方库
    // 让我们来将环境变量RUST_BACKTRACE设置为一个非0值，从而获得回溯信息
    // $env:RUST_BACKTRACE=1;cargo run

    // 注意：
    // 为了获取这些带有调试信息的回溯，你必须启用调试符号（debug symbol）
    // 在运行 cargo build 或 cargo run 命令时，如果没有附带 --release 标志，那么调试符号就是默认开启的


    // 一般情况下，我们都程序产生的错误，我们都可以手动修复，并不需要终止整个程序
    // Rust 提供了一个 枚举：Result，里面有两个变体：OK 和 Err
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    // 这里的T和E是泛型参数：
    // 1. T 代表了 Ok 变体中包含的值类型，该变体中的值会在执行成功时返回
    // 2. E则代表了 Err 变体中包含的错误类型，该变体中的值会在执行失败时返回

    // 引入一个可以返回 Result 变体的函数：Result<File, std::io::Error>
    // let f = File::open("hello.txt");

    // File::open 函数打开一个文件，可能成功也可能失败
    // 当 File::open 函数运行成功时，变量 f 中的值将会是一个包含了文件句柄的Ok实例
    // 当它运行失败时，变量f中的值则会是一个包含了用于描述错误种类信息的 Err 实例
    // 所以这里可以使用模式匹配来处理两种情况
    // let f = match f {
    //     Ok(file) => file,
    //     Err(err) => panic!("There was a problem opening the file: {:?}", err)
    // };

    // 与 Option 枚举一样，Result枚举及其变体已经通过预导入模块被自动地引入当前作用域中
    // 所以我们不需要在使用 Ok 变体与 Err 变体之前在 match 分支中显式地声明 Result::
    // 由于没有 hello.txt，显然 open 函数返回的是 Err 变体
    // 所以程序执行过程中会发生 panic
    // thread 'main' panicked at 'There was a problem opening the file: Os { code: 2, kind: NotFound, message: "系统找不到指定的文件。" }', src\main-handle-error:79:21

    // 根据不同的错误类型做不同的处理

    // 不管 File::open 是因为何种原因而运行失败的，示例上面的代码都会触发 panic!
    // 但我们想要的其实是根据不同的失败原因做出不同的反应：
    // 当File::open因为文件不存在而运行失败时，我们可以创建这个文件并返回这个文件的句柄
    // 而当 File::open 因为诸如没有访问权限之类的原因而运行失败时，我们才会直接触发 panic!

    // let f = match f {
    //     Ok(file) => file,
    //     Err(err) => match err.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(file) => file,
    //             Err(e) => panic!("Tried to create file but there was problem: {:?}", e)
    //         },
    //        other_error => panic!("There was a problem opening the file:{:?}", other_error)
    //     }
    // };

    // File::open 返回的 Err变体中的错误值类型，是定义在某个标准库中的结构体类型：io::Error
    // 这个结构体拥有一个被称作 kind 的方法
    // 我们可以通过调用它来获得io::ErrorKind 值
    // 这个 io::ErrorKind 枚举是由标准库提供的，它的变体被用于描述 io 操作所可能导致的不同错误。
    // 这里使用的变体是 ErrorKind::NotFound，它用于说明我们尝试打开的文件不存在
    // 我们不但对变量 f 使用了 match 表达式，还在内部对 error.kind() 使用了 match 表达式
    // 在这个匹配分支中，我们需要检查 error.kind() 返回的值是不是 ErrorKind 枚举的 NotFound 变体
    // 如果是的话，我们就接着使用函数 File::create 来创建这个文件
    // 由于 File::create 本身也有可能会运行失败，所以我们也需要对它的返回值添加一个 match 表达式
    // 它的返回值依旧是一个 Result 变体，如果文件创建失败
    // 那么就可以打印出一条不同的错误提示信息
    // 外部 match 的最后一个分支保持不变，用于在出现其余错误时让程序触发 panic

    // 使用 unwrap 和 expect 简化 match 匹配操作

    // 1. unwrap
    // 虽然使用 match 运行得很不错，但使用它所编写出来的代码可能会显得有些冗长且无法较好地表明其意图
    // 类型Result<T, E>本身也定义了许多辅助方法来应对各式各样的任务
    // 其中一个被称为 unwrap 的方法实现了我们在示例第一个 match 表达式的效果
    // 当 Result 的返回值是 Ok 变体时，unwrap 就会返回 Ok 内部的值
    // 当 Result 的返回值是 Err 变体时，unwrap 则会替我们调用 panic! 宏

    // thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "系统找不到指定的文件。" }', src\main-handle-error:128:37
    // let f = File::open("hello.txt").unwrap();


    // 2. expect
    // expect的方法允许我们在 unwrap 的基础上指定panic! 所附带的错误提示信息
    // 使用expect并附带上一段清晰的错误提示信息可以阐明你的意图，并使你更容易追踪到panic的起源

    // thread 'main' panicked at '文件打开失败: Os { code: 2, kind: NotFound, message: "系统找不到指定的文件。" }', src\main-handle-error:137:37
    let f = File::open("hello.txt").expect("文件打开失败");

    // expect 所实现的功能与 unwrap 完全一样：要么返回指定文件句柄，要么触发panic! 宏调用
    // 唯一的区别在于，expect 触发 panic! 时会将传入的参数字符串作为错误提示信息输出，
    // 而 unwrap 触发的 panic! 则只会携带一段简短的默认信息，如上调用 unwrap 所示

    // 使用 expect 更好，字定义错误信息，同时方便定位出现问题的地方
}
