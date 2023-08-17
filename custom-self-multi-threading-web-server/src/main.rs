use std::fs;
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
        // println!("request coming");
        
        handle_request(stream);
    }
}

fn handle_request(mut stream: TcpStream) {
    // 处理 tcp 链接

    let mut buffer = [0; 1024];
    
    stream.read(&mut buffer).unwrap();

    // let requestMethod = b"";

    let reqText = String::from_utf8_lossy(&buffer[..]);
    // println!("requset: {}", reqText);

    let indexPath = "GET / HTTP/1.1\r\n";

    let (status, path) = if reqText.starts_with(indexPath) {
        ("HTTP/1.1 200 OK\r\n", "./resources/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n", "./resources/404.html")
    };

    let content = fs::read_to_string(path).unwrap();

    let length = content.len();

    let contentLengthHeader = format!("content-length:{}\r\n\r\n", content);

    let res = format!("{}{}{}", status, contentLengthHeader, content);

    stream.write(res.as_bytes()).unwrap();

    stream.flush();

}
