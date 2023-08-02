// 简单的web server

use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9090").unwrap();

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
    let get_method = b"GET / HTTP/1.1\r\n";
    // 这里可以打印日志
    if buffer.starts_with(get_method) {
        // 拼接 http 响应头
        let res = format!("HTTP/1.1 200 OK\r\n\r\n{}", 100);
        println!("response");
        stream.write(res.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
