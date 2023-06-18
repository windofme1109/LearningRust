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
       task::sleep(Duration::from_secs(10)).await;
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




#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use futures::io::Error;
    use futures::task::{Context, Poll};
    use async_std::io::Write;
    use async_std::io::Read;
    use async_std::task;

    use std::cmp::min;
    use std::pin::Pin;

    /// 为了便于测试，修改 handle_connection 的函数签名
    /// 之所以可以修改签名，原因在于 async_std::net::TcpStream 实际上并不是必须的
    /// 只要任何结构体实现了 async_std::io::Read, async_std::io::Write 和 marker::Unpin 就可以替代它
    async fn handle_connection(mut stream:  impl Read + Write + Unpin) {
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
            task::sleep(Duration::from_secs(10)).await;
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
    struct MockTcpStream {
        read_data: Vec<u8>,
        write_data: Vec<u8>,
    }

    // 下面，来构建一个 mock 的 TcpStream 并实现了上面这些 trait， 它包含一些数据
// 这些数据将被拷贝到 read 缓存中, 然后返回 Poll::Ready 说明 read 已经结束
    impl Read for MockTcpStream {
        fn poll_read(
            self: Pin<&mut Self>,
            _: &mut Context,
            buf: &mut [u8],
        ) -> Poll<Result<usize, Error>> {
            let size: usize = min(self.read_data.len(), buf.len());
            buf[..size].copy_from_slice(&self.read_data[..size]);
            Poll::Ready(Ok(size))
        }
    }


    impl Write for MockTcpStream {
        // 该方法作用是：复制任何数到 mock 的 TcpStream
        // 复制完成后，返回 Poll::Ready
        fn poll_write(
            mut self: Pin<&mut Self>,
            _: &mut Context,
            buf: &[u8],
        ) -> Poll<Result<usize, Error>> {
            self.write_data = Vec::from(buf);

            Poll::Ready(Ok(buf.len()))
        }

        // 因为我们这里不需要使用刷新（flush）或者关闭（close）mock 的 TcpStream
        // poll_flush 和 poll_close 仅仅返回 Poll::Ready 即可

        fn poll_flush(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Error>> {
            Poll::Ready(Ok(()))
        }

        fn poll_close(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Error>> {
            Poll::Ready(Ok(()))
        }
    }

    use std::marker::Unpin;

    // mock 的 TcpStream 需要实现 Unpin trait
// 表示其中可以安全地在内存中移动
    impl Unpin for MockTcpStream {}

    use std::fs;


// 在使用初始化数据设置好 MockTcpStream 后
// 我们可以使用 #[async_std::test] 来运行 handle_connection 函数
// 该函数跟 #[async_std::main] 的作用类似

    // 为了确保 handle_connection 函数正确工作
    // 需要根据初始化数据检查正确的数据被写入到 MockTcpStream 中
    #[async_std::test]
    async fn test_handle_connection() {
        let input_bytes = b"GET / HTTP/1.1\r\n";

        // 定义一个缓冲区
        let mut contents = vec![0u8; 1024];
        // clone_from_slice 方法将接收的源切片克隆到自己内部
        // 因此 contents 0 - input_bytes.len() 的位置会是 input_str 内容
        contents[..input_bytes.len()].clone_from_slice(input_bytes);

        let mut stream = MockTcpStream {
            read_data: contents,
            write_data: Vec::new(),
        };


        // 将 mock 的 tcp 流传入 handle_connection 函数中进行处理
        handle_connection(&mut stream).await;

        // 经过 handle_connection 函数的处理
        // 因为我们模拟的连接的请求行是：GET / HTTP/1.1
        // 因此经过 handle_connection 函数的处理，其应该是将 hello.html 内容写入 tcp 流中
        // 最终的响应体是由 HTTP/1.1 200 OK\r\n\r\n 和 hello.html 中的内容组成
        let expected_contents = fs::read_to_string("hello.html").unwrap();

        let expected_response = format!("HTTP/1.1 200 OK\r\n\r\n{}", expected_contents);

        assert!(stream.write_data.starts_with(expected_response.as_bytes()));
    }
}
