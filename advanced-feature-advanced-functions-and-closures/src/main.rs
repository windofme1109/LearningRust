// 函数指针
// 我们曾经讨论过如何将闭包传递给函数，但实际上你同样可以将普通函数传递至其他函数！
// 这一技术可以帮助你将已经定义好的函数作为参数，而无须重新定义新的闭包
// 函数会在传递的过程中被强制转换成 fn 类型，注意这里使用了小写字符 f 从而避免与 Fn 闭包 trait 相混淆
// fn类型也就是所谓的函数指针（function pointer）


fn main() {
    println!("Hello, world!");
    

    // 函数指针
    // 向函数中传入函数，将 add_one 函数作为 do_twice 的参数
    let result = do_twice(add_one, 5);
    // The answer is: 12
    println!("The answer is: {}", result);
    
    // 数组的 Map 函数就可以接收闭包，也可以接收普通的函数作为参数

    let list_of_numbers = vec![1, 2, 3];
    // map 函数接收闭包作为参数，将数字转换为字符串
    let list_of_strings: Vec<String> = 
    list_of_numbers
    .iter()
    .map(|i| {
        i.to_string()
    })
    .collect();

    // map 函数接收普通函数作为参数，将数字转换为字符串
    let list_of_strings2: Vec<String> = 
    list_of_numbers
    .iter()
    .map(ToString::to_string)
    .collect();

    // 注意：
    // 这里调用 to_string 必须使用完全限定语法，因为此作用域中存在多个可用的 to_string 函数
    // 本例使用的是 ToString trait中的to_string函数
    // 而标准库已经为所有实现了 Display 的类型都自动实现了这一trait

    // 另外还有一种十分有用的模式，它利用了元组结构体和元组结构枚举变体的实现细节
    // 这些类型的初始化语法()与调用函数有些相似
    // 实际上，它们的构造器也确实被实现为了函数，该函数会接收它们的参数并返回一个新的实例
    // 因此，我们可以把构造器视作实现了闭包 trait 的函数指针，并在那些接收闭包的方法中使用它们
    // 
    // 也就是说，可以将元组结构体、枚举的变体的初始化过程当做一个闭包（函数）
    // 并可以将其传入接收闭包或者函数的函数中
    let list_of_statuses: Vec<Status> =
        (0u32..20)
        .map(Status::Value)
        .collect();



        // 由于闭包使用了 trait 来进行表达，所以你无法在函数中直接返回一个闭包
        // 在大多数希望返回 trait 的情形下，你可以将一个实现了该 trait 的具体类型作为函数的返回值
        // 但你无法对闭包执行同样的操作，因为闭包没有一个可供返回的具体类型;例如，你无法把函数指针 fn 用作返回类型

        // 也就是说，Rust 无法直接返回一个闭包或者是函数

}


// 在一个函数中，尝试返回闭包
// 编译过程中，会报错：
// error[E0782]: trait objects must include the `dyn` keyword
//   --> src/main.rs:64:25
//   |
// 64 | fn returns_closure() -> Fn(i32) -> i32 {
//   |                         ^^^^^^^^^^^^^^
//   |
// help: add `dyn` keyword before this trait
//   |
// 64 | fn returns_closure() -> dyn Fn(i32) -> i32 {
//   |  
// trait 对象必须使用 dyn 关键字
// fn returns_closure() -> Fn(i32) -> i32 {
//     |x| x + 1
// }

// 单独添加 dyn 关键字，也会报错：
// error[E0746]: return type cannot have an unboxed trait object
//   --> src/main.rs:83:25
//   |
// 83 | fn returns_closure() -> dyn Fn(i32) -> i32 {
//   |                         ^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
//   |
// 是因为 Rust 不知道返回的闭包的大小
// fn returns_closure() -> dyn Fn(i32) -> i32 {
//     |x| x + 1
// }

// 因此，需要使用智能指针将返回的闭包包裹
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

enum Status {
    Value(u32),
Stop, }


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

// 当然，在某些情形下，我们可能只想接收 fn 而不想接收闭包
// 比如与某种不支持闭包的外部代码进行交互时：C函数可以接收函数作为参数，但它却没有闭包