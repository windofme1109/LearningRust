// 自定义一个 timer future
// 实现 Future trait
// 使用 Tokio 作为执行器


use std::time::{Duration, Instant};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread::sleep;

struct AsyncTimer {
    // 过期时间，是一个 Instant 实例,Instant 可以用于获取和当前时间相关的内容
    expiration_time: Instant,
}

impl Future for AsyncTimer {
    type Output = String;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 首先获取当前时刻，判断是否过期
        // 如果过期，就返回 Poll::Ready，表示异步任务已经完成
        if Instant::now() >= self.expiration_time {
            println!("Hello, it's time for Future 1");
            Poll::Ready(String::from("Future 1 has completed"))
        } else {
            // 如果没有过期
            println!("Hello, it's not yet time for Future 1. Going to sleep");
            // 获得 waker 的克隆
            let waker = cx.waker().clone();
            let expiration_time = self.expiration_time;
            // 没有过期，启动一个线程
            std::thread::spawn(move || {
               let current_time = Instant::now();

                // 继续判断启动线程的时候是否过期，如果没有过期，让线程休眠指定的时间
                if current_time < expiration_time {
                    sleep(expiration_time - current_time);
                }

                // 线程休眠结束，调用 waker 中的 wake 方法，表示我们可以继续轮询，执行异步任务
                waker.wake();
            });
            // 返回 Poll::Pending，表示异步任务还没有完成
            Poll::Pending
        }
    }
}


fn read_from_file2() -> impl Future<Output = String> {
    async {
        sleep(Duration::new(2, 0));
        // println!("Processing file2");
        String::from("Future 2 has Completed")
    }

}

#[tokio::main]
async fn main() {
    // println!("Hello, world!");
    let h1 = tokio::spawn(async {
        // Instant 中的 now 方法返回当前的时刻，
        // Duration 中的 from_millis 方法接收一个数字，生成一个毫秒形式的持续时间
        // Instant::now() + Duration::from_millis(4000) 表示：生成 4000 毫秒后的时刻，这个时刻表示定时器的过期时间
        let future1 = AsyncTimer {
            expiration_time: Instant::now() + Duration::from_millis(4000)
        };

        let res = future1.await;

        println!("{:?}", res);
    });
    println!("{:?}", Instant::now());

    let h2 = tokio::spawn(async {
        let file2_content = read_from_file2().await;
        println!("{:?}", file2_content);
    });

    // 使用 tokio 作为异步任务的执行器

    let _ = tokio::join!(h1, h2);

}
