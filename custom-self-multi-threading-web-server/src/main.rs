use std::thread;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9009").unwrap();

    for s in listener.incoming() {
        let stream = s.unwrap();
        println!("request coming");
    }
}

fn handle_request(mut stream: TcpStream) {
    // 处理 tcp 链接

    let mut buffer = [0; 1024];
    
    stream.read(&mut buffer).unwrap();

    // buffer.starts_with(needle)
}
