// 定义公共的线程池
use std::thread;
use std::sync::{Arc, mpsc, Mutex};

// 定义这个枚举的作用是：
// Message 作为在通道中发送的消息的主体，用来取代 Job
// 在线程中，根据 Message 的不同变体，从而执行不同的操作
enum Message {
    NewJob(Job),
    Terminated
}


pub struct ThreadPool {
    workers: Vec<Worker>,
    // 通道的发送端，发送的类型是 Job
    sender: mpsc::Sender<Message>
}

type Job = Box< dyn FnOnce() + Send + 'static>;

// Worker 持有真正的线程
struct Worker {
    id: usize,
    //
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        println!("id {}", id);
        let thread = thread::spawn(move || {
            loop {
                // 下面这种先获取锁，再从锁中获取信息（通过 recv 方法实现）
                // 存在一个问题：就是所有的请求，只会由第一个线程来处理
                // 原因可能是：我们首先获取了锁：rx，然后不断轮询，就会导致第一个线程不断的持有锁
                // 进而从锁中获取收到的闭包，然后执行

                // 而实际上，整体的过程应该是：拿到锁 -> 从锁中获取信息 -> 释放锁
                // 这样其他线程才有机会拿到锁

                // let rx = receiver.lock().unwrap();
                // 拿到 job
                // let job = rx.recv().unwrap();

                // 尝试获取锁，下面这种链式调用，既可以实现上述的 “拿到锁 -> 从锁中获取信息 -> 释放锁” 的过程
                // let job = receiver.lock().unwrap().recv().unwrap();

                // println!("id {} thread received a job, executing", id);

                // // 在线程中执行闭包
                // job();

                let msg = receiver.lock().unwrap().recv().unwrap();

                match msg {
                    Message::NewJob(job) => {
                        // 接收到闭包，开始执行
                        job();
                    },
                    Message::Terminated => {
                        // 收到终止信号，就跳出循环
                        break;
                    }
                }

            }
        });

        Worker {
            id,
            thread,
        }

    }
}


impl ThreadPool {
    pub fn new(count: usize) -> ThreadPool {
        
        assert!(count > 0);
        
        let mut workers = Vec::with_capacity(count);
        println!("count: {}", count);
        // 通道的接收端和发送端持有的类型必须相同
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..count {

            let worker = Worker::new(id, Arc::clone(&receiver));


            workers.push(worker);
        }

        ThreadPool {
            workers,
            sender
        }

    }
}


impl ThreadPool {
    /// 接收一个闭包，并将其发到某个线程上执行执行
    ///
    ///
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
        // 使用通道将闭包 f 发送到线程上，因此这里需要使用智能指针进行包裹
        // 使用 Box 类型包裹 f
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

// 为 ThreadPool 实现 Drop trait
// 当 ThreadPool 实例离开作用域时，会执行一些操作
// 我们这里执行的是清理线程的操作
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");
        // 先发送一遍终止信号
    }
}
