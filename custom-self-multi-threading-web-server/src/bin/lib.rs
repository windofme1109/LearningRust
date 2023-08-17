// 定义公共的线程池
use std::thread::JoinHandle;


pub struct ThreadPool {
    // threads: Vec<>
}

impl ThreadPool {
    pub fn new(count: usize) {
        
        assert!(count > 0);
        
        let threadList = Vec::with_capacity(count);

        for id in 0..count {
            threadList.push();
        }

    }
}
