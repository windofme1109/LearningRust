use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {

    // 多线程练习

    let join_handler = thread::spawn(|| {
        
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(100));
        }
        
        // println!("hello thread");
    });


    // 调用 join 方法，会阻塞当前的线程，直到调用 join 方法的线程执行完任务
    // 然后被阻塞的线程（这里是 main 线程）才会开始执行
    // join_handler.join();

    
    // 正常来说，主线程执行结束，就会终止其他线程，无论其他线程是否存在执行的任务
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(100));
    }
    // 调用 join 方法的位置不同，会得到不同的执行效果
    // 因为主线程会在调用 join 方法的地方阻塞
    // 在子线程结束之前，join 方法后的主线程代码不会执行
    // 子线程结束以后，主线程才会继续向下执行 
    join_handler.join();
    // 所以，在主线程的 for 循环后调用 join
    // 可以实现主线程和子线程的交叉打印
    // 等到主线程打印完成，就专注打印新线程的东西
    // 主线程只会在新线程运行结束后退出
}
