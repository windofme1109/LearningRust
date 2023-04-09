
// Option 系统内置的枚举
// Rust 中没有 null，也就是空值
// 虽然没有空值，但是 Rust 也是可以通过某种方式来表示空值的概念
// Rust 中使用枚举的变体来标识一个值的无效或者是缺少
// 标准库中 Option 的定义
// enum Option<T> {
//     Some(T),
//     None
// }


fn main() {
    // 由于Option<T>枚举非常常见且很有用，所以它也被包含在了预导入（prelude）模块中
    // 这意味着我们不需要显式地将它引入作用域
    // 它的变体也是这样的：我们可以在不加 Option:: 前缀的情况下直接使用Some 或 None
    // 但 Option<T> 枚举依然只是一个普通的枚举类型，Some(T) 和 None 也依然只是 Option<T> 类型的变体
    let some_integer = Some(5);
    let some_string = Some("a string");
    // 我们使用了 None 而不是 Some 变体来进行赋值
    // 那么我们需要明确地告知 Rust 这个 Option<T> 的具体类型
    // 这是因为单独的None变体值与持有数据的 Some 变体不一样，编译器无法根据这些信息来正确推导出值的完整类型
    let absent_number:Option<i32> = None;

    // 当我们有一个 Some 变体时，我们就可以确定值时存在的，并且被 Some 持有
    // 而有一个 None 变体时，我们就知道当前的值不是有效的
    // 这看上去与空值没有什么差别，那为什么Option<T>的设计就比空值好呢
    // 先看一个例子
    let x: i8 = 5;
    let y: Option<i8> = Some(10);
    // error[E0277]: cannot add `Option<i8>` to `i8`
    // let z = x + y;

    // i8 类型不能和 Option<i8> 类型直接相加，因为 i8 和 Option<i8> 不是一个类型
    // 因为Option<T>和T（这里的T可以是任意类型）是不同的类型
    // 所以编译器不会允许我们像使用普通值一样去直接使用 Option<T> 的值
    // 因此 x + y 会报错

    // 当我们在Rust中拥有一个i8
    // 类型的值时，。我们可以
    // 充满信心地去使用它而无须在使用前进行空值检查。而只有当我们持
    // 有的类型是Option<i8>（或者任何可能用到的值）时，我们才必须要
    // 考虑值不存在的情况，同时编译器会迫使我们在使用值之前正确地做
    // 出处理操作
    // 如果我们持有的是 i8 类型的值，我们可以放心地去使用这个类型，因为编译器可以确保我们使用的值是有效的
    // 当我们使用的值是 Option<i8>（或者任何可能用到的值），我们必须要要考虑值不存在的情况
    // 同时编译器会迫使我们在使用值之前做出正确的操作

    // 为了使用 Option<T> 中可能存在的 T，我们必须要将它转换为 T
    // 一般而言，这能避免使用空值时的一个最常见的问题：假设某个值存在，实际上却为空

    // 如果我们需要持有一个可能为空的值，需要将其放入对应类型的 Option<T> 的值中
    // 而且必须显式处理可能为空值的情况

    // 无论在任何地方，只要我们遇到类型不是 Option<T> 的的值，就可以放心大胆的使用，无需考虑其为空值的情况
    // 当你持有了一个 Option<T> 类型的Some变体时你应该怎样将其中的T值取出来使用呢？
    // Option<T> 枚举针对不同的使用场景提供了大量的实用方法
    // 你可以在官方文档中找到具体的使用说明

    // 总结
    // Option<T> 有两种变体 Some 和 None
    // Some 表示某种类型的值，None 表示空值，使用 None 时，必须指定类型
    // Option<T> 与 T 不是同一个类型，不能直接相互操作，如果想把 T 值取出来，必须使用相关的方法
    // 我们需要使用的某种类型的值，有可能为空，此时我们要将其放入对应类型的 Option<T> 中
    // 如果遇到正常类型 T，非 Option<T>，那么我们就可以放心地使用，而无需考虑空值的情况
    // 为了使用 Option<T>，必须处理每种变体的情况
}