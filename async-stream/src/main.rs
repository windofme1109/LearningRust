// async 与 stream
//
// async/.await 是 Rust 语法的一部分
// 它在遇到阻塞操作时( 例如 IO )会让出当前线程的所有权而不是阻塞当前线程，这样就允许当前线程继续去执行其它代码，最终实现并发
//
// 有两种方式可以使用 async：
// 1. async fn 用于声明函数
// 2. async { ... } 用于声明语句块
// 它们会返回一个实现 Future 特征的值


use std::future::Future;
use std::io;
use std::pin::Pin;
use std::sync::mpsc;
use std::task::{Context, Poll};
use futures::executor::block_on;

// 声明一个 async 函数
// foo 返回一个 Future<Output = u8>
// 当调用 foo().await 时，该 Future 将被运行，当调用结束后我们将获取到一个 u8 值
async fn foo() -> u8 {
    5
}

// 声明一个 async 代码块
fn bar() -> impl Future<Output = u8> {

    // 下面的代码块返回一个 Future<Output = u8>
    async {
        let x = foo().await;
        x +5
    }
}

// async 是懒惰的，直到被执行器 poll 或者 .await 后才会开始运行，其中后者是最常用的运行 Future 的方法
// 当 .await 被调用时，它会尝试运行 Future 直到完成，但是若该 Future 进入阻塞，那就会让出当前线程的控制权
// 当 Future 后面准备再一次被运行时(例如从 socket 中读取到了数据)，执行器会得到通知，并再次运行该 Future ，如此循环，直到完成

// async 的生命周期
//
// async fn 函数如果拥有引用类型的参数，那它返回的 Future 的生命周期就会被这些参数的生命周期所限制
// 就是说，引用类型的参数的生命周期要长于 Future
// 例如：
async fn foo_2(x: &u8) -> u8 { *x }

// 上面的函数跟下面的函数是等价的:
fn foo_expanded<'a>(x: &'a u8) -> impl Future<Output = u8> + 'a {
    // 在 Future 内部引用了 x
    async move { *x }
}

// 要求 Future 与 参数 x 的生命周期至少相同
// 当 x 依然有效时，该 Future 就必须继续等待( .await ), 也就是说 x 必须比 Future 活得更久

// 在一般情况下，在函数调用后就立即 .await 不会存在任何问题，例如foo(&x).await
// 若 Future 被先存起来或发送到另一个任务或者线程，就可能存在问题了
// 示例：
// fn bad() -> impl Future<Output=u8> {
//     let x = 5;
//     borrow_x(&x)
// }
//
async fn borrow_x(x: &u8) -> u8 {
    *x
}

// 在 borrow_x 函数中使用了引用类型 x，因此或报错：
// error[E0597]: `x` does not live long enough
//   --> src\main.rs:56:14
//    |
// 56 |     borrow_x(&x)
//    |     ---------^^-
//    |     |        |
//    |     |        borrowed value does not live long enough
//    |     argument requires that `x` is borrowed for `'static`
// 57 | }
//    | - `x` dropped here while still borrowed
// 因为 x 只存活到 bad 函数末尾，而我们不知道 borrow_x 函数内部什么时候使用 x，因为 borrow_x 是异步的，这里并没有立即执行
// 解决办法是：将具有引用参数的 async fn 函数转变成一个具有 'static 生命周期的 Future

fn good() -> impl Future<Output = u8> {
    async {
        let x = 5;
        borrow_x(&x).await
    }
}

// 通过将参数移动到 async 语句块内， 我们将它的生命周期扩展到 'static， 并跟返回的 Future 保持了一致
// 因为 async 返回一个 Future，在这个 Future 内定义并使用的引用类型，其生命周期显然与 Future 一致


// 3. async move
//
// async 允许我们使用 move 关键字来将环境中变量的所有权转移到语句块内，就像闭包那样
// 好处是你不再发愁该如何解决借用生命周期的问题
// 坏处就是无法跟其它代码实现对变量的共享

