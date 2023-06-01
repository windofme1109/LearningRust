use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
pub struct ThreadPool {
    // thread 是动态数组实例，其元素的类型是：thread::JoinHandle<()>
    // 这是因为 spawn 方法的返回值就是 JoinHandle 类型
    // threads: Vec<thread::JoinHandle<()>>

    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

impl Drop for ThreadPool {

    // 为 ThreadPool 实现 Drop 方法
    // 用池中每个线程的 join 方法，从而使它们能够在关闭前完成当前正在处理的工作
    fn drop(&mut self) {
        // 首先遍历了线程池中所有的workers
        // 这里使用了 &mut，因为我们需要修改 worker 且正好 self 本身是一个可变引用
        // 针对遍历中的每一个 worker，代码会打印出信息来表明当前的 worker 正在停止运行
        // 并会接着在它的线程上调用join
        // 假如 join 调用失败，随后的 unwrap 就会触发 panic 并进入不那么优雅的关闭过程
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            // 直接调用 join 方法会报错
            // error[E0507]: cannot move out of `worker.thread` which is behind a mutable reference
            //     --> src/lib.rs:26:13
            //     |
            // 26   |             worker.thread.join().unwrap();
            //     |             ^^^^^^^^^^^^^ ------ `worker.thread` moved due to this method call
            //     |             |
            //     |             move occurs because `worker.thread` has type `JoinHandle<()>`, which does not implement the `Copy` trait
    |
            worker.thread.join().unwrap();
        }
    } 
}

// struct Job {

// }

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>

}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
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
                
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {} got a job; executing.", id);
                job();

                // 通过使用 loop 并在循环代码块内部而不是外部请求锁和任务， lock 方法中返回的 MutexGuard 会在 let job 语句结束后被立即丢弃
                // 这确保了我们只会在调用 recv 的过程中持有锁，并能够在调用 job()之前将锁释放。因此，我们的服务器才可以同时响应多个请求
            }
        });



        Worker { 
            id, 
            thread
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
        

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..size {
            // 创建线程并将其存储到动态数组
            workers.push(Worker::new(id, Arc::clone(&receiver)))
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
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}