use std::fs;
use std::thread;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::time::Duration;

use custom_self_multi_threading_web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9009").unwrap();
    let pool = ThreadPool::new(4);

    // 这里调用 take 方法，目的是生成一个指定数量元素的迭代器，这样就可以在收到指定数量的请求以后终止迭代
    for s in listener.incoming().take(2) {
        let stream = s.unwrap();
        // println!("request coming");

        pool.execute(|| {
            handle_request(stream);
        });
    }

    // ThreadPool 则会在main函数结束时离开作用域，并调用自己的 drop 实现
}

fn handle_request(mut stream: TcpStream) {
    // 处理 tcp 链接

    let mut buffer = [0; 1024];
    
    stream.read(&mut buffer).unwrap();

    println!("{}", String::from_utf8_lossy(&buffer));

    let reqText = String::from_utf8_lossy(&buffer[..]);

    let index_path = "GET / HTTP/1.1\r\n";
    let sleep_path = "GET /sleep HTTP/1.1\r\n";

    let (status, path) = if reqText.starts_with(index_path) {
        ("HTTP/1.1 200 OK\r\n", "./resources/index.html")
    } else if reqText.starts_with(sleep_path) {
        thread::sleep(Duration::from_millis(5000));
        ("HTTP/1.1 200 OK\r\n", "./resources/sleep.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n", "./resources/404.html")
    };

    let content = fs::read_to_string(path).unwrap();

    let length = content.len();

    let content_length_header = format!("content-length:{}\r\n\r\n", length);

    let res = format!("{}{}{}", status, content_length_header, content);

    stream.write(res.as_bytes()).unwrap();

    stream.flush();

}
