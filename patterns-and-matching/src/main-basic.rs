// Rust 中的模式匹配

// 模式是 Rust 中一种用来匹配类型结构的特殊语法，它时而复杂，时而简单
// 将模式与 match 表达式或其他工具配合使用可以更好地控制程序流程
// 一个模式通常由以下组件组合而成：
// 1. 字面量
// 2. 解构的数组、枚举、结构体或元组
// 3. 变量
// 4. 通配符
// 5. 占位符
// 这些组件可以描述我们将要处理的数据形状，而数据形状则可以被用来匹配值，进而判断出程序能否获得可供后续代码处理的正确数据
// 模式被用来与某个特定的值进行匹配。如果模式与值匹配成功，那么我们就可以在代码中使用这个值的某些部分

// Rust 中的模式匹配有下面几种形式：
// match 分支
// if let 条件表达式
// while let 条件循环
// for 循环
// let 语句
// 函数的参数

fn main() {

    // match 表达式
    //
    // match 表达式在形式上由 match 关键字、待匹配的值，以及至少一个匹配分支组合而成
    // 而分支则由一个模式及匹配模式成功后应当执行的表达式组成：
    // match 值
    // {
    //    模式 => 表达式,
    //    模式 => 表达式,
    //    模式 => 表达式,
    // }
    // match表达式必须穷尽 （exhaustive）匹配值的所有可能性。为了确保代码满足这一要求，我们可以在最后的分支处使用全匹配模式
    // 例如，变量名可以被用来覆盖所有剩余的可能性，一个能够匹配任何值的变量名永远不会失败

    // 另外，还有一个特殊的_模式可以被用来匹配所有可能的值，且不将它们绑定到任何一个变量上，因此，这个模式常常被用作匹配列表中的最后一个分支
    // 当你想要忽略所有未被指定的值时，_模式会非常有用

    // println!("Hello, world!");

    let name = Some("Jack");
    match name {
        Some(s) => {
            println!("the lucky man is {}", s);
        },
        None => {
            println!("There are no lucky man");
        }
    }

    let num = 10;

    match num {
        1 => {
            println!("the lucky number is {}", 1);
        },
        10 => {
            println!("the lucky man is {}", 10);
        },
        _ => {
            println!("There are no lucky man");
        }
    }


    // if let条件表达式
    // 我们经常把 if let 表达式当作只匹配单个分支的 match 表达式来使用
    // 但实际上if let还能够添加一个可选的 else 分支，如果 if let 对应的模式没有匹配成功，那么 else 分支的代码就会得到执行

    // 另外，我们同样可以混合使用 if let、else if 及 else if let 表达式来进行匹配，相较于单次只能将一个值与模式
    // 比较的 match 表达式来说，这种混合语法可以提供更多的灵活性，并且一系列 if let、else if、else if let 分支中的条件也不需要彼此相关

    // 基本的 if let 表达式

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else {
        println!("No favorite color");
    }

    // 组合 if let、else if 及 else if let

    // 如果用户明确指定了一个偏爱的颜色，那么我们就将它直接用作背景色；否则，我们会继续判断当天是否是星期二
    // 并在条件满足时采用绿色作为背景色，如果条件匹配再次失败，就进而判断用户给出的字符串是否能够被成功解析为数字
    // 当数字解析成功时，我们会根据数字的大小选用紫色或橙色
    // 如果以上所有条件均不满足，那么我们就选择蓝色作为背景色

    // 根据我们设定的变量的值，下面的匹配结果最终输出为：Using purple as the background color
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day");
    } else if let Ok(age) = age {

        // 和match分支类似，if let 分支能够以同样的方式对变量进行覆盖
        // if let Ok(age) = age 这条语句中引入了新的变量age来存储Ok变体中的值
        // 而它覆盖了右侧的同名变量。这意味着我们必须把判断条件if age > 30 放置到匹配成功后执行的代码块中，而不能把这两个条件组合成if let Ok(age) = age && age >30
        // 因为覆盖了同名变量的 age 只有在花括号后的新作用域中才会变得有效
        if age > 30 {
            println!("Using purple as the background color")
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // 与 match 表达式不同，if let 表达式的不利之处在于它不会强制开发者穷尽值的所有可能性
    // 即便我们省略了随后可选的 else 块，并因此遗漏了某些需要处理的情形，编译器也不会在这里警告我们存在可能的逻辑性缺陷


    // while let
    // 条件循环 while let 的构造与 if let 十分类似，但它会反复执行同一个模式匹配直到出现失败的情形
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);


    // 3
    // 2
    // 1
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // 上面的示例会依次打印出3、2、1。其中的 pop 方法会试图取出动态数组的最后一个元素并将它包裹在 Some(value) 中返回
    // 如果动态数组为空，则 pop 返回 None
    // while 循环会在 pop 返回 Some 时迭代执行循环体中的代码
    // 并在 pop 返回 None 时结束循环。使用 while let 便可以将栈中的每个元素逐一弹出了

    // for 循环
    // 我们同样可以在 for 循环内使用模式，for 语句中紧随关键字 for 的值就是一个模式，比如 for x in y 中的 x

    // for 循环中解构元组
    let v1 = vec!['a', 'b', 'c'];
    for (index, value) in v1.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // 输出如下：
    // a is at index 0
    // b is at index 1
    // c is at index 2

    // enumerate 方法来作为迭代器的适配器，本身返回的也是一个迭代器
    // 它会在每次迭代过程中生成一个包含值本身及值索引的元组
    // 例如，首次调用enumerate会产生元组 (0, 'a')，当我们将这个值与模式 (index,value) 进行匹配时
    // index 就会被赋值为 0 ， 而 value 则会被赋值为'a'，这也就是第一行输出中的内容


    // let 语句
    // let 赋值语句实际上也是模式
    // 更正式的let语句的定义如下所示：
    // let PATTERN = EXPRESSION;

    // 在类似于 let x = 5; 的语句中，单独的变量名作为最朴素的模式被放于PATTERN对应的位置
    // Rust会将表达式与模式进行比较，并为所有找到的名称赋值
    // 因此，在 let x = 5; 这个示例中，x 作为模式表达的含义是将此处匹配到的所有内容绑定至变量x
    // 因为x就是整个模式本身，所以它实际上意味着无论表达式会返回什么样的值，我们都可以将它绑定至变量x中

    // 解构元组，一次性创建三个变量
    let (x, y, z) = (1, 2, 3);

    // 在上面的示例中使用模式来匹配一个元组，由于 Rust 在比较值 (1, 2, 3) 与模式 (x, y, z) 时发现它们是一一对应的
    // 所以 Rust 会最终将 1、2、3 分别绑定至 x、y、z 上
    // 可以将这个元组模式理解为嵌套的3个独立变量模式

    // 注意，如果模式中元素的数量与元组中元素的数量不同，那么整个类型就会匹配失败，进而导致编译错误

    // let (x, y) = (1, 2, 3);
    // 编译错误信息如下：
    // error[E0308]: mismatched types
    //    --> src\main-basic:177:9
    //     |
    // 177 |     let (x, y) = (1, 2, 3);
    //     |         ^^^^^^   --------- this expression has type `({integer}, {integer}, {integer})`
    //     |         |
    //     |         expected a tuple with 3 elements, found one with 2 elements

    //  如果你需要忽略元组中的某一个或多个值，那么我们可以使用_或..语法，示例如下：
    let (x, y, _) = (1, 2, 3);

    // 函数的参数
    // 函数的参数同样也是模式
    // 调用一个函数，并传入参数，这就是一个模式匹配的过程
    add_five(5);

    let points = (10, 5);

    // Current location: (10, 5)
    print_coordinates(&points);

}


// 函数的签名中的变量 x 就是一个模式
fn add_five(x: i32) -> i32 {
    x + 5
}


// 从一个元组中解构出参数
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
