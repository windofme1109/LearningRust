// 添加 #[derive(Debug)] 注解，表示当前结构体可以通过 debug 模式打印
#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

fn main() {
    let s = area(30, 50);
    let s2 = area_2((30, 50));

    let rect1 = Rectangle {
        width: 30,
        length: 50
    };
    let s3 = area_3(&rect1);

    println!("the area is {}", s);
    println!("the area2 is {}", s2);
    println!("the area3 is {}", s3);

    // 正常情况下，Rust 不能打印出结构体，因为结构体默认没有实现 std::fmt::Display
    // error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`

    // println! 宏可以执行多种不同的文本格式化命令，而作为默认选项
    // 格式化文本中的花括号会告知 println! 使用名为 Display 的格式化方法
    // 这类输出可以被展示给直接的终端用户。我们目前接触过的所有基础类型都默认地实现了 Display
    // 因为当你想要给用户展示 1 或其他基础类型时没有太多可供选择的方式
    // 但对于结构体而言，println! 则无法确定应该使用什么样的格式化内容：
    //    1. 在输出的时候是否需要逗号？
    //    2. 需要打印花括号吗？
    //    3. 所有的字段都应当被展示吗？
    // 正是由于这种不确定性，Rust没有为结构体提供默认的Display实现

    // 可以使用这种形式的占位符：{:?}，它会告知 println! 当前的结构体需要使用名为Debug的格式化输出。
    // Debug是另外一种格式化 trait，它可以让我们在调试代码时以一种对开发者友好的形式打印出结构体。
    // 单独使用 {:?} 这种占位符还是不能打印出结构体的内容，因为必须为自己的结构体显式地选择这一功能
    // 为了完成该声明，我们需要在结构体定义前添加 #[derive(Debug)] 注解，表示派生 Debug 整个注解
    // {:?} 占位符 + #[derive(Debug)] 注解，才能正确打印出结构体
    // the rect is Rect { width: 30, length: 50 }

    println!("the rect is {:?}", rect1);

    // {:#?} 占位符，可以对打印结果进行格式化，如下所示：
    // the rect is Rect {
    //     width: 30,
    //     length: 50,
    // }
    println!("the rect is {:#?}", rect1);
}

// 直接定义计算长方形面积的函数
//接收长和宽作为参数
fn area(width: u32, length: u32) -> u32 {
    return width * length;
}

// 优化版本
// 长方形的长和宽实际上是一组关联数据，因为只有长和宽才能定义一个长方形
// 因此，应该将长和宽组合到一起，使得代码更加易懂
// 这里使用元组将长和宽组合到一起
fn area_2(rect: (u32, u32)) -> u32 {
    // 元素的索引0对应了宽度 width，而索引1则对应了长度 length
    let (width, length) = rect;
    width * length
}

// 优化版本 - 2
// 使用元组定义参数，好处是传一个参数（元组）就代表了长方形的长和宽，将长和宽紧密的联系起来了
// 但是函数内部在使用的时候，只能根据元组的索引去获得数据，即索引 0 代表宽，索引 1 代表长
// 没有任何语义性，只能靠这种约定，如果 area 进行迭代，其内部需要长和宽进行其他操作，一旦使用了错误的索引，就会将长和宽搞混，进而出现问题
// 因此，我们可以使用结构体来为这些数据增加有意义的标签
// 结构体 Rectangle 内部包含了 u32 类型的字段 width 和 length
// 在函数内部就可直接使用 Rectangle 实例的 width 属性和 length
// 语义性非常明确
// 定义 rect 为 结构体 Rect 实例的引用
fn area_3(rect: &Rectangle) -> u32 {
    rect.width * rect.length
}