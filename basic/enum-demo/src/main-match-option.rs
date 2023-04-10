
// 匹配 Option<T>
// match 模式匹配结合 Option<T>，可以实现对 Some 中保持的数据的操作
fn plus_one(x: Option<i32>) -> Option<i32> {
    // x 是 Option<T> 枚举类型
    // 可能的值为 Some 或者是 None
    // x 会依次与 match 中的每个模式进行匹配
    match x {
        // 匹配到 Some 这个变体，那么 x 也必须是 Some(value) 的形式
        // 因此这里的 i 绑定了 Some 所包含的值 value
        // 然后执行分支代码，将 i 加 1，然后返回一个新的 Some 变体，此时 Some 包含的值就是 value + 1
        Some(i) => Some(i + 1),
        // 匹配到 None 这个变体，目前是没有进行操作，依旧返回一个 None 的变体
        None => None
    }
}

// 模式匹配必须匹配所有的值
// fn add_one(x: Option<i32>) -> Option<i32> {
//     // Option<T> 枚举共有两种变体：Some 和 None
//     // 模式匹配中，如果只写一种 Some，编译就会报错
//     // Rust中的匹配是穷尽的 (exhausitive): 我们必须穷尽所有的可能性，
//     // 来确保代码是合法有效的
//     //  pattern `None` not covered
//     match x {
//         Some(i) => Some(i + 1),
//     }
// }


// 通配符 _
// 在某些情况下，我们在使用模式匹配过程中，不需要列举出所有的可能
// 前面提到过，如果不列举出所有的可能，编译就会报错
// 那么为了解决这个冲突，Rust 提供了一种操作，就是使用通配符，来省略用不到的模式

enum MyNum {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten
}

fn match_odd(x: MyNum) {
    // 有一个 1 - 10 的枚举，只需要匹配奇数枚举，
    // 此时就可以使用 通配符_，省略我们不需要的模式，
    
    match x {
        MyNum::One => println!("one"),
        MyNum::Three => println!("three"),
        MyNum::Five => println!("five"),
        MyNum::Seven=> println!("seven"),
        MyNum::Nine => println!("nine"),
        // 使用通配符，省略剩下的模式
        _ => (),
    }
}

fn main() {
    // 定义一个值为 5 的 Some 变体
    let five = Some(5);
    // 调用 plus_one，传入 five，因此匹配到 Some 这个模式，所以
    // six 是包裹的值为 6 的 Some 变体
    let six = plus_one(five);
    // None 这个变体，因此匹配到 Some 这个模式，所以 none 是一个 None 的变体
    let none = plus_one(None);

    // 将 match 与枚举相结合在许多情形下都是非常有用的
    // 在 Rust 代码中看到许多类似的套路:
    // 使用match来匹配枚举值，并将其中的值绑定到某个变量上，接着根据这个值执行相应的代码

}