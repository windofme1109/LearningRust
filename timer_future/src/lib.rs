// 使用 Waker 来唤醒任务
//
// 对于 Future 来说，第一次被 poll 时无法完成任务是很正常的
// 但它需要确保在未来一旦准备好时，可以通知执行器再次对其进行 poll 进而继续往下执行，该通知就是通过 Waker 类型完成的。
//
// Waker 提供了一个 wake() 方法可以用于告诉执行器：相关的任务可以被唤醒了
// 此时执行器就可以对相应的 Future 再次进行 poll 操作

// 实现一个定时器

// 基本思路是：当计时器创建时，我们会启动一个线程接着让该线程进入睡眠，等睡眠结束后再通知给 Future

// 新线程用来计时，Future 在主线程
// 当新线程计时结束，要通知主线程的 Future

use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

// 新建线程在睡眠结束后会需要将状态同步给定时器 Future ，由于是多线程环境
// 我们需要使用 Arc<Mutex<T>> 来作为一个共享状态，用于在新线程和 Future 定时器间共享
pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>
}


impl TimerFuture {
   /// 创建一个新的`TimerFuture`，在指定的时间结束后，该`Future`可以完成
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(
            SharedState {
                completed: false,
                waker: None
            }
        ));

        let thread_shared_state = shared_state.clone();

        // 创建一个新的线程
        thread::spawn(move || {
            // 让线程休眠指定的时间，以实现定时功能
            thread::sleep(duration);
            // 继续通过 lock 方法获得一个互斥锁
            let mut shared_state = thread_shared_state.lock().unwrap();

            // 定时结束，向外发出通知
            // 可以继续 poll 对应的 Future
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake();
            }
        });

        TimerFuture {
            shared_state
        }
    }

}

/// 在 Future 和等待的线程间共享状态
struct SharedState {
    // 定时（睡眠）是否结束
    completed: bool,
    // 当睡眠结束以后，线程可以使用 waker 通知 TimeFuture 来唤醒任务，即继续执行 poll
    waker: Option<Waker>
}

// 实现 Future trait
impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // lock 函数用来获取互斥锁，在获取过程中会阻塞当前的线程直到获取完成
        // 返回该互斥锁以后，该线程是唯一持有锁的线程
        let mut shared_state = self.shared_state.lock().unwrap();

        // 当 completed 为 true，说明定时器定时结束
        if shared_state.completed {
            // 此时异步任务完成
            Poll::Ready(())
        } else {
            // 当 completed 为 false，说明异步任务还没有结束
            // 我们需要设置 waker 函数
            // 设置 waker，这样新线程在睡眠（计时）结束后可以唤醒当前的任务，接着再次对`Future`进行`poll`操作
            // cx.waker() 会返回一个 Waker 的引用
            // Waker 是一个句柄（handle），用于通过通知其执行者它已准备好运行来唤醒任务。
            // 这个句柄封装了一个 RawWaker 实例，它定义了执行器特定的唤醒行为

            // 下面的 clone 每次被 poll 时都会发生一次，实际上，应该是只 clone 一次更加合理
            // 选择每次都 clone 的原因是： TimerFuture 可以在执行器的不同任务间移动，如果只克隆一次
            // 那么获取到的 waker 可能已经被篡改并指向了其它任务，最终导致执行器运行了错误的任务
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}
