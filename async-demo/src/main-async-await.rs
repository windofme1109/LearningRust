use futures::executor::block_on;
fn main() {
    // println!("Hello, world!");
    // 直接调用一个异步函数，不会有任何执行结果
    // 想要获得异步函数的结果，需要一个执行器
    // futures trait 中提供了很多的执行器：executor
    // block_on 就是其中之一
    // let future = hello_world();
    // // block_on 会阻塞当前线程直到指定的 Future 执行完成
    // block_on(future);

    // 调用多个异步函数
    // 使用 block_on 三次阻塞去等待三个任务的完成
    // let song = block_on(learn_song());
    // block_on(sing_song(song));
    // block_on(dance());

    //
    block_on(async_main());
}

// 使用 async 语法创建一个一部函数
// 异步函数的返回值是一个 Future，若直接调用该函数，不会输出任何结果，因为 Future 还未被执行
async fn hello_world() {
    // 直接在一个异步函数中调用另外一个异步函数，也是没有任何执行结果
    // hello_cat();
    // 想要获得执行结果，可以使用 .await 语法
    // 在 async fn 函数中使用 .await 可以等待另一个异步调用的完成
    // 但是与 block_on 不同，.await 并不会阻塞当前的线程，而是异步的等待 Future A 的完成
    // 在等待的过程中，该线程还可以继续执行其它的 Future B，最终实现了并发处理的效果
    hello_cat().await;
    println!("Hello, world!");
}


async fn hello_cat() {
    println!("Hello cat!");
}

struct Song {
    author: String,
    name: String
}



async fn learn_song() -> Song {
    Song {
        author: String::from("周杰伦"),
        name: String::from("发如雪")
    }
}

async fn sing_song(song: Song) {
    println!(
        "唱一首{}的{} ~ {}",
        song.author, song.name, "狼牙月，伊人憔悴"
    );
}

async fn learning_and_sing() {
    // 这里使用 .await 来等待学歌的完成，但是并不会阻塞当前线程，该线程在学歌的任务
    // .await 后，完全可以去执行跳舞的任务
    let song = learn_song().await;
    // let song = learn_song().await;


    // 唱歌必须在学歌之后
    sing_song(song).await;
}

async fn dance() {
    println!("唱到情深处，身体不由自主的动了起来~ ~");
}

async fn async_main() {
    let f1 = learning_and_sing();
    let f2 = dance();
    // join! 可以并发的处理和等待多个 Future
    // 若 learn_and_sing Future 被阻塞，那 dance Future 可以拿过线程的所有权继续执行
    // 若 dance 也变成阻塞状态，那 learn_and_sing 又可以再次拿回线程所有权，继续执行。
    // 若两个都被阻塞，那么 async main.rs 会变成阻塞状态，然后让出线程所有权
    // 并将其交给 main.rs 函数中的 block_on 执行器
    futures::join!(f1, f2);
}