async fn blocks() {
    let my_string = "foo".to_string();
    let future_one = async {
        println!("{}", my_string);
    };
    let future_two = async {
        println!("{}", my_string);
    };
   // 运行两个 Future 直到完成
    let ((), ()) = futures::join!(future_one, future_two);
}

// 使用 async move 捕获作用域中的变量，使其只能作用一个 async 块
fn move_block() -> impl Future<Output=()> {
    let my_string = "foo".to_string();
    async move {
        println!("{}", my_string);
    }

}

async fn move_block_2() {
    let my_string = "foo".to_string();
    async move {
        println!("{}", my_string);
    };

    // async {
    //     println!("{}", my_string);
    // };
    //
    // 编译代码，会报错：
    // error[E0382]: borrow of moved value: `my_string`
    //    --> src\main.rs:122:5
    //     |
    // 117 |       let my_string = "foo".to_string();
    //     |           --------- move occurs because `my_string` has type `String`, which does not implement the `Copy` trait
    // 118 | /     async move {
    // 119 | |         println!("{}", my_string);
    //     | |                        --------- variable moved due to use in generator
    // 120 | |     };
    //     | |_____- value moved here
    // 121 |
    // 122 | /     async {
    // 123 | |         println!("{}", my_string);
    //     | |                        --------- borrow occurs due to use in generator
    // 124 | |     };
    //     | |_____^ value borrowed here after move

    // 第一个 async 代码块使用了 move 关键字，因此 my_string 移动到了其内部
    // 所以第二个 async 代码块中使用 my_string 就会报错
}

// 4. 当 .await 遇见多线程执行器
//
// 需要注意的是，当使用多线程 Future 执行器( executor )时， Future 可能会在线程间被移动，因此 async 语句块中的变量必须要能在线程间传递
// 至于 Future 会在线程间移动的原因是：它内部的任何 .await 都可能导致它被切换到一个新线程上去执行
//
// 由于需要在多线程环境使用，意味着 Rc、 RefCell 、没有实现 Send 的所有权类型、没有实现 Sync 的引用类型，它们都是不安全的，因此无法被使用
// todo 没有在作用域的含义，指的是没有在 .await 的调用期间，不去使用这些类型吗
// 需要注意！实际上它们还是有可能被使用的，只要在 .await 调用期间，它们没有在作用域范围内
//
// 类似的原因，在 .await 时使用普通的锁也不安全，例如 Mutex
// 原因是，它可能会导致线程池被锁：当一个任务获取锁 A 后，若它将线程的控制权还给执行器，然后执行器又调度运行另一个任务，该任务也去尝试获取了锁 A ，结果当前线程会直接卡死，最终陷入死锁中
//
// 因为异步任务阻塞线程后，会将线程的控制权让出
//
// 因此，为了避免这种情况的发生，我们需要使用 futures 包下的锁 futures::lock 来替代 Mutex 完成任务

// 5. Stream 流处理
//
// Stream trait 类似于 Future trait，但是前者在完成前可以生成多个值，这种行为跟标准库中的 Iterator trait 倒是颇为相似
// Stream 是 futures 下的一个 trait，引入方式是：
// use futures::stream

// stream 中包括：
// Stream trait，适用于可以异步生成值序列的对象
// StreamExt 和 TryStreamExt trait，提供用于链接和组合流的适配器
// 顶级流构造函数，如 iter，它从迭代器创建流

// Stream trait 定义
// trait Stream {
//     // Stream生成的值的类型
//     type Item;
//
//     // 尝试去解析Stream中的下一个值,
//     // 若无数据，返回 Poll::Pending, 若有数据，返回 Poll::Ready(Some(x)), Stream 完成则返回 Poll::Ready(None)
//     fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
// }

// Stream 结束的标志是：Poll::Ready(None)，也就是说，碰到 None 变体，就结束 Stream 操作

