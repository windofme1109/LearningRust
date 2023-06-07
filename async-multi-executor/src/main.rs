// 同时执行多个 Future
// join!
//
// futures 包中提供了很多实用的工具，其中一个就是 join! 宏
// 它允许我们同时等待多个不同 Future 的完成，且可以并发地运行这些 Future

struct Book;

async fn enjoy_book() -> Book {
    Book
}

struct Music;

async fn enjoy_music() -> Music {
    Music
}


async fn enjoy_book_and_music() -> (Book, Music) {
    let book = enjoy_book().await;
    let music = enjoy_music().await;
    (book, music)
}

// 上面的代码可以执行，但是执行时由先后顺序的，必须先执行 enjoy_book，再执行 enjoy_music

// 如果我们想同时执行 enjoy_book 和 enjoy_music，即并发的执行 enjoy_book 和 enjoy_music

// 我们可能这样写：

async fn concurrent_enjoy_book_and_music() -> (Book, Music) {
    let book_future = enjoy_book();
    let music_future = enjoy_music();
    (book_future.await, music_future.await)
}

// 看上去像模像样，这种做法在某些语言中也许可以，但是 Rust 不行
// 因为在某些语言中，Future 一旦创建就开始运行，等到返回的时候，基本就可以同时结束并返回了
// 但是 Rust 中的 Future 是惰性的，直到调用 .await 时，才会开始运行
// 而那两个 await 由于在代码中有先后顺序，因此它们是顺序运行的
// 就是说，还是要等到 book_future 执行完，music_future 才能继续执行

// 为了正确的并发运行两个 Future， 需要使用 futures::join! 宏
use futures::join;
async fn concurrent_enjoy_book_and_music_with_join() -> (Book, Music) {
    let book_future = enjoy_book();
    let music_future = enjoy_music();
    join!(book_future, music_future)
}


// join! 会返回一个元组，里面的值是对应的 Future 执行结束后输出的值
// 如果希望同时运行一个数组里的多个异步任务，可以使用 futures::future::join_all 方法

// 2. try_join!
//
// 由于 join! 必须等待它管理的所有 Future 完成后才能完成
// 如果你希望在某一个 Future 报错后就立即停止所有 Future 的执行，可以使用 try_join!，特别是当 Future 返回 Result 时
// try_join! 接收多个 Future，如果 每个 Future 都返回 Ok 变体，那么 try_join! 最终返回一个由 Ok 变体组成的元组
// 只要由一个 Future 返回 Err 变体，那么 try_join! 就会返回这个错误
use futures::try_join;
async fn get_book() -> Result<Book, String> { /* ... */ Ok(Book) }
async fn get_music() -> Result<Music, String> { /* ... */ Ok(Music) }
async fn get_book_and_music() -> Result<(Book, Music), String> {
    let book = get_book();
    let music = get_music();
    try_join!(book, music)
}

// 传给 try_join! 的所有 Future 都必须拥有相同的错误类型
// 如果错误类型不同，可以考虑使用来自 futures::future::TryFutureExt 模块的 map_err 和 err_info 方法将错误进行转换
use futures::TryFutureExt;

async fn get_book_2() -> Result<Book, ()> { /* ... */ Ok(Book) }
async fn get_music_2() -> Result<Music, String> { /* ... */ Ok(Music) }
async fn get_book_and_music_2() -> Result<(Book, Music), String> {
    // map_err 函数是由 TryFutureExt 提供的函数
    // 在使用 map_err 之前，必须导入 TryFutureExt trait
    // map_err 的主要作用是：将 future 的错误类型转换，接收一个闭包作为参数
    // 闭包内，可以进行错误转换
    let book_fut = get_book_2().map_err(|()| "Unable to get book".to_string());
    let music_fut = get_music();
    try_join!(book_fut, music_fut)
}

// select！

// join! 只有等所有 Future 结束后，才能集中处理结果
// 如果你想同时等待多个 Future ，且任何一个 Future 结束后，都可以立即被处理，可以考虑使用 futures::select!

use futures::{
    future::FutureExt,
    pin_mut,
    select
};


