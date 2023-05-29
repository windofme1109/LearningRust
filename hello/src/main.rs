// 构建 Web 服务器
// 
// Web服务器涉及的两个主要协议分别是超文本传输协议(HTTP)和 传输控制协议(TCP)
// 它们两者都是基于请求-响应(request- response)的协议
// 也就是说，这个协议由客户端发起请求，再由服务器监听并响应客户端。请求和响应的内容会由协议本身定义

// TCP是一种底层协议，它描述了信息如何从一个服务器传送到另外一个服务器的细节，但它并不指定信息的具体内容
// HTTP 协议建立在 TCP 之上，它定义了请求和响应的内容
// 从技术上来说，基于其他底层协议使用HTTP也是可以的
// 但在绝大多数情况下，HTTP都是通过TCP发 送数据的。我们将会处理TCP中的原始字节并与HTTP请求及响应打交道


use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;


fn main() {
    // TcpListener 用来建立一个 TCP socket 服务器，监听 TCP 连接
    // bind 方法用来创建一个新的 TcpListener，并将 TcpListener 绑定到指定的地址和端口上：127.0.0.1:7878
    // 返回的监听器（listener）则准备好接受 TCP 连接
    // 之所以选择bind作为函数的名称是因为在网络领域中，连接到端口这一行为也被称作“绑定到端口”(binding to a port)
    // bind函数的返回值类型 Result<T, E> 意味着绑定操作是有可能失败的
    // 比如，连接到端口 80 需要管理员权限(非管理员只能监听大于 1024 的端口)，
    // 当我们以非管理员身份尝试连接到 80 端口时就会被系统拒绝从而失败
    // 另外，假如我们运行了 2 个监听到同一地址上的程序实例，那么绑定也不会成功
    let linstener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // TcpListener 上的 incoming 方法会返回一个产生流序列的迭代器，更准确地说，是TcpStream 类型的流
    // 单个流 (stream) 代表了一个在客户端和服务器之间打开的连接
    // 而连接 (connection) 则代表了客户端连接服务器、服务器生成响应，以及服务器关闭连接的全部请求与响应过程
    // 为此，TcpStream 会读取自身的数据来观察客户端发送的内容，并允许我们将响应写回到流上去
    // 简单来说，下面的代码中的 for 循环会依次处理每个连接，并生成一系列的流供我们处理
    for stream in linstener.incoming() {
        // 生成 TCP 流
        // 在目前对流处理过程中，我们选择在出现任何错误的情形下调用 unwrap 来结束程序，而在程序成功的情形下打印出一段信息
        // incoming 方法之所以会在客户端连接服务器时产生错误，是因为我们并没有对连接本身进行遍历，而仅仅只是遍历了连接尝试(connection attempt)
        // 连接过程 可能会因为相当多的原因而失败，其中大部分都与操作系统相关
        // 例如，许多操作系统都会限制同时打开的连接数，试图创建超过这个数 目的新连接就会产生错误，直到某些已经打开的连接关闭为止
        let stream = stream.unwrap();
        // println!("Connection established");

        // 从 tcp 流中读取数据
        handle_connection(stream);
    }
}


// stream参数被声明为了可变的，因为 TcpStream 实例的内部记录了返回给我们的数据
// 它可能会读取多于我们请求的数据，并将这些数据保存下来以备下次请求时使用
// 因为 TcpStream的内部状态可能会被改变，所以我们需要将它标记为mut。
fn handle_connection(mut stream: TcpStream) {
    // 在栈上声明一个用于存放数据的buffer，大小是 512 个字节
    // [0; 512] 用来声明一个固定大小的数组
    let mut buffer = [0; 512];
    // read 方法会从 TcpStream 中读取数据并将其存储至缓冲区中
    // 返回值是读取了多少字节
    stream.read(&mut buffer).unwrap();


    // 编写 http 响应
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush();

    // 我们将缓冲区中的字节转换成字符串并打印了出来
    // 函数 String:: from_utf8_lossy 可以接收一个 &[u8] 并产生对应的 String
    // 它名字中的 lossy 部分暗示了这个函数遇到无效 UTF-8序 列时的行为:它会用 (U+FFFD REPLACEMENT CHARACTER)来替换所有无效的序列
    // 我们可能会在缓冲区中那部分没有被请求数据占据的地方看到这种替代字符
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

// HTTP 请求格式
// HTTP是一个基于文本的协议，它的请求采用了如下所示的格式:
// Method Request-URI HTTP-Version CRLF
// headers CRLFCRLF
// message-body

// 第一行被称作请求行(request line)，它存放了客户端请求的信息
// 第一部分表明了当前请求使用的方法，比如 GET 或 POST，它描述了客户端请求数据的方式
// 第二部分 Request-URI 代表了客户端正在请求的统一资源标识符 (Uniform Resource Identifier，URI): URI 大体上类似于统一资源定位符 (Uniform Resource Locator，URL)但不完全一样。这里可以理解为 URI 就是 URL
// 最后一部分是客户端使用的 HTTP 版本，
// 接着，请求行就以 CRLF 序列结束了。CR 与 LF 分别代表回车 (Carriage Return) 与换行 (Line Feed)，它们是从打字机时代传承下来的术语
// CRLF 序列也被写作 \r\n，其中 \r 代表回车，\n 代表换行。CRLF序列会将请求行和请求数据的其他部分区别开来
// 需要注意的是，我们会在打印 CRLF 时看到一个新行而不是字符\r\n
// 第二行开始是 header，也就是请求头，请求头也是以 CRLF 结尾，注意，这里是两个 CRLF，表示请求体与请求头之间空一行
// 最后一个内容就是 message-body，也就是请求体

// HTTP 响应格式

// HTTP-Version Status-Code Reason-Phrase CRLF
// headers CRLFCRLF
// message-body

// 第一行被称作状态行 (status line)，它包含了当前响应的 HTTP 版本、一个汇总了请求结果的数字状态码，以及一段提供了状态 码文本描述的原因短语。状态行已 CRLF 结尾

// 第二行是任意数量的消息头，也就是响应头，响应头同样是以两个 CRLF 结尾
// 最后一部分内容是响应消息体，同样响应消息体与响应头之间空一行