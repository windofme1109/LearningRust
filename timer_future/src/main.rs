// 执行器 Executor
//
// Rust 的 Future 是惰性的：只有屁股上拍一拍，它才会努力动一动
// 其中一个推动它的方式就是在 async 函数中使用 .await 来调用另一个 async 函数，但是这个只能解决 async 内部的问题
// 那么这些最外层的 async 函数，谁来推动它们运行呢？答案就是我们之前多次提到的执行器 executor
//
// 执行器会管理一批 Future (最外层的 async 函数)，然后通过不停地 poll 推动它们直到完成
// 最开始，执行器会先 poll 一次 Future ，后面就不会主动去 poll 了
// 而是等待 Future 通过调用 wake 函数来通知它可以继续，它才会继续去 poll
// 这种 wake 通知然后 poll 的方式会不断重复，直到 Future 完成

// future 提供 ArcWaker trait，提供了一个方便的途径去构建一个 Waker
use {
    futures::{
        future::{BoxFuture, FutureExt},
        task::{waker_ref, ArcWake},
    },
    std::{
        future::Future,
        sync::mpsc::{sync_channel, Receiver, SyncSender},
        sync::{Arc, Mutex},
        task::{Context, Poll},
        time::Duration,
    },

};

// 引入之前实现的定时器模块
use timer_future::TimerFuture;

// 执行器需要从一个消息通道( channel )中拉取事件，然后运行它们
// 当一个任务准备好后（可以继续执行），它会将自己放入消息通道中，然后等待执行器 poll

/// 任务执行器，负责从通道中接收任务然后执行
struct Executor {
    // Receiver 是 Rust 通道（或 sync_channel）类型的接收端
    // 这一半只能由一个线程拥有
    ready_queue: Receiver<Arc<Task>>
}

/// `Spawner`负责创建新的 Future 然后将它发送到任务通道中
#[derive(Clone)]
struct Spawner {
    // 同步通道的发送端（sync_channel），同 Receiver 类似，发送端这一半只能由一个线程拥有
    task_sender: SyncSender<Arc<Task>>
}

/// 一个 Future，它可以调度自己（将自己放入任务通道中），然后等待执行器去 poll
struct Task {
    /// 进行中的 Future，在未来的某个时间点会被完成
    ///
    /// 按理来说 Mutex 在这里是多余的，因为我们只有一个线程来执行任务
    /// 但是由于 Rust 并不聪明，它无法知道 Future 只会在一个线程内被修改，并不会被跨线程修改
    /// 因此我们需要使用 Mutex 来满足这个笨笨的编译器对线程安全的执着
    ///
    /// 如果是生产级的执行器实现，不会使用 Mutex，因为会带来性能上的开销，取而代之的是使用 UnsafeCell
    ///
    /// BoxFuture 是一个拥有的动态类型的 Future，用于您不能静态输入结果或需要添加一些间接的情况
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    /// 可以将该任务自身放回到任务通道中，等待执行器的 poll
    task_sender: SyncSender<Arc<Task>>,
}

fn new_executor_and_spawner() -> (Executor, Spawner) {

    // 任务通道允许的最大缓冲数(任务队列的最大长度)
    // 当前的实现仅仅是为了简单，在实际的执行中，并不会这么使用
    const MAX_QUEUED_TASKS: usize = 10_000;

    // sync_channel 创建一个新的同步有界通道
    // 在 SyncSender 上发送的所有数据将以与发送时相同的顺序在 Receiver 上可用
    // 与异步通道一样，接收器将阻塞直到消息可用，sync_channel 在发送者的语义上有很大的不同
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    // 将通道的发送端和接收段分别指定给 Spawner 和 Executor
    (Executor {ready_queue}, Spawner {task_sender})
}

// 生成一个 Future
impl Spawner {

    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {

        // todo boxed 方法暂时没有查到其声明，后面需要搞懂 boxed 方法的基本用法
        // 猜测是将 future 进行装箱
        let future = future.boxed();
        // 生成一个 task，将刚刚生成的 Future 放入其中，并绑定 task_sender（任务发送端）
        let task = Arc::new( Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone()
        });

