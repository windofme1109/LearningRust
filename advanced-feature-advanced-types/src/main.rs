// 高级类型

// 1. 使用 newtype 模式实现类型安全与抽象
// newtype 可以理解为是一种封装
// newtype 模式可以绕过孤儿规则从而为外部类型实现外部的 trait
// 将外部类型封装为内部类型，从而实现某种外部 trait

// newtype 可以被用来静态地保证各种值之间不会被混淆及表明值使用的单位
// 如前面提到过的 Meters 和 Millimeters 的例子，封装了 u32 类型，但是使用单位不同（不同的结构体）
// 因此不能混用，在使用 Meters 的场合使用了Millimeters，就会发生编译错误


// newtype 模式的另外一个用途是为类型的某些细节提供抽象能力
// 例如，新类型可以暴露出一个与内部私有类型不同的公共API，从而限制用户可以访问的功能

// newtype 模式还可以被用来隐藏内部实现
// 例如，我们可以提供People 类型来封装一个用于存储人物 ID 及其名称的 HashMap<i32,String>
// People 类型的用户只能使用我们提供的公共 API
// 比如一个添加名称字符串到 People 集合的方法；而调用该方法的代码不需要知道我们在内部赋予了名称一个对应的i32 ID
// newtype 模式通过轻量级的封装隐藏了实现细节

// 2. 使用类型别名创建同义类型
// 除了 newtype 模式，Rust 还提供了创建类型别名 （type alias）的功能
// 它可以为现有的类型生成另外的名称。这一特性需要用到 type 关键字

// 使用 type 关键字创建类型别名
type Kilometers = i32;

// 类型别名最主要的用途是减少代码字符重复
// 如果我们有一个比较长的类型，如下所示：
// Box<dyn Fn() + Send + 'static>
// 很多地方都用到了这个类型：

// let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"))

// fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
//     // --略
//
// }
//
// fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
//     // --略
//     Box::new(|| ())
// }

// 因此，我们可以使用类型别名来来简化重复类型的书写
type Thunk = Box<dyn Fn() + Send + 'static>;
fn takes_long_type(f: Thunk) {
    // --略

}

fn returns_long_type() -> Thunk {
    // --略
    Box::new(|| ())
}

// 标准库中，也使用了类型别名
// 如标准库中的 std::io 模块，该模块下的 I/O 操作常常会返回Result<T, E>
// 来处理操作失败时的情形，另外 ， 该代码库使用了一个 std::io::Error 结构体来表示所有可能的 I/O 错误
// 而大部分 std::io 模块下的函数都会将返回类型 Result<T, E> 中的 E 替换为 std::io::Error
// 即 Result<T, std::io::Error>
// 显然，每使用一次 Result，都要写一次 std::io::Error
// 示例如下：
// use std::io::Error;
// use std::fmt;
// pub trait Write {
//     fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
//     fn flush(&mut self) -> Result<(), Error>;
//     fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
//     fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
// }

// 所以，标准库 std::io 中有一个对 Result<T, std::io::Error> 的类型别名：
// type Result<T> = Result<T, std::io::Error>

// 由于该声明被放置在 std::io 模块中，所以我们可以使用完全限定别名 std::io::Result<T> 来指向它
// 即指向将 std::io::Error 填入 E 的 Result<T, E>
// 简化后的 Write trait 函数如下所示：
// pub trait Write {
//     fn write(&mut self, buf: &[u8]) -> Result<usize>;
//     fn flush(&mut self) -> Result<()>;
//     fn write_all(&mut self, buf: &[u8]) -> Result<()>;
//     fn write_fmt(&mut self, fmt: Arguments) -> Result<()>;
// }
// 也就是说，我们直接 std::io 中的 Result<T> 即可，而不用使用 Result<T, E>

// 使用类型别名可以从两个方面帮助我们：
// 1. 编写代码更加轻松，并且为整个 std::io 提供了一致的接口
// 2. 由于它仅仅是别名，也就是另外一个 Result<T, E>，所以我们可以在它的实例上调用Result<T, E>拥有的任何方法，甚至是？运算符

fn main() {
    // println!("Hello, world!");

    // 由于 Kilometers 和 i32 是同一种类型，所以我们可以把两个类型的值相加起来
    // 甚至是将 Kilometers 类型的值传递给以 i32 类型作为参数的函数
    let x: i32 = 5;
    let y: Kilometers = 5;
    // x + y = 10
    println!("x + y = {}", x + y);

}
