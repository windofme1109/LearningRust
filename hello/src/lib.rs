use std::sync::mpsc;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;


pub struct ThreadPool {
    // thread 是动态数组实例，其元素的类型是：thread::JoinHandle<()>
    // 这是因为 spawn 方法的返回值就是 JoinHandle 类型
    // threads: Vec<thread::JoinHandle<()>>

    // 定义一个 workers 属性
    // 用来持有 Worker 类型的动态数组
    workers: Vec<Worker>,
    // 定义一个 sender 属性，用来持有 channel 的发送端
    // 作用是向线程中发送需要执行的代码
    // sender: mpsc::Sender<Job>

    // 将 Job 类型修改为 Message
    sender: mpsc::Sender<Message>
}

// 定义一个枚举
enum Message {
    // 需要运行 Job 的 NewJob 变体
    NewJob(Job),
    // 线程退出循环并停止的 Terminate 变体
    Terminate
}

// struct Job {

// }

// Job 类型作为 Box 指针的类型别名，用来持有闭包
type Job = Box<dyn FnOnce() + Send + 'static>;

impl Drop for ThreadPool {


    // 为 ThreadPool 实现 Drop 方法
    // 用池中每个线程的 join 方法，从而使它们能够在关闭前完成当前正在处理的工作
    fn drop(&mut self) {


        println!("Sending terminate message to all workers.");

        // 第一次循环次向每个 worker 发送了 Terminate 消息
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        // 首先遍历了线程池中所有的workers
        // 这里使用了 &mut，因为我们需要修改 worker 且正好 self 本身是一个可变引用
        // 针对遍历中的每一个 worker，代码会打印出信息来表明当前的 worker 正在停止运行
        // 并会接着在它的线程上调用join
        // 假如 join 调用失败，随后的 unwrap 就会触发 panic 并进入不那么优雅的关闭过程
        //
        // 第二次则在每个 worker 的线程上调用了 join
        // 如果我们尝试在同一个循环中发送消息并立即调用 join，那么就无法保证当前正在迭代的 worker 就是从通道中获得消息的那一个
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            // 直接调用 join 方法会报错
            // error[E0507]: cannot move out of `worker.thread` which is behind a mutable reference
            //     --> src/lib.rs:26:13
            //     |
            // 26  |             worker.thread.join().unwrap();
            //     |             ^^^^^^^^^^^^^ ------ `worker.thread` moved due to this method call
            //     |             |
            //     |             move occurs because `worker.thread` has type `JoinHandle<()>`, which does not implement the `Copy` trait

            // 这个错误意味着我们不能调用 join，因为当前的代码仅仅是可变借用了worker
            // 而 join 方法则要求取得其参数的所有权。为了解决这一问题，我们需要把线程移出拥有其所有权的 Worker 实例，以便 join 可以消耗掉它
            // 如果 Worker 持有的是一个 Option<thread::JoinHandle<()>>
            // 那么我们就可以在 Option 上调用 take 方法来将 Some 变体的值移出来
            // 并在原来的位置留下 None 变体
            // 换句话说，正在运行中的 Worker 会在 thread 中持有一个 Some 变体
            // 当我们希望清理 Worker 时，就可以使用 None 来替换掉 Some，从而使 Worker 失去可以运行的线程
            // worker.thread.join().unwrap();


            // 为 Option 值调用 take 方法会将 Some 变体的值移出并在原来的位置留下None变体
            // 使用了 if let 来解构 Some 从而得到线程，并接着在这个线程上调用了 join
            // 当某个 Worker 的线程值是 None 时，我们就知道 worker 已经清理了这个线程而无须进行任何操作
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }

        // 为了更好地理解为何需要两个循环，你可以想象一下拥有两个worker 的场景
        // 如果我们使用单个循环来迭代所有的 worker，那么在进行首次迭代时，代码会将一个结束信息发送到通道中后接着在第一个 worker 线程上调用join
        // 假设这个 worker 正好忙于处理其他请求，那么第二个 worker 就会从通道中获取这个结束信息并退出自己的循环
        // 由于结束信号被第二个线程截取了，所以我们等待的第一个worker线程永远不会停止。一次死锁事件发生了！

        // 为了阻止这种情况的发生，我们首先用一个循环把全部 Terminate 消息发送到通道中，随后再到另一个循环中等待所有的进程结束
        // 由于 worker 会在收到结束信号后停止接收请求，所以只要我们在调用 join 之前发送了与 workers 数目相等的结束消息
        // 就可以确保每一个  worker 都能够收到自己的结束信号
    }
}


// 为什么需要一个单独的 Worker 类型呢
// 因为按照我们之前的设定，我们使用 thread::spawn 创建线程，spawn 方法接收一个闭包作为参数
// spawn 方法在创建完线程以后会立即执行自己接收到的代码参数
// 但是实际上，我们创建完线程以后，不需要立即执行，而是进入等待状态并执行随后传给它的代码

// 因此我们需要创建 Worker 类，作为中间类型
// Worker 类型持有线程（JoinHandle）

// Worker 类型持有一个 id 和 thread
// thread 属性用来持有 JoinHandle，也就是一个线程
struct Worker {
    id: usize,
    // thread: thread::JoinHandle<()>
    thread: Option<thread::JoinHandle<()>>

}