        // 将刚刚的 task 发送到通道中
        self.task_sender.send(task).expect("任务队列已满");
    }
}

/// 在执行器 poll 一个 Future 之前，首先需要调用 wake 方法进行唤醒，然后再由 Waker 负责调度该任务并将其放入任务通道中
/// 创建 Waker 的最简单的方式就是实现 ArcWake trait
///
/// ArcWake trait 是一种唤醒特定任务的方法
///
/// 通过实现此 trait，可以将预期包装在 Arc 中的类型转换为 Waker 对象
/// 这些唤醒器可用于向执行者发出信号，表明它拥有的任务已准备好再次轮询
impl ArcWake for Task {
    // wake_by_ref 表示关联的任务已准备好取得进展并且应该被轮询
    //
    // 可以从任意线程调用此函数，包括未创建基于 ArcWake 的 Waker 的线程
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let clone = arc_self.clone();

        // 通过发送任务到任务管道的方式来实现 wake
        // 这样 wake 后，任务就能被执行器 poll
        arc_self.task_sender.send(clone).expect("任务队列已满");
    }
}

// 当 Task 实现了 ArcWake trait 后，它就变成了 Waker，在调用 wake() 对其唤醒后会将 Task 复制一份所有权( Arc )
// 然后将其发送到任务通道中。最后我们的执行器将从通道中获取任务，然后进行 poll 执行
// 因此需要为 Executor 实现一个执行方法

impl Executor {
    fn run(&self) {

        // 获取一个 future，若它还没有完成(仍然是Some，不是None)，则对它进行一次 poll 并尝试完成它
        // ready_queue 是一个 Receiver，recv 方法用于从通道中获取值
        // 如果没有可用数据并且有可能发送更多数据（至少仍然存在一个 Sender），此函数将始终阻塞当前线程
        // 一旦消息被发送到相应的 Sender（或 SyncSender），这个接收者将被唤醒并返回该消息
        // 这里使用 while let 的意义是：要一直检查是不是收到了数据
        while let Ok(task) = self.ready_queue.recv() {
            // 从 task 中取出 Future
            let mut future_slot = task.future.lock().unwrap();
            // 尝试从 future_slot（互斥锁）中取出 Some 变体
            if let Some(mut future) = future_slot.take() {
                // 基于任务自身创建一个 LocalWaker
                // waker_ref 函数的作用是：从对 Arc<impl ArcWake> 的引用创建对 Waker 的引用
                // Task 实现了 ArcWaker trait，因此其是一个 waker
                let waker = waker_ref(&task);


                // 将一个 &Waker 类型转换为一个 Context
                //Context 类型表示一个异步任务的上下文
                // 目前，Context 仅用于提供对可用于唤醒当前任务的 &Waker 的访问
                let context = &mut Context::from_waker(&*waker);

                // BoxFuture<T> 是 Pin<Box<dyn Future<Output = T> + Send + 'static>> 的类型别名
                // 通过调用 as_mut 方法，可以将上面的类型转换成 Pin<&mut dyn Future + Send + 'static>
                // 手动调用 Future 的 poll 方法开始轮询
                // 并将 context 传入
                if future.as_mut().poll(context).is_pending() {
                    // 如果 poll 的返回值是 Poll::Pending，说明 Future 还没执行完，因此将它放回任务中，等待下次被 poll
                    *future_slot = Some(future);
                }
            }
        }
    }
}


fn main() {


    // 生成一个执行器 executor 和 Future 生成器
    let (executor, spawner) = new_executor_and_spawner();
    // 生成一个任务 Future
    // async 定义的代码块是一个 Future，也就是一个异步任务
    spawner.spawn(async {
        println!("howdy!");

        // 创建定时器 Future，并等待它完成
        TimerFuture::new(Duration::new(5, 0)).await;

        println!("done!")
    });

    // drop掉任务，这样执行器就知道任务已经完成，不会再有新的任务进来
    drop(spawner);


    // 运行执行器直到任务队列为空
    // 任务运行后，会先打印`howdy!`, 暂停2秒，接着打印 `done!`
    executor.run();

}
