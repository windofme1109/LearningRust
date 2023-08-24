// 多线程服务器

use std::net::TcpListener;
use std::net::TcpStream;

use std::thread;
use std::time::Duration;
use std::fs;
use std::io::prelude::*;

use custom_multi_threading_web_server::ThreadPool;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:9009").unwrap();

    let pool = ThreadPool::new(4);

    // take 方法接收一个整数，用来限制生成的迭代器的元素数量
    // 这里我们限制只能接收两个请求，也就是说，接收完两个请求，就会退出当前的循环
    for stream in listener.incoming().take(2) {
        let s = stream.unwrap();
        pool.execute(|| {
            handle_connection(s);
        });
        
    }

    println!("Shutting Down.");

}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer= [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    let get_method_index_path = b"GET / HTTP/1.1\r\n";
    let get_method_sleep_path = b"GET /sleep HTTP/1.1\r\n";

    let (status, file_path) = if buffer.starts_with(get_method_index_path) {
        ("HTTP/1.1 200 OK\r\n", "./resources/index.html")
    } else if buffer.starts_with(get_method_sleep_path) {
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK\r\n", "./resources/sleep.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n", "./resources/404.html")
    };

    let contents = fs::read_to_string(file_path).unwrap();

    let headers = format!("content-type:text/html;charset=utf-8\r\ncontent-length: {}\r\n\r\n", contents.len());

    let res = format!("{}{}{}", status, headers, contents);

    stream.write(res.as_bytes()).unwrap();

    stream.flush().unwrap();

}   
