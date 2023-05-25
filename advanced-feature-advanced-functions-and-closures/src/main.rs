// 动态大小类型和 Sized trait
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
    println!("Hello, world!");
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

    // 函数指针
    // 向函数中传入函数，将 add_one 函数作为 do_twice 的参数
    let result = do_twice(add_one, 5);
    // The answer is: 12
    println!("The answer is: {}", result);
    //
}


// 函数指针
// 我们曾经讨论过如何将闭包传递给函数，但实际上你同样可以将普通函数传递至其他函数！
// 这一技术可以帮助你将已经定义好的函数作为参数，而无须重新定义新的闭包
// 函数会在传递的过程中被强制转换成 fn 类型，注意这里使用了小写字符 f 从而避免与 Fn 闭包 trait相混淆
// fn类型也就是所谓的函数指针（function pointer）

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// 其中函数 do_twice 的参数
// f 被指定为了 fn 类型，它会接收 i32 类型作为参数
// 并返回一个i32作为结果。随后 do_twice 函数体中的代码调用了两次 f

// fn 是一个类型而不是一个 trait。因此，我们可以直接指定 fn 为参数类型
// 而不用声明一个以 Fn trait 为约束的泛型参数

// 由于函数指针实现了全部 3 种闭包 trait （ Fn 、 FnMut 以及 FnOnce）
// 所以我们总是可以把函数指针用作参数传递给一个接收闭包的函数
// 也正是出于这一原因，我们倾向于使用搭配闭包 trait 的泛型来编写函数，这样的函数可以同时处理闭包与普通函数

// 当然，在某些情形下，我们可能只想接收fn而不想接收闭包
// 比如与某种不支持闭包的外部代码进行交互时：C函数可以接收函数作为参数，但它却没有闭包