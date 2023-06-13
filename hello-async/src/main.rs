// 异步服务器
// 利用 Rust 提供的异步能力，实现一个异步服务器


extern crate core;

// use core::task;
use std::fs;
use std::io::prelude::*;
// use std::net::TcpListener;
// use std::net::TcpStream;
use std::time::Duration;
use std::thread::sleep;


use async_std::prelude::*;
use async_std::task;

// 非阻塞的 TcpListener
use async_std::net::TcpListener;
use async_std::net::TcpStream;
// //
use async_std::task::spawn;
//
use futures::stream::StreamExt;


#[async_std::main]
async fn main() {
    // 监听本地端口 7878 ，等待 TCP 连接的建立
    // 使用 async_std 提供的 TcpListener 绑定 ip 和端口
    // let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // 阻塞等待请求的进入
    // 因为 incoming 函数返回的时要给迭代器，这个迭代器是阻塞的
    // 在 listener 等待其他连接的时候，执行器不能运行其他任务
    // 而且下一个任务必须等待前一个任务完成才能继续迭代
    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();
    //     // 这里无法实现并发
    //     handle_connection(stream).await;
    // }

    // 为了解决不能并发处理请求的问题
    // 我们这里使用异步流（stream）来解决这个问题

    // 使用 async_std 提供的 TcpListener 创建 Tcp 监听器
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();

    // listener.incoming() 从一个阻塞的迭代器变成一个非阻塞的 Stream
    listener
        .incoming()
        .for_each_concurrent(/* limit */ None, |tcpstream| async move {

            let stream = tcpstream.unwrap();
            println!("connection establish");

            // handle_connection(stream).await;

            // async 并发和多线程其实并不冲突，而 async-std 包也允许我们使用多个线程去处理
            // 由于 handle_connection 实现了 Send 特征且不会阻塞，因此使用 async_std::task::spawn 是非常安全的
            spawn(handle_connection(stream));
        })
        .await;

    // 异步版本的 TcpListener 为 listener.incoming() 实现了 Stream trait，以上修改有两个好处:
    // listener.incoming() 不再阻塞
    // 使用 for_each_concurrent 并发地处理从 Stream 获取的元素

}


// fn handle_connection(mut stream: TcpStream) {
//     // 从连接中顺序读取 1024 字节数据
//     let mut buffer = [0; 1024];
//     stream.read(&mut buffer).unwrap();
//
//     let get = b"GET / HTTP/1.1\r\n";
//
//
//     // 处理HTTP协议头，若不符合则返回404和对应的 `html` 文件
//     let (status_line, filename) = if buffer.starts_with(get) {
//         ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
//     } else {
//         ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
//     };
//     let contents = fs::read_to_string(filename).unwrap();
//
//     // 将回复内容写入连接缓存中
//     let response = format!("{status_line}{contents}");
//     stream.write_all(response.as_bytes()).unwrap();
//     // 使用 flush 将缓存中的内容发送到客户端
//     stream.flush().unwrap();
// }

/// 为了实现异步能力，将处理连接的函数变成一个异步函数
///
async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    // stream.read(&mut buffer).unwrap();

    // 因为这里的 stream 是 async_std 中提供的 TcpStream
    // 所以 read 方法是一个异步函数，所以必须调用 .await
    stream.read(&mut buffer).await.unwrap();


    let basic_path = b"GET / HTTP/1.1\r\n";
    let sleep_path = b"GET /sleep HTTP/1.1\r\n";

    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let (status_line, filename) = if buffer.starts_with(basic_path) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep_path) {

        // async_std 中的 task 模块中的 sleep 方法，不会阻塞线程
        // 这是一个异步函数，使用时需要加上 .await
       task::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{status_line}{contents}");

    // stream.write(response.as_bytes()).unwrap();

    // write 方法也是异步方法
    stream.write(response.as_bytes()).await.unwrap();

    // stream.flush().unwrap();

    // flush 方法也是异步方法
    stream.flush().await.unwrap();
}
