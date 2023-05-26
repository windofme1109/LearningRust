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

// 3. 永远不返回的类型 Never 类型

// Rust 有一个名为!的特殊类型
// 它在类型系统中的术语为空类型 (empty type)，因为它没有任何的值。我们倾向于叫它never类型，
// 因为它在从不返回的函数中充当返回值的类型

// 一个返回 Never 类型的函数
// fn bar() -> ! {
//     // 
// }

// 上面的这个函数可以读作“函数 bar 永远不会返回值”
// 不会返回值的函 数也被称作发散函数 (diverging function)。我们不可能创建出类型为 ! 的值来让 bar 返回

// match 语句中的 continue 返回的就是 Never 类型
// match表达式中，每个分支都必须返回相同的类型
// 因此，像下面这种match 表达式是不合法的：
// match guess {
//     Ok(_) => 5,
//     Err(_) => 'hello'
// }

// 而使用 continue 就没有这个问题

// panic! 宏的实现同样使用了 never 类型。
// Option<T> 值上调用 unwrap 函数吗? 它会生成一个值或触发 panic。下面便是 unwrap 函数的定义:
// impl<T> Option<T> {
//     pub fn unwrap(self) -> T {
//         match self {
//             Some(val) => val,
//             None => panic!("called `Option::unwrap()` on a `None` value"),
//         }
//      } 
// }

// Rust注 意到 val 拥有类型 T，而 panic! 则拥有返回类型!，所以整个 match 表达式的返回类型为 T
// 这段代码之所以可以正常工作是因为panic! 只会中断当前的程序而不会产生值
// 因为我们不会在 None 的情况下为 unwrap 返回一个值，所以这段代码是合法的

// loop 表达式的返回值也是 !
// loop {
//     print!("and ever ");
// }

// 由于loop循环永远不会结束，所以这个表达式以!作为自己的返回类型
// 当然，循环中也可能会存在break指令，并会在逻辑执行至 break时中止。


// 4. 动态大小类型和 Sized trait


// Rust 需要在编译时获取一些特定的信息来完成自己的工作，比如应该为一个特定类型的值分配多少空间等
// 但 Rust 的类型系统中又同时存在这样一处令人疑惑的角落： 动态大小类型（Dynamically Sized Type，DST）的概念
// 它有时也被称作不确定大小类型（unsized type）

// 我们讨论一下 str 的动态大小类型，注意，这里讨论的是 str 本身而不是 &str
// str 正好是一个动态大小类型。我们只有在运行时才能确定字符串的长度
// 这也意味着我们无法创建一个 str 类型的变量，或者使用str类型来作为函数的参数
// 如下所示的代码无法正常工作：
// let s1: str = "Hello there!";
// let s2: str = "How's it going?"

// Rust 需要在编译时确定某个特定类型的值究竟会占据多少内存，而同一类型的所有值都必须使用等量的内存
// 假如Rust允许我们写出上面这样的代码，那么这两个 str 的值就必须要占据等量的空间
// 但它们确实具有不同的长度：s1 需要 12 字节的存储空间，而 s2 则需要15字节
// 这也是我们无法创建出动态大小类型变量的原因

// 既然无法使用 str 动态类型创建变量
// 我们应该使用 &str 字符串切片类型创建变量
// 切片的数据结构中会存储数据的起始位置及切片的长度
// 因此，尽管 &T 被视作存储了 T 所在内存地址的单个值
// 但 &str 实际上是由两个值组成的：str 的地址与它的长度
// 这也使我们可以在编译时确定 &str 值的大小：其长度为 usize 长度的两倍
// 换句话说，无论 &str 指向了什么样的字符串，我们总是能够知道 &str 的大小
// 这就是 Rust 中使用动态大小类型的通用方式：它们会附带一些额外的元数据来存储动态信息的大小
// 我们在使用动态大小类型时总是会把它的值放在某种指针的后面

// 我们可以将 str 与所有种类的指针组合起来，例如Box<str>或 Rc<str>等
// 事实上，我们已经见到过类似的用法了，只不过当时使用了另外一种动态大小类型：trait
// 每一个 trait 都是一个可以通过其名称来进行引用的动态大小类型

// 为了处理动态大小类型，Rust 还提供了一个特殊的 Sized trait 来确定一个类型的大小在编译时是否可知
// 编译时可计算出大小的类型会自动实现这一 trait
// 另外，Rust 还会为每一个泛型函数隐式地添加 Sized 约束：

