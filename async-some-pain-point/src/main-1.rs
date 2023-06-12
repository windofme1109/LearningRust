// Rust 异步编程
// 一些疑难问题的解决方法

// 1. 在 async 语句块中使用 ?
// async 语句块和 async fn 最大的区别就是前者无法显式的声明返回值，在大多数时候这都不是问题
// 但是当配合 ? 一起使用时，问题就有所不同

async fn foo() -> Result<u8, String> {
    Ok(1)
}

async fn bar() -> Result<u8, String> {
    Ok(1)
}

fn main() {
    // println!("Hello, world!");
    // 直接编译下面的代码会报错：
    // error[E0282]: type annotations needed
    //   --> src\main-1:21:9
    //    |
    // 21 |         Ok(())
    //    |         ^^ cannot infer type of the type parameter `E` declared on the enum `Result`
    //    |
    // help: consider specifying the generic arguments
    //    |
    // 21 |         Ok::<(), E>(())
    //    |           +++++++++
    // let fut = async {
    //     foo().await?;
    //     bar().await?;
    //     Ok(())
    // };
    // 原因在于编译器无法推断出 Result<T, E> 中的 E 的类型
    // 因为 async 关键字定义的代码块
    // 返回值是 Ok 变体
    // 因此编译器推断其返回值类型为 Result
    // 但是编译器无法推断 Result 中 Err 变体的类型
    // 虽然我们这里使用了 ?，表示出现错误就返回 Err 变体，并且 Err 变体的类型是 String
    // 但是编译器可不知道这点

    // 解决方法是：指定泛型参数
    let fut = async {
        foo().await?;
        bar().await?;
        // 使用 ::< ... > 的方式来增加类型注释
        Ok::<(), String>(())
    };
}
