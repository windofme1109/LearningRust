// @绑定
// @运算符允许我们在测试一个值是否匹配模式的同时创建存储该值的变量

fn main() {
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable);
        },
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range");
        },
        Message::Hello {id} => {
            println!("Found some other id: {}", id);
        }
    }

    // 模式中测试一个值的同时使用 @ 来绑定它
    // 运行该示例会打印出 Found an id in range: 5
    // 通过在 3...7 之前使用 id_variable @
    // 我们在测试一个值是否满足区间模式的同时可以捕获到匹配成功的值

    // 第二个分支仅仅在模式中指定了值的区间，而与这个分支关联的代码块中却没有一个包含了 id 字段的值的可用变量
    // id 字段的值可以是 10、11或12，但随后的代码却无法得知匹配值具体是哪一个，由于我们没有将这个值存储在某个变量中，所以该模式分支的代码无法使用id字段中的值
    // 这个 id 可以理解为是一个占位符，仅仅是取出 Hello 结构体中的 id 值，然后测试其是否在指定的区间内


    // 最后一个分支的模式则指定了一个没有区间约束的变量，这个变量可以被用于随后的分支代码中
    // 因为这里的代码使用了结构体字段简写语法。这个分支的匹配没有像前两个分支那样对 id 值执行任何测试，因此所有的值都可以匹配这个模式。
    //
    //
    // @使我们可以在模式中测试一个值的同时将它保存到变量中
}

enum Message {
    Hello {id: i32}
}