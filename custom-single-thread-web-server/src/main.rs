// 简单的web server

use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use std::time::Duration;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9009").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("stream {:?}", stream);
        handle_connection(stream);
    }

}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    
    stream.read(&mut buffer).unwrap();
    // println!("buffer {:?}", buffer);
    let get_method_index_path = b"GET / HTTP/1.1\r\n";
    let get_method_sleep_path = b"GET /sleep HTTP/1.1\r\n";

    // 这里可以打印日志
    let (status_line, file_path) = if buffer.starts_with(get_method_index_path) {
        ("HTTP/1.1 200 OK\r\n", "./resources/index.html")
    } else if buffer.starts_with(get_method_sleep_path) {
        thread::sleep(Duration::from_secs(8));
        ("HTTP/1.1 200 OK\r\n", "./resources/sleep.html")
    } else { 
        ("HTTP/1.1 404 NOT FOUND\r\n", "./resources/404.html")
    };

    let contents = fs::read_to_string(file_path).unwrap();
        
    let headers = format!("content-type:text/html;charset=utf-8\r\ncontent-length: {}\r\n\r\n", contents.len());
    
    // 拼接 http 响应头
    let res = format!("{}{}{}", status_line, headers, contents);
    // println!("response {}", res);
    stream.write(res.as_bytes()).unwrap();
    stream.flush().unwrap();
}
