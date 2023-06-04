use futures::executor::block_on;
fn main() {
    // println!("Hello, world!");
    // 直接调用一个异步函数，不会有任何执行结果
    // 想要获得异步函数的结果，需要一个执行器
    // futures trait 中提供了很多的执行器：executor
    // block_on 就是其中之一
    let future = hello_world();
    // block_on 会阻塞当前线程直到指定的 Future 执行完成
    block_on(future);
}

// 使用 async 语法创建一个一部函数
// 异步函数的返回值是一个 Future，若直接调用该函数，不会输出任何结果，因为 Future 还未被执行
async fn hello_world() {
     println!("Hello, world!");
}


async fn hello_cat() {
    println!("Hello cat!");
}