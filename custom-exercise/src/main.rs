use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {

    // 多线程练习

    let join_handler = thread::spawn(|| {
        
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
        
        // println!("hello thread");
    });

    // join_handler.join();
    
    // 正常来说，主线程执行结束
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