impl Worker {
    // fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    // 将 receiver 的类型中的 Job 修改为 Message
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {

                // 首先调用了 receiver 的 lock 方法来请求互斥锁
                // 并接着使用 unwrap 来处理可能出现的错误情形
                // 请求获取锁的操作会在互斥体被污染时出错，而互斥体会在某个持有锁的线程崩溃而锁没有被正常释放时被污染
                // 在这种情形下，调用 unwrap 触发当前线程的 panic 是非常恰当的行为
                // 当然，你也可以将 unwrap 修改为 expect 来附带一个有意义的错误提示信息

                // 在互斥体上得到锁以后，我们就可以通过调用 recv 来从通道中接收 Job 了
                // 与发送端的 send 方法类似，recv会在持有通道发送端的线程关闭时出现错误，所以我们同样使用了 unwrap 来拦截所有错误
                // 调用 recv 会阻塞当前线程，当通道中不存在任务时，当前线程就会一直处于等待状态。而 Mutex<T> 则保证了一次只有一个 Worker 线程尝试请求任务

                // let job = receiver.lock().unwrap().recv().unwrap();
                let message = receiver.lock().unwrap().recv().unwrap();

                // 因为 execute 方法会发送包裹着任务的 Message:NewJob 变体
                // Worker::new 中的代码会从通道中接收并处理 Message，
                // 并在收到 NewJob 变体时处理任务，在收到 Terminate 变体时退出循环
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);
                        job();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        break;
                    }
                }

                // println!("Worker {} got a job; executing.", id);
                // job();

                // 通过使用 loop 并在循环代码块内部而不是外部请求锁和任务， lock 方法中返回的 MutexGuard 会在 let job 语句结束后被立即丢弃
                // 这确保了我们只会在调用 recv 的过程中持有锁，并能够在调用 job()之前将锁释放。因此，我们的服务器才可以同时响应多个请求
            }
        });

         Worker {
             id,
             thread: Some(thread)
         }

    }
}

impl ThreadPool {
    ///创建线程池。
    /// 
    /// 线程池中线程的数量
    /// 
    /// # panic
    /// 
    /// `new` 函数会在 size 为 0 时触发 panic。
    pub fn new(size: usize) -> ThreadPool {
    // 负的线程数量没有任何意义，因此这里选择了 usize 作为 size 参数的类型
        
        // 使用 assert! 宏进行断言，当 size 为 0 时，触发程序的 panic
        // 我们也可以在 new 函数中返回一个 Result 而不再使用 assert! 宏
        assert!(size > 0);

        // with_capacity 方法创建一个指定长度的数组
        // with_capacity 与 Vec::new 有些类似，但区别在于 with_capacity 会为动态数组预分配出指定的空间
        // 在知晓存储大小的前提下预先分配存储空 间要比使用 Vec::new 在插入时动态扩展大小更有效率一些
        // let mut threads = Vec::with_capacity(size);
        let mut workers = Vec::with_capacity(size);
        
        // 我们希望刚刚创建的Worker结构体能够从存储在ThreadPool的队列中获取需要执行的代码，并将它们发送到线程中运行
        // 因此我们使用 channel 作为线程通信的方式
        // 将我们需要线程执行的代码发送给线程，也就是 Worker 实例
        // 创建一个 channel，并持有发送端 sender
        let (sender, receiver) = mpsc::channel();
//
        // 为了在多个线程中共享所有权并允许线程修改共享值，我们可以使用 Arc<Mutex<T>>
        // Arc 类型允许多个工作线程拥有同一个接收者，而 Mutex 则保证了一次只有一个工作线程能够从接收端得到任务
        let receiver = Arc::new(Mutex::new(receiver));


        for id in 0..size {
            // 创建线程并将其存储到动态数组

            // 生成的每个 Worker 实例都会持有 channel 的接收端 receiver
            // 如果我们将 receiver 直接传递给多个 Worker 实例，那么编译会失败：
            // error[E0382]: use of moved value: `receiver`
            // 直接传递给线程，会发生所有权的转移，因此，多个线程无法直接使用 receiver
            // 因为 Rust 提供的 channel 是多生产者、单消费者的，这也意味着你不能简单地通过克隆接收端来解决上述问题
            // 我们希望所有的线程都使用这一个消费者从而能够在线程间分发任务
            // workers.push(Worker::new(id, receiver))

            // 创建新的 Worker 时克隆Arc来增加引用计数
            // 从而使所有的工作线程可以共享接收端的所有权
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            // threads
            workers,
            sender
        }

    }
}

impl ThreadPool {
    // 仿照 Thread::spawn 方法的签名构造 execute 方法
    // pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    // where
    // F: FnOnce() -> T + Send + 'static,
    // T: Send + 'static
    // 因为 execute 接收的闭包最终要传入 spawn 方法中
    // 而请求处理的线程只会执行一次闭包，所以这里就是使用 FnOnce 这个 trait 约束闭包的类型
    // 同时类型参数 F 还需要满足 Send trait 和 生命周期 'static
    // 因为满足 Send trait 约束的闭包才可以从一个线程传递到另一个线程
    // 而由于我们不知道线程究竟会执行多长时间，所以闭包必须是 'static 的
    // FnOnce 后的 () 意味着传入的闭包既没有参数，也不返回结果
    // 就像函数定义一样，我们可以省略签名中的返回值，但却不能省略函数名后的圆括号，即便括号中没有任何参数
    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send +'static {

        // 接收到闭包后会创建出一个新的Job实例，使用 Box 指针包裹一个函数类型
        let job = Box::new(f);
        // 将这个任务传递给通道的发送端，为了应对发送失败的情形，我们在 send 后直接调用了 unwrap
        // self.sender.send(job).unwrap();

        // 通过通道发送的不再是 job 实例
        // 而是发送 Message 枚举的 NewJob 变体
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}