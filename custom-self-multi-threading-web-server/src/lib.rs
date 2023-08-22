// 定义公共的线程池
use std::thread;
use std::sync::{Arc, mpsc, Mutex};


pub struct ThreadPool {
    workers: Vec<Worker>,
    // 通道的发送端，发送的类型是 Job
    sender: mpsc::Sender<Job>
}

type Job = Box< dyn FnOnce() + Send + 'static>;

// Worker 持有真正的线程
struct Worker {
    id: usize,
    //
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        println!("id {}", id);
        let thread = thread::spawn(move || {
            loop {
                //
                // let rx = receiver.lock().unwrap();
                // 拿到 job
                // let job = rx.recv().unwrap();

                // 尝试获取锁
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("id {} thread received a job, executing", id);

                // 在线程中执行闭包
                job();
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
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..count {

            let worker = Worker::new(id, Arc::clone(&receiver));


            workers.push(worker);
        }

        ThreadPool {
            workers: workers,
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
        let executeFn = Box::new(f);

        self.sender.send(executeFn).unwrap();
    }
}