// 关于 Stream 的一个常见例子是消息通道（ futures 包中的）的消费者 Receiver
// 每次有消息从 Send 端发送后，它都可以接收到一个 Some(val) 值， 一旦 Send 端关闭(drop)，且消息通道中没有消息后，它会接收到一个 None 值
// 下面的例子只是演示一下 Receiver 与 Stream 的类似之处
// Receiver 并没有 next 方法，也不支持异步操作
// async fn send_recv() {
//     const BUFFER_SIZE: usize = 10;
//     let (mut tx, mut rx) = mpsc::channel::<i32>(BUFFER_SIZE);
//
//     tx.send(1).await.unwrap();
//     tx.send(2).await.unwrap();
//     drop(tx);
//
//     // `StreamExt::next` 类似于 `Iterator::next`, 但是前者返回的不是值，而是一个 `Future<Output = Option<T>>`，
//     // 因此还需要使用`.await`来获取具体的值
//     assert_eq!(Some(1), rx.next().await);
//     assert_eq!(Some(2), rx.next().await);
//     assert_eq!(None, rx.next().await);
// }

// 对 Stream 进行迭代和并发
//
// 跟迭代器类似，我们也可以迭代一个 Stream。 例如使用 map，filter，fold 方法，以及它们的遇到错误提前返回的版本： try_map，try_filter，try_fold。
//
// 但是跟迭代器又有所不同，for 循环无法在这里使用，但是命令式风格的循环 while let 是可以用的，同时还可以使用 next 和 try_next 方法：
use futures::stream::Stream;
async fn sum_with_next(mut stream: Pin<&mut dyn Stream<Item = i32>>) -> i32 {
    // 引入 next
    use futures::stream::StreamExt;
    let mut sum = 0;
    while let Some(item) = stream.next().await {
        sum += item;
    }

    sum
}

async fn sum_with_try_next(mut stream: Pin<&mut dyn Stream<Item = Result<i32, io::Error>>>) -> Result<i32, io::Error> {
    use futures::stream::TryStreamExt;
    let mut sum = 0;
    // 问号运算符 (?) 解包有效值或返回错误值，将它们传播到调用函数
    // 它是一元后缀运算符，只能应用于类型 Result<T, E> 和 Option<T>
    while let Some(item) = stream.try_next().await? {
        sum += item;
    }

    Ok(sum)
}

// 上面代码是一次处理一个值的模式，但是需要注意的是：如果你选择一次处理一个值的模式，可能会造成无法并发，这就失去了异步编程的意义
// 因此，如果可以的话我们还是要选择从一个 Stream 并发处理多个值的方式，通过 for_each_concurrent 或 try_for_each_concurrent 方法来实现

async fn jump_around(mut stream: Pin<&mut dyn Stream<Item = Result<u8, io::Error>>>,
) -> Result<(), io::Error> {
    // 引入 try_for_each_concurrent
    use futures::stream::TryStreamExt;
    const MAX_CONCURRENT_JUMPERS: usize = 100;
    // try_for_each_concurrent 方法是 TryStreamExt 提供的方法
    // 主要作用是：尝试运行这个流，当流中的元素可用时同时为流中的每个元素执行提供的异步闭包，一旦发生错误就退出
    // 因此可以在异步闭包中执行异步任务
    // 而 try_for_each_concurrent的返回值又是一个异步任务
    stream.try_for_each_concurrent(MAX_CONCURRENT_JUMPERS, |num| async move {
        // 异步闭包内还可以继续执行异步任务
        jump_n_times(num).await?;
        report_n_jumps(num).await?;
        Ok(())
    }).await?;

    Ok(())
}

// Stream 是异步流，可以迭代和并发异步任务
// 详细说明，可以参考文档：https://docs.rs/futures/0.3.28/futures/stream/index.html

fn main() {
    println!("Hello, world!");

    block_on(blocks());
}
