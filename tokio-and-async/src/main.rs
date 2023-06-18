use std::thread::sleep;
use std::time::Duration;

fn main() {

    // 在一个线程上调用两个会阻塞线程的函数
    // 函数会顺序执行，因为是在一个线程上执行，所以后一个任务会等前一个任务执行完成再执行
    println!("Hello before reading file!");
    let file1_content = read_from_file1();
    println!("{:?}", file1_content);
    println!("Hello after reading file1!");
    let file2_content = read_from_file2();
    println!("{:?}", file2_content);
    println!("Hello after reading file2!");
}

// 模拟异步读取文件的函数
// 阻塞线程 4 秒钟，然后返回值
fn read_from_file1() -> String {
    sleep(Duration::new(4, 0));
    String::from("Hello, there from file 1")
}

// 模拟异步读取文件的函数
// 阻塞线程 2 秒钟，然后返回值
fn read_from_file2() -> String {
    sleep(Duration::new(2, 0));
    String::from("Hello, there from file 2")
}
