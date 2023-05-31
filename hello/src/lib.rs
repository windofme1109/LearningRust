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
    sender: mpsc::Sender<Job>
}

// Job 类型作为 Box 指针的类型别名，用来持有闭包
type Job = Box<dyn FnOnce() + Send + 'static>;

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
    thread: thread::JoinHandle<()>

}

impl Worker {

    // new 函数接收 id 参数和 receiver
    // receiver
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| {
            // receiver;
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {} got a job; executing.", id);
                job();
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
        self.sender.send(job).unwrap();
    }
}