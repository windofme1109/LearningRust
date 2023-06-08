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
// 而 pin_mut! 宏会为 Future 实现 Unpin trait，这两个 trait 恰恰是使用 select 所必须的:
// 1. Unpin，由于 select 不会通过拿走所有权的方式使用 Future，而是通过可变引用的方式去使用
//    这样当 select 结束后，该 Future 若没有被完成，它的所有权还可以继续被其它代码使用
// 2. FusedFuture 的原因跟上面类似，当 Future 一旦完成后，那 select 就不能再对其进行轮询使用
//    Fuse 意味着熔断，相当于 Future 一旦完成，再次调用 poll 会直接返回 Poll::Pending
// 只有实现了 FusedFuture，select 才能配合 loop 一起使用
// 假如没有实现，就算一个 Future 已经完成了，它依然会被 select 不停的轮询执行

// Stream 稍有不同，它们使用的 trait 是 FusedStream，通过 .fuse()(也可以手动实现)实现了该 trait 的 Stream
// 对其调用 .next() 或 .try_next() 方法可以获取实现了 FusedFuture trait 的 Future

use futures::{stream::{Stream, StreamExt, FusedStream}};

async fn add_two_streams(
    mut s1: impl Stream<Item = u8> + FusedStream + Unpin,
    mut s2: impl Stream<Item = u8> + FusedStream + Unpin

) -> u8 {
    let mut total = 0;
    loop {

        // 
        let item = select! {
            // s1 的 next 方法可以获得实现了 FusedFuture trait 的 Future
            // 在 Future 变成 Ready 状态后，就能获取 Ready 包裹的值
            // s1 和 s2 有一个 Future 变成 Ready 状态，那么整个代码块的返回值就是这个 Ready 包裹的值 
            // Ready 状态中包裹的值就是 Some 变体
            x = s1.next() => x,
            x = s2.next() => x,
            complete => break
        };

        // item 是 Some 变体，表示 item 成功的接收了异步 stream 任务的返回值
        if let Some(next_num) = item {
            total = next_num;
        }
    }

    total
}


// 在 select 循环中并发

// 一个很实用但又鲜为人知的函数是 Fuse::terminated() ，可以使用它构建一个空的 Future 
// 空自然没啥用，但是如果它能在后面再被填充呢？

// 考虑以下场景：当你要在 select 循环中运行一个任务，但是该任务却是在 select 循环内部创建时，上面的函数就非常好用了

use futures::{future::{Fuse, FusedFuture}};


// 新建两个异步任务
async fn get_new_num() -> u8 { /* ... */ 5 }

async fn run_on_new_num(_: u8) { /* ... */ }

/// interval_timer 参数，要求实现 Stream trait、FuseStream trait 和 Unpin trait
/// interval_timer 是一个异步 Stream
async fn run_loop(
    mut interval_timer: impl Stream<Item = u8> + FusedStream + Unpin,
    starting_num: u8
) {
    // 生成一个 FusedFuture
    let run_on_new_num_fut = run_on_new_num(starting_num).fuse();

    // terminated 创建一个已经终止的新的 Fuse-wrapped 的 future
    // 这可以与循环 loop 和 select! 结合使用，在匹配过程中会绕过终止（terminated）的 future
    let get_new_num_fut = Fuse::terminated();

    //
    pin_mut!(run_on_new_num_fut, get_new_num_fut);

    // loop 可以理解为是在不停的轮询，看哪个异步任务完成
    loop {
        select! {
            // select_next_some 方法作用：当 stream 中的下一项也是 Ready 状态会返回一个 ready 状态的 Future
            // 该方法与 next 方法类似，但是区别是：如果在一个空的 stream 上，select_next_some 方法不会返回 None
            // 取而代之的是，返回的 Future 类型将从 FusedFuture::is_terminated 方法中返回 true
            // select_next_some 与 select! 可以一起使用
            // interval_timer 是一个异步任务流
            // select_next_some 会一步一步向下执行
            () = interval_timer.select_next_some() => {
                // 定时器已结束，若 get_new_num_fut 没有在运行，就创建一个新的
                // 如果潜在的 Future 不会再被查询（polled），那么就返回 true
                // 也就是说，get_new_num_fut 这个异步任务已经结束了
                if get_new_num_fut.is_terminated() {
                    // set 方法的作用是：为固定引用后面的内存分配一个新值
                    // 这个操作会覆盖 pin 住的数据，但是这样做是没有关系的，其析构函数会在覆盖前运行，因此没有违反 pin 规则
                    // 所以，我们这里使用新的异步任务替换原来的任务
                    get_new_num_fut.set(get_new_num().fuse())

                    // 这里在 select! 宏内部创建新的异步任务，然后将其放到外部的容器中，这个容器就是 Fuse::terminated 方法创建的
                }
            },
            // get_new_num_fut 这个异步任务执行成功，返回一个数字
            new_num = get_new_num_fut => {
                // 收到新的数字 -- 创建一个新的 run_on_new_num_fut 并丢弃掉旧的
                run_on_new_num_fut.set(run_on_new_num().fuse())
            },
            // 运行 run_on_new_num_fut
            () = run_on_new_num_fut => {},
            // 若所有任务都完成，直接 panic
            // 原因是 interval_timer 应该连续不断的产生值，而不是结束后，执行到 complete 分支
            complete => panic!("`interval_timer` completed unexpectedly")
        }
    }
}

