use std::io; // preload
// 引入第三方模块
use Rand::Rng; // trait
fn main() {
    println!("猜数");
    println!("猜测一个数");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // 声明一个变量，rust 中，所有的变量默认是不可变的
    // let foo = 1; // immutable
    //  cannot assign twice to immutable variable
    // foo = 2;
    // 如果想让一个变量可以变化，使用 mut 关键字
    // String 是字符串类
    // :: 表示这个一个关联函数。类似于 Java 中的静态方法
    // new 函数创建一个空字符串
    let mut guess = String::new();

    // & 符号表示引用，也就是对同一块内存地址的引用
    // io是一个库，有一个函数是 stdin，返回 Stdin 实例，用来处理标准的输入
    // 调用 Stdin 实例中的 read_line 函数，用来读取我们的输入，并将读取的结果赋值给一个变量：guess
    io::stdin().read_line(&mut guess).expect("无法读取行");
    println!("你猜的数字是：{}", guess);
}
