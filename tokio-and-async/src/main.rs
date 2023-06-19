use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::thread::sleep;
use std::time::Duration;



// 自定义一个 Future，实现 Future trait

struct ReadFileFuture {}

impl Future for ReadFileFuture {
    type Output = String;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Tokio Stop polling me");


        // 手动调用 waker 方法，通知 Tokio 运行时，异步任务已经准备好了，可以再次轮询了
        cx.waker().wake_by_ref();

        // Poll::Pending

        // 唤醒以后，返回一个 Poll::Ready
        Poll::Ready(String::from("Hello, there from file 1"))
    }
}




// 将 main 函数变成一个异步函数
#[tokio::main]
async fn main() {


    // 自定义 Future，并使用 Tokio 进行调度
    let h1 = tokio::spawn(
        async {
            let future1 = ReadFileFuture {};
            let file1_content = future1.await;
            println!("{:?}", file1_content);
        }
    );

    let h2 = tokio::spawn(
        async {
            let file2_content = read_from_file2().await;
            println!("{:?}", file2_content);
        }
    );

    tokio::join!(h1, h2);
    // 1. 在一个线程上调用两个会阻塞线程的函数
    // 函数会顺序执行，因为是在一个线程上执行，所以后一个任务会等前一个任务执行完成再执行
    // println!("Hello before reading file!");
    // let file1_content = read_from_file1();
    // println!("{:?}", file1_content);
    // println!("Hello after reading file1!");
    // let file2_content = read_from_file2();
    // println!("{:?}", file2_content);
    // println!("Hello after reading file2!");


    // 2. 使用多个线程执行异步任务，避免主线程阻塞
    // println!("Hello before reading file!");
    // let handle_1 = thread::spawn(|| {
    //     let file1_content = read_from_file1();
    //     println!("{:?}", file1_content);
    // });
    //
    // let handle_2 = thread::spawn(|| {
    //     let file2_content = read_from_file2();
    //     println!("{:?}", file2_content);
    // });
    //
    // handle_1.join().unwrap();
    // handle_2.join().unwrap();

    // 3. 使用 tokio 异步运行时完成异步任务
    // println!("Hello before reading file!");
    // // 生成异步任务
    // let h_1 = tokio::spawn(async {
    //     let _file1_content = read_from_file1().await;
    // });
    //
    // let h_2 = tokio::spawn(async {
    //     let _file2_content = read_from_file2().await;
    // });
    //
    // let _ = tokio::join!(h_1, h_2);
}

// 模拟异步读取文件的函数
// 阻塞线程 4 秒钟，然后返回值
// fn read_from_file1() -> String {
//     sleep(Duration::new(4, 0));
//     String::from("Hello, there from file 1")
// }
//
// // 模拟异步读取文件的函数
// // 阻塞线程 2 秒钟，然后返回值
// fn read_from_file2() -> String {
//     sleep(Duration::new(2, 0));
//     String::from("Hello, there from file 2")
// }


// 模拟异步读取文件的函数
// 阻塞线程 4 秒钟，然后返回值
// async fn read_from_file1() -> String {
//     sleep(Duration::new(4, 0));
//     println!("Processing file1");
//     String::from("Hello, there from file 1")
// }
//
// // 模拟异步读取文件的函数
// // 阻塞线程 2 秒钟，然后返回值
// async fn read_from_file2() -> String {
//     sleep(Duration::new(2, 0));
//     println!("Processing file2");
//     String::from("Hello, there from file 2")
// }



// async 定义的函数，会返回一个 Future 类型
// 下面的函数不使用 async 定义，但是返回 Future 类型，其与 async 定义的函数等价：

fn read_from_file1() -> impl Future<Output = String> {

    async {
        sleep(Duration::new(4, 0));
        println!("Processing file1");
        String::from("Hello, there from file 1")
    }


}

fn read_from_file2() -> impl Future<Output = String> {
    async {
        sleep(Duration::new(2, 0));
        // println!("Processing file2");
        String::from("Hello, there from file 2")
    }

}
