use std::thread;


// 定义一个线程池
pub struct ThreadPool {
    // 存储生成的线程
    // threads: Vec<thread::JoinHandle<()>>
    workers: Vec<Worker>
}

impl ThreadPool {
    /// 创建线程池
    /// 
    /// 线程池中线程的数量
    /// 
    /// # panic
    /// 
    /// `new` 函数会在 size 为 0 的时候触发 panic
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for _ in 0..size {
            // 创建线程并将它们存储至动态数组中

        }

        // ThreadPool { 
        //     threads
        // }

        ThreadPool { 
            workers
        }
    }

   
}

impl ThreadPool {

    pub fn execute<F>(&self, f: F) 
    where 
        F: FnOnce() + Send + 'static 
    {

    }
}

struct Worker {
    // 每个 Worker 实例的标识
    id: usize,
    // 每个 Worker 持有一个 JoinHandle 实例，用来执行接收到的闭包
    thread: thread::JoinHandle<()>

}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker { 
            id, 
            thread
        }
    }
}