// 模式的可失败形式与不可失败形式
// 模式可以被分为不可失败 （ irrefutable ） 和可失败（refutable）两种类型
// 1. 不可失败的模式
// 不可失败的模式能够匹配任何传入的值
// 例如，语句let x = 5;中的 x 便是不可失败模式，因为它能够匹配表达式右侧所有可能的返回值
//
// 2. 可失败模式
// 可失败模式则可能因为某些特定的值而匹配失败
// 例如，表达式 if let Some(x) = a_value 中的 Some(x)便是可失败模式
// 如果 a_value 变量的值是 None 而不是 Some，那么表达式左侧的Some(x)模式就会发生不匹配的情况

// 函数参数、let 语句及 for 循环只接收不可失败模式，因为在这些场合下，我们的程序无法在值不匹配时执行任何有意义的行为
//
// if let和 while let 表达式则只接收可失败模式，因为它们在被设计时就将匹配失败的情形考虑在内了：条件表达式的功能就是根据条件的成功与否执行不同的操作


fn main() {
    // 对 let 语句使用可失败模式，那么会编译不通过
    let some_option_value = Some(10);
    // let Some(x) = some_option_value;
    // 上面的代码会编译不通过：
    //     error[E0005]: refutable pattern in local binding
    //   --> src\main-irrefutable-refutable:20:9
    //    |
    // 20 |     let Some(x) = some_option_value;
    //    |         ^^^^^^^ pattern `None` not covered

    // let Some(x) = some_option_value; 没有覆盖 None 这个模式

    // 因此，对于可失败模式，我们需要使用能接收可失败模式的模式匹配：
    if let Some(x) = some_option_value {
        println!("matched value is {}", x);
    }

    // 给 if let 使用不可失败模式，会产生一个警告：
    if let x = 5 {
        println!("{}", x)
    }
    //     warning: irrefutable `if let` pattern
    //   --> src\main-irrefutable-refutable:36:8
    //    |
    // 36 |     if let x = 5 {
    //    |        ^^^^^^^^^
    //    |
    //    = note: this pattern will always match, so the `if let` is useless

    // 警告信息说明，这个模式总会匹配成功（不可失败模式），所以 if let 是没有意义的

    // 因此，避免在 if let 中使用不可失败模式

// 在match表达式的匹配分支中，除了最后一个，其他必须全部使用可失败模式，而最后的分支则应该使用不可失败模式
// 因为它需要匹配值的所有剩余的情形。
// Rust允许你在仅有一个分支的 match 表达式中使用不可失败模式，但这种语法几乎没有任何用处，它可以被简单的 let 语句所代替


}