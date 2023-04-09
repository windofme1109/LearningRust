// 结构体
// Rust 中的结构体与JavaScript 中的对象十分类似
// 是一种由大括号包裹的 key: value 形式

// 定义一个结构体使用 struct 关键字
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

// 上面的例子中，使用了 String 而不是 &str 切片类型，这是一个有意为之的选择
// 因为我们希望这个结构体的实例拥有自身全部数据的所有权
// 在这种情形下，只要结构体是有效的，那么它携带的全部数据也就是有效的

// 元组结构体
// 元组结构体同样拥有用于表明自身含义的名称
// 但你无须在声明它时对其字段进行命名，仅保留字段的类型即可。
// 一般来说，当你想要给元组赋予名字，并使其区别于其他拥有同样定义的元组时，你就可以使用元组结构体
// 想声明一个约束元组类型的结构体，就可以使用元组结构体，只声明类型，不声明字段
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 可以定义一个空的结构体
// 当你想要在某些类型上实现一个trait，却不需要在该类型中存储任何数据时，空结构体就可以发挥相应的作用
struct Person {}

fn main() {

    // 声明一个结构体实例
    let user = User {
        email: String::from("abc@163.com"),
        username: String::from("Nikky"),
        active: true,
        sign_in_count: 100
    };

    // 访问结构体实例中的属性，使用点的形式，这一点和 JavaScript 等语言类似
    let user_email = user.email;
    println!("user_email {}", user_email);

    // 定义一个可以修改属性的结构体实例
    // 默认定义的结构体实例，我们不能直接修改其属性
    // 如果想要修改，必须使用 mut 关键字定义结构体实例
    let mut user2 = User {
        email: String::from("qwe@163.com"),
        username: String::from("Curry"),
        active: true,
        sign_in_count: 100
    };
    // 一旦实例可变，那么实例中的所有字段都将是可变的，Rust不允许我们单独声明某一部分字段的可变性
    user2.active = false;

    let user3 = build_user(String::from("jack"), String::from("cdc@163.com"));


    // 从一个实例中创建另外一个实例
    // 多数情况下，对于由同一个结构体创建的实例，除了需要修改的小部分字段
    // 其余字段的值与旧实例中的完全相同
    // 我们可以使用结构体更新语法来快速实现此类新实例的创建
    let user4 = User {
        email: String::from("aaa@163.com"),
        username: String::from("ada"),
        ..user
    };
    // ..user 是结构体更新语法，表示剩下的那些还未被显式赋值的字段都与给定实例 user 拥有相同的值
    // user4 中的 active 和 sign_in_count 和 user 中的相同


    let white = Color(0, 0, 0);
    let black = Color(255, 255, 255);

    let point = Point(100, 1000,99);
}

// 使用函数生成结构体
fn build_user(username: String, email: String) -> User {
    User {
        // 在变量名与字段名相同时使用简化版的字段初始化方法，即只写属性名即可，与 JavaScript 中声明对象中同名属性与变量类型
        username,
        email,
        active: true,
        sign_in_count: 100101
    }
}
