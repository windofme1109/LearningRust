// 枚举

// 使用 enum 关键字定义枚举

enum IpAddrKind {
    V4,
    V6,
}

// 枚举允许我们将关联的数据嵌入枚举变体内
// 而无需将枚举集成至结构体中
enum IpAddrKind_2 {
    V4(String),
    V6(String)
}

// 可以将枚举的每个变体设置为不同的数据类型
enum IpAddrKind_3 {
    V4(u8, u8, u8, u8),
    V6(String)
}

// 给枚举的变体定义不同的数据类型
enum Message {
    //
    Quit,
    // 结构体
    Move {x: i32, y: i32},
    // 字符串
    Write(String),
    // 多个整型 i32
    ChangeColor(i32, i32, i32)
}

// 与结构体类似，可以使用 impl 给枚举定义方法
impl Message {
    // 方法的第一个参数是 &self，依旧是对枚举实例的不可变引用
    fn call(&self) {
        // self
    }
}

// 将枚举与具体的数据关联
struct IpAddr {
    // 将 kind 指定为枚举类型
    kind: IpAddrKind,
    address: String
}

fn main() {

    // 使用枚举
    // 使用枚举的两个变体
    // 枚举的变体全都位于其标识符的命名空间中，并使用两个冒号来将标识符和变体分隔开来
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // 调用 route 函数，并传入 IpAddrKind 的变体
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // println!("Hello, world!");

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };


    // 数据附加到了枚举的每个变体中，这样便不需要额外地使用结构体
    let home = IpAddrKind_2::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind_2::V6(String::from("::1"));

    let home = IpAddrKind_3::V4(127, 0, 0, 1);
    let loopback = IpAddrKind_3::V6(String::from("::1"));


    let m_1 = Message::Write(String::from("hello word"));
    // 调用枚举方法
    m_1.call();
}

// 由于枚举的变体的类型都相同，所以可以定义一个函数来同一处理
fn route(ip_kind: IpAddrKind) {}

