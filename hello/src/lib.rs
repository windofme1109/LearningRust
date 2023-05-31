use std::thread;

pub struct ThreadPool {
    // thread 是动态数组实例，其元素的类型是：thread::JoinHandle<()>
    // 这是因为 spawn 方法的返回值就是 JoinHandle 类型
    // threads: Vec<thread::JoinHandle<()>>

    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

struct Job {

}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>

}

impl Worker {
    fn new(id: usize, receiver: mpsc::receiver<Job>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
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
        for id in 0..size {
            // 创建线程并将其存储到动态数组
            workers.push(Work::new(id, receiver))
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

    }
}