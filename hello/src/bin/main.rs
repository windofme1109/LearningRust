// 构建 Web 服务器

// 多线程服务器

// 目前 main 文件 在 src 的 bin 目录下，所以，当前 hello 目录中的主包就是代码包（lib.rs），而不是二进制包

use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use std::time::Duration;
use std::thread;

use hello::ThreadPool;

// 添加线程池

// 线程池(thread pool)是一组预先分配出来的线程，它们被用于等待并随时处理可能的任务
// 当程序接到一个新任务时，它会将线程池中的一个线程分配给这个任务，并让该线程处理这个任务
// 线程池中其余可用的线程能够在第一个线程处理任务时接收其他到来的任务
// 当第一个线程处理完它的任务后，我们会将它放回线程池，并使 其变为空闲状态以准备处理新的任务
// 一个线程池允许你并发地处理 连接，从而增加服务器的吞吐量。

// 不同于无限制地创建线程，线程池中只会有固定数量的等待线程
// 新连接进来的请求会被发送至线程池中处理，而线程池则维护了一个接收请求的队列
// 池中可用的线程会从这个请求队列中取出请求并处理，然后再向队列索要下一个请求
// 基于这种设计，我们可以同 时处理 N 个请求，这里的 N 也就是线程数量
// 当所有的线程都在处理慢请求时，随后的请求依然会被阻塞在等待队列中
// 虽然没能完全避免阻塞的出现，但我们增加了可同时处理的慢请求数量

fn main() {

    let linstener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // 创建一个可以配置线程数量的线程池，将线程池中，线程的数量配置为 4
    let pool = ThreadPool::new(4);

    // 测试我们的服务器是否会在接收两个请求以后就会停机
    // 定义在 Iterator trait 中的 take 方法限制了我们的迭代过程最多只会进行两次
    // 而 ThreadPool 则会在 main 函数结束时离开作用域，并调用自己的 drop 实现
    for stream in linstener.incoming().take(2) {
        let stream = stream.unwrap();
        // println!("connected");
            println!("Connection established");
        // 为每个连接都创建一个线程去处理
        // execute 方法接收一个闭包，并将它分配给线程池中的线程去执行
        pool.execute(|| {
            // handle_connection(stream);
            
            
            // 从 tcp 流中读取数据
            handle_connection(stream);
        });
        
    }

    println!("iteration completed");
}


fn handle_connection(mut stream: TcpStream) {

    // 在栈上声明一个用于存放数据的buffer，大小是 512 个字节
    // [0; 512] 用来声明一个固定大小的数组
    let mut buffer = [0; 512];
    
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let sleep = b"GET /sleep HTTP/1.1\r\n";
    // 使用模式匹配优化代码实现
    // 只保留两个不同模式下的差异部分，即状态行和要读取的文件的名称
    let (status_line, filename) = if buffer.starts_with(get) {
    
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        // 如果请求路径是 /sleep，那么我们将程序休眠 5 秒钟，然后再返回响应成功时的 html 内容
        // 一个请求是：127.0.0.1:7878 ，另一个请求是：127.0.0.1:7878/sleep
        // 如果我们和之前一样反复地输入 /URI，那么应该会非常迅速地获得响应结果
        // 但如果你在加载 / 页面之前加载了 /sleep，那么你就会观察到/需要花费至少5秒钟才能渲染出成功响应的HTML页面

        // 因为我们只有一个线程，这个线程需要依次处理请求，如果前一个请求花费时间比较长，就会阻塞随后的请求队列
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    // 公共的部分
    let contents = fs::read_to_string(filename).unwrap();
    // format! 格式化给定的字符串，将 html 内容与请求行拼接成完整的 http 响应
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    
}
   
