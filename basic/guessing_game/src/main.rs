use std::io; // prelude
use std::cmp::Ordering;
// 引入第三方模块

// trait 可以理解为是其他语言的接口，定义了许多方法
// 课程是 2020 年的，rand 的版本是 0.3.14 使用方式和现在不同
// use Rand::Rng; // trait

// 2023-04-04 0.8.5 版本的 rand 引入方式和使用方式发生了变化
use rand::prelude::*;

fn main() {
    // println!("猜数");
    println!("猜测一个数");

    // 0.3.14 的使用方式，生成一个 1-100 之间的随机数
    // let secret_number = rand::thread_rng().gen_range(1, 101);
    // 0.8.5 的使用方式
    let secret_number = thread_rng().gen_range(1..=100);

    // println!("random {}", secret_number);
    // 声明一个变量，rust 中，所有的变量默认是不可变的
    // let foo = 1; // immutable
    //  cannot assign twice to immutable variable
    // foo = 2;
    // 如果想让一个变量可以变化，使用 mut 关键字
    // String 是字符串类
    // :: 表示这个一个关联函数。类似于 Java 中的静态方法
    // new 函数创建一个空字符串


    // loop 表示循环
    loop {
        println!("请输入一个数");

        let mut guess = String::new();
        // & 符号表示引用，也就是对同一块内存地址的引用
        // io是一个库，有一个函数是 stdin，返回 Stdin 实例，用来处理标准的输入
        // 调用 Stdin 实例中的 read_line 函数，用来读取我们的输入，并将读取的结果赋值给一个变量：guess
        // except 函数用来处理异常
        io::stdin().read_line(&mut guess).expect("无法读取行");
        println!("你猜的数字是：{}", guess);

        // 将 guess 转换为数字类型
        // shadow
        // parse() 返回一个 Result，Result 是一个枚举，有两个值：OK 和 ERR，分别表示成功和失败
        // 对于 ERR 的情况，调用 expect 函数处理失败，但是一旦调用了 expect，程序就会崩溃从而退出
        // let guess: u32 = guess.trim().parse().expect("please type a number");

        // 使用 match 进行匹配，代替 expect 来处理错误，防止输入错误导致的程序退出
        // match 的每个分支称为 arm
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // continue 表示跳出当前循环
            Err(_) => continue,
        };
        // match 表示模式匹配，类似于 switch case
        // cmp 是数字实例的比较方法，返回一个 Order 枚举
        match guess.cmp(&secret_number) {
            // Ordering 是标准库的 cmp 模块提供的一个枚举，包含 三个值：Less、Greater 和 Equal
            // 分别表示小于大于和等于
            Ordering::Less => println!("Too small"),  // arm
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                // break 表示中断本次循环
                break;
            },
        }
    }


}