// 当某个 Future 有多个拷贝都需要同时运行时，可以使用 FuturesUnordered 类型
// 下面的例子跟上个例子大体相似，但是它会将 run_on_new_num_fut 的每一个拷贝都运行到完成
// 而不是像之前那样一旦创建新的就终止旧的


use futures::{stream::FuturesUnordered};

// 使用从 get_new_num 获取的最新数字 来运行  run_on_new_num
//
// 每当计时器结束后，get_new_num 就会运行一次
// 它会立即取消当前正在运行的 run_on_new_num
// 并且使用新返回的值来替换
async fn run_loop_2(
    mut interval_timer: impl Stream<Item = ()> + FusedStream + Unpin,
    starting_num: u8,
) {
    // FuturesUnordered 是一个结构体
    // 其主要作用是：可以以任何顺序完成一系列的 future，体现了 unorder 的含义
    // 这种结构经过优化，可以用来管理大量的 future
    // 由 FuturesUnordered 管理的 future，只有在其生成 wake-up 通知时才会被轮询
    // 这减少了轮询大量 future 所需的工作量
    // 可以通过收集（collecting）由 future 组成的迭代器来填充 FuturesUnordered
    //
    // 或者将 futures 推到现有的 FuturesUnordered
    // 添加新 future 时，必须调用 poll_next 才能开始接收新 future 的 wake-up
    // 注意，可以通过 collect 方法创建一个现成的 FuturesUnordered
    // 或者您可以使用 FuturesUnordered::new 构造函数创建一个空的集合
    let mut run_on_new_num_futs = FuturesUnordered::new();

    // 添加新的 future
    run_on_new_num_futs.push(run_on_new_num(starting_num));

    let get_new_num_fut = Fuse::terminated();

    pin_mut!(get_new_num_fut);
    loop {
        select! {
            () = interval_timer.select_next_some() => {
                // 定时器已结束，若 get_new_num_fut 没有在运行，就创建一个新的
                // 新的 get_new_num_fut 会覆盖原来的任务
                if get_new_num_fut.is_terminated() {
                    get_new_num_fut.set(get_new_num().fuse());
                }
            },
            new_num = get_new_num_fut => {
                // 收到新的数字 -- 创建一个新的 run_on_new_num_fut
                // 这里没有覆盖前一个 future，而是向 run_on_new_num_futs 推送新的
                run_on_new_num_futs.push(run_on_new_num(new_num));
            },
            // 运行 run_on_new_num_futs, 并检查是否有已经完成的
            // 依次执行 run_on_new_num_futs 中的异步任务
            res = run_on_new_num_futs.select_next_some() => {
                println!("run_on_new_num_fut returned {:?}", res);
            },
            // 若所有任务都完成，直接 panic， 原因是 interval_timer 应该连续不断的产生值，而不是结束后，执行到 complete  分支
            complete => panic!("`interval_timer` completed unexpectedly"),
        }
    }
}

// run_loop_2 的整体执行流程是：
// 1. 定时异步流依次执行异步任务
// 2. 每个定时任务执行结束：
//    如果 get_new_num_fut 执行结束，那么创建一个新的任务 get_new_num_fut，并替换原来的任务
// 3. 因为是 loop 循环，所以在等待定时异步任务流执行的同时，get_new_num_fut 也在执行，因此在 select 代码块中对应分支中，处理其执行成功的情况：
//    创建一个新的 run_on_new_num_fut，并向 run_on_new_num_futs 推送新的 future
// 4. 运行 run_on_new_num_futs 中的异步任务, 并检查是否有已经完成的

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
