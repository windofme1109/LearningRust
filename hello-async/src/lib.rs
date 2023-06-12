// 测试 handle_connection 函数
// 对于测试 Web 服务器，使用集成测试往往是最简单的，但是在本例子中，将使用单元测试来测试连接处理函数的正确性
//
// 为了保证单元测试的隔离性和确定性，我们使用 MockTcpStream 来替代 TcpStream
// 首先，修改 handle_connection 的函数签名让测试更简单
// 之所以可以修改签名，原因在于 async_std::net::TcpStream 实际上并不是必须的
// 只要任何结构体实现了 async_std::io::Read, async_std::io::Write 和 marker::Unpin 就可以替代它

// use super::*;
use futures::io::Error;
use futures::task::{Context, Poll};
use async_std::io::Write;
use async_std::io::Read;

use std::cmp::min;
use std::pin::Pin;

struct MockTcpStream {
    read_data: Vec<u8>,
    write_data: Vec<u8>,
}

// 为了保证单元测试的隔离性和确定性，我们使用 MockTcpStream 来替代 TcpStream
// 首先，修改 handle_connection 的函数签名让测试更简单，之所以可以修改签名，原因在于 async_std::net::TcpStream 实际上并不是必须的
// 只要任何结构体实现了 async_std::io::Read, async_std::io::Write 和 marker::Unpin 就可以替代它

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
    fn poll_write(
        mut self: Pin<&mut Self>,
        _: &mut Context,
        buf: &[u8],
    ) -> Poll<Result<usize, Error>> {
        self.write_data = Vec::from(buf);

        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Error>> {
        Poll::Ready(Ok(()))
    }
}

use std::marker::Unpin;
impl Unpin for MockTcpStream {}

use std::fs;


// 在使用初始化数据设置好 MockTcpStream 后，我们可以使用 #[async_std::test] 来运行 handle_connection 函数，该函数跟 #[async_std::main] 的作用类似
// 为了确保 handle_connection 函数正确工作，需要根据初始化数据检查正确的数据被写入到 MockTcpStream 中
#[async_std::test]
async fn test_handle_connection() {
    let input_bytes = b"GET / HTTP/1.1\r\n";
    let mut contents = vec![0u8; 1024];
    contents[..input_bytes.len()].clone_from_slice(input_bytes);
    let mut stream = MockTcpStream {
        read_data: contents,
        write_data: Vec::new(),
    };

    handle_connection(&mut stream).await;
    let mut buf = [0u8; 1024];
    stream.read(&mut buf).await.unwrap();

    let expected_contents = fs::read_to_string("hello.html").unwrap();
    let expected_response = format!("HTTP/1.1 200 OK\r\n\r\n{}", expected_contents);
    assert!(stream.write_data.starts_with(expected_response.as_bytes()));
}