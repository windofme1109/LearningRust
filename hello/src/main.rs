// 构建 Web 服务器
// 多线程服务器

use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use std::time::Duration;
use std::thread;

fn main() {

    let linstener = TcpListener::bind("127.0.0.1:7878").unwrap();

   
    for stream in linstener.incoming() {
        
        let stream = stream.unwrap();
        // println!("Connection established");

        // 从 tcp 流中读取数据
        handle_connection(stream);
    }
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
        thread::sleep(Duration::from_secs(5));
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
   