// fn generic<T>(t: T) {
//
// }

// 上面的泛型函数会被默认转换为：

// fn generic<T :Sized>(t: T) {
//
// }

// 在默认情况下，泛型函数只能被用于在编译时已经知道大小的类型
// 但是，你可以通过如下所示的特殊语法来解除这一限制：
// fn generic<T: ?Sized>(t: &T) {
// // --略
// --
// }
// ?Sized trait 约束表达了与 Sized 相反的含义，我们可以将它读作 “T可能是也可能不是Sized的”
// 这个语法只能被用在 Sized 上，而不能被用于其他 trait

// 另外还需要注意的是，我们将 t 参数的类型由 T 修改为了 &T。因为类型可能不是 Sized 的
// 所以我们需要将它放置在某种指针的后面。在本例中，我们选择使用引用





fn main() {
    // println!("Hello, world!");

    // 类型别名

    // 由于 Kilometers 和 i32 是同一种类型，所以我们可以把两个类型的值相加起来
    // 甚至是将 Kilometers 类型的值传递给以 i32 类型作为参数的函数
    let x: i32 = 5;
    let y: Kilometers = 5;
    // x + y = 10
    println!("x + y = {}", x + y);

    // Never 类型
    let guess = Ok(5);
    // 不合法的 match 表达式，因为分支返回了不同的值 
    // match guess {
    //     Ok(_) => 5,
    //     Err(_) => "hello"
    // }

    // 报错信息如下
    // error[E0308]: `match` arms have incompatible types
    //    --> src/main.rs:128:19
    //    |
    // 126| /     match guess {
    // 127| |         Ok(_) => 5,
    //    | |                  - this is found to be of type `{integer}`
    // 128| |         Err(_) => "hello"
    //    | |                   ^^^^^^^ expected integer, found `&str`
    // 129| |     }
    //    | |_____- `match` arms have incompatible types

    // 使用 continue 不存在类型不统一的问题
    // 因为 continue 的返回值就是 Never 类型
    // loop {
    //     let a: i32 = match guess {
    //         Ok(_) => 5,
    //         Err(_) => continue,
    //     };
    // }

    // 当Rust计算 guess的类型时，它会发现在可用于匹配的两个分支中
    // 前者的返回类型为 u32 而后者的返回类型为!
    // 因为 ! 无法产生一个可供返回的值，所以 Rust 采用了 u32 作为 guess 的类型

    // 对于此类行为，还有另外一种更加正式的说法:类型!的表达式可以被强制转换为其他的任意类型
    // 我们之所以能够使用 continue 来结束 match 分支
    // 是因为 continue 永远不会返回值
    // 相反地，它会将程序的控制流转移至上层循环
    // 因此，这段代码在输入值为 Err 的情况下不会对 guess 进行赋值
    

    // panic! 返回值也是 !
    // loop 表达式的返回值也是 !
    // let r = loop {
    //     print!("and ever ");
    // }


    // 动态大小类型

    // str 是动态类型，只能运行时才能确定其大小
    // 因此像下面的代码时无法通过编译的：
    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";

    // error[E0308]: mismatched types
    //   --> src\main.rs:16:19
    //    |
    // 16 |     let s1: str = "Hello there!";
    //    |             ---   ^^^^^^^^^^^^^^ expected `str`, found `&str`
    //    |             |
    //    |             expected due to this
    //
    // error[E0308]: mismatched types
    //   --> src\main.rs:17:15
    //    |
    // 17 | let s2: str = "How's it going?";
    //    |         ---   ^^^^^^^^^^^^^^^^^ expected `str`, found `&str`
    //    |         |
    //    |         expected due to this
    //
    // error[E0277]: the size for values of type `str` cannot be known at compilation time
    //   --> src\main.rs:16:9
    //    |
    // 16 |     let s1: str = "Hello there!";
    //    |         ^^ doesn't have a size known at compile-time
    //    |
    //    = help: the trait `Sized` is not implemented for `str`
    //    = note: all local variables must have a statically known size
    //    = help: unsized locals are gated as an unstable feature
    // help: consider borrowing here
    //    |
    // 16 |     let s1: &str = "Hello there!";
    //    |             +
    //
    // error[E0277]: the size for values of type `str` cannot be known at compilation time
    //   --> src\main.rs:17:5
    //    |
    // 17 | let s2: str = "How's it going?";
    //    |     ^^ doesn't have a size known at compile-time

    // 上面的报错信息显示：值的大小无法在编译期间确定

}
