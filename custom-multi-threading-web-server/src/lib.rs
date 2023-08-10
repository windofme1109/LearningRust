use std::thread;
use std::sync::{mpsc, Arc, Mutex};

// 定义一个线程池
pub struct ThreadPool {
    // 存储生成的线程
    // threads: Vec<thread::JoinHandle<()>>

    // 直接持有 works 而不是一个线程
    workers: Vec<Worker>,
    // 持有通道的发送端
    sender: mpsc::Sender<Message>
}

// struct Job {

// }


// Job 现在是一个类型的别名
// 可以理解为是一个闭包的类型
type Job = Box<dyn FnOnce() + Send + 'static>;


// 作为在通道中传递的消息的内容
// Message 枚举持有两个变体：NewJob 和 Terminate
// 线程要根据这两个信号执行不同的操作
enum Message {
    // 有需要运行的 Job 闭包
    NewJob(Job),
    // 线程退出循环并停止
    Terminate
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

        // 创建一个通道
        let (sender, receiver) = mpsc::channel();
        // 使用Arc和Mutex在所有工作线程中共享通道的接收端
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            // 创建 Worker 实例并将它们存储至动态数组中
            // 将通道的接收端传递给 Worker 实例，让每个工作的线程持有 receiver
            // 使用 clone 方法克隆 Arc 来增加引用计数，从而使所有的工作线程以共享接收端的所有权
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        // ThreadPool { 
        //     threads
        // }

        ThreadPool { 
            workers,
            sender
        }
    }

   
}

impl ThreadPool {


    /// 每个线程执行的操作
    /// 
    /// exexutor 接收一个闭包作为参数
    pub fn execute<F>(&self, f: F) 
    where 
        F: FnOnce() + Send + 'static 
    {
        // 使用 Box 智能指针包裹闭包 f
        let job = Box::new(f);
        // 将 job 发送到通道中
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}


impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");
        // 向每个worker发送了 Terminate 消息
        // 这个循环保证每个线程都能收到终止信号
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        
        println!("Shutting down all workers");

        for worker in &mut self.workers  {
            println!("Shutting down worker {}", worker.id);
            // 在每个线程结束时手动调用 join 方法
            // thread 是 Option 枚举，首先使用 take 方法 Some 变体拿出来并在原来的位置留下 None 变体，这里使用 if let 进行匹配
            if let Some(thread) = worker.thread.take() {
                    thread.join().unwrap();
            }
            
        }
    }
}

struct Worker {
    // 每个 Worker 实例的标识
    id: usize,
    // 每个 Worker 持有一个 JoinHandle 实例，用来执行接收到的闭包
    // 实例使用 Option 枚举包裹 JoinHandle 实例，这样可以方便的从移动线程
    thread: Option<thread::JoinHandle<()>>
    

}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {

        // 工作的线程持有 receiver，所以要把 receiver 传递给 thread
        let thread = thread::spawn(move || {
            // 每个线程不停的查询通道的接收端，如果接收到任务，就取出任务，然后执行
            loop {
                // 这段代码首先调用了 receiver 的 lock 方法来请求互斥锁，并接着使用 unwrap 来处理可能出现的错误情形
                // 请求获取锁的操作会在互斥体被污染时出错，而互斥体会在某个持有锁的线程崩溃而锁没有被正常释放时被污染
                // 在这种情形下，调用 unwrap 触发当前线程的 panic 是非常恰当的行为
                // 当然也可以将 unwrap 修改为 expect 来附带一个有意义的错误提示信息
                // 在互斥体上得到锁以后，我们就可以通过调用 recv 来从通道中接收 Job
                // let job = receiver.lock().unwrap().recv().unwrap();
                // 调用 recv 会阻塞当前线程，当通道中不存在任务时，当前线程就会一直处于等待状态
                // 而 Mutex<T> 则保证了一次只有一个 Worker 线程尝试请求任务
                // println!("Worker {} got a job; executing.", id);

                // job();

                let message = receiver.lock().unwrap().recv().unwrap();
                


                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);
                        job();
                    },
                    Message::Terminate => {
                        // 收到终止信号时，退出循环
                        println!("Worker {} was told to terminate.", id);
                        break;
                    }
                }


            }
            
            
        });

        Worker { 
            id, 
            thread: Some(thread)
        }
    }
}