async fn task_one() { /* ... */ }
async fn task_two() { /* ... */ }

async fn race_task() {

    // fuse 函数的主要作用是：
    // 一旦异步操作完成，那么 future 中的 poll 函数就不会被调用
    // 将普通的 Future 变成 FusedFuture
    // 通常情况下，一旦 future 的 `poll` 返回了 `Poll::Ready`
    // 任何进一步的调用都可能表现出不良行为，例如永远阻塞线程，发生 panic，永不返回等
    // 如果已知 poll 函数可能被调用得太频繁
    // 那么这个方法可以使用 fuse 方法，来告诉调用者，一旦异步任务完成，就不能继续调用 poll 函数
    let t1 = task_one().fuse();
    let t2 = task_two().fuse();

    // 前面说过 pin_mut! 宏的作用，将一个值固定到栈中
    // 将 t1 和 t2 pin 住，防止其被移动
    pin_mut!(t1, t2);

    // 并行执行 t1 和 t2 两个异步任务
    // 无论两者哪个先完成，都会执行对应代码块中的代码
    select! {
        () = t1 => println!("任务1率先完成"),
        () = t2 => println!("任务2率先完成"),
    }
}

// 上面的代码会同时并发地运行 t1 和 t2
// 无论两者哪个先完成，都会调用对应的 println! 打印相应的输出，然后函数结束且不会等待另一个任务的完成

// default 和 complete
//
// select!还支持 default 和 complete 分支:
//
// complete 分支当所有的 Future 和 Stream 完成后才会被执行，它往往配合 loop 使用，loop 用于循环完成所有的 Future
// default 分支，若没有任何 Future 或 Stream 处于 Ready 状态， 则该分支会被立即执行

// 与 Unpin 和 FusedFuture 进行交互
// 在使用 select! 的示例中，有一个这样的例子：
// let t1 = task_one().fuse();
// let t2 = task_two().fuse();
//
// pin_mut!(t1, t2);

// .fuse() 方法可以让 Future 实现 FusedFuture trait
// 而 pin_mut! 宏会为 Future 实现 Unpin trait，这两个特征恰恰是使用 select 所必须的:
// 1. Unpin，由于 select 不会通过拿走所有权的方式使用 Future，而是通过可变引用的方式去使用
//    这样当 select 结束后，该 Future 若没有被完成，它的所有权还可以继续被其它代码使用
// 2. FusedFuture 的原因跟上面类似，当 Future 一旦完成后，那 select 就不能再对其进行轮询使用
//    Fuse 意味着熔断，相当于 Future 一旦完成，再次调用 poll 会直接返回 Poll::Pending
// 只有实现了 FusedFuture，select 才能配合 loop 一起使用
// 假如没有实现，就算一个 Future 已经完成了，它依然会被 select 不停的轮询执行

// Stream 稍有不同，它们使用的特征是 FusedStream，通过 .fuse()(也可以手动实现)实现了该特征的 Stream
// 对其调用 .next() 或 .try_next() 方法可以获取实现了 FusedFuture 特征的Future

use futures::future;

fn main() {
    // ready 方法的作用是：创建一个 Ready 状态的 Future，并包裹一个指定的值
    let mut a_fut = future::ready(4);
    let mut b_fut = future::ready(6);
    let mut total = 0;
    // println!("Hello, world!");

    loop {
        // select! 宏接收的是一个模式匹配
        // 分支可以是 Future、complete 和 default
        // Future 的匹配条件是：Future 变成 Ready 状态
        // complete 匹配条件是：所有的 Future 完成
        // default 若没有任何 Future 或 Stream 处于 Ready 状态， 则该分支会被立即执行
        select! {
            a = a_fut => total += a,
            b = b_fut => total += b,
            // a_fut 和 b_fut 全部执行完成后，才会执行 complete 分支
            complete => break,
            // 该分支永远不会运行，因为 Future 已变成 Ready 状态，所以 Future 会先运行，然后是 complete
            default => panic!()
        }
    }

    assert_eq!(10, total)
}
