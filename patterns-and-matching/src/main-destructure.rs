// 可用的模式语法
// 模式匹配的各种使用方法

fn main() {
    // 1. 匹配字面量
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    // 因为 x 的值是 1，所以匹配的是第一个，因此打印出 one

    // 2. 匹配命名变量
    // 命名变量（named variable）是一种可以匹配任何值的不可失败模式
    // 值得一提的是，当我们在 match 表达式中使用命名变量时，情况可能会变得稍微有些复杂：
    // 由于 match 开启了一个新的作用域，所以被定义在 match 表达式内作为模式一部分的变量会覆盖掉 match 结构外的同名变量
    // 正如覆盖其他普通变量一样

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x)
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    // x 为 Some(5)，变体内保存的值为 5，因此不会与第一个分支模式匹配
    // 第二个分支模式为 Some(y)，因此可以匹配 Some 变体中携带的任意值，所以这里匹配成功
    // 因为我们处在 match 表达式创建的新作用域中，所以这里的 y 是一个新的变量
    // 而不是我们在程序起始处声明的那个存储了 10 的 y
    // 这个新的 y 的绑定能够匹配 Some 中的任意值，而 x 是 Some(5)
    // 因此，新的y会被绑定到 x 变量中 Some 内部的值。由于这个值是 5，所以 y 会被赋值为 5
    // 这个匹配的执行结果是：Matched, y = 5

    // 如果 x 为 None，那么不会匹配前两个分支就会执行第三个分支
    // 在第三个分支创建的新的作用域中，我们没有引入任何新的变量，或者说是名为 x 的变量，这个表达式使用的x没有被任何变量所覆盖
    // 所以这里的 x 就是程序一开始创建的 x，所以第三个分支的执行结果是：Default case, x = 5
    //
    // match 表达式创建出来的作用域会随着当前表达式的结束而结束，而它内部的 y 自然也无法幸免
    // 代码最后的println! 打印出 at the end: x = Some(5), y = 10


    // 3. 多重模式
    // 我们可以在match表达式的分支匹配中使用 | 来表示或 （or）的意思，它可以被用来一次性地匹配多个模式

    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => ()
    }

    // 第一个分支可以匹配 x 为 1 或者 2 的情况

    // 4. 使用...来匹配值区间
    // 我们可以使用...来匹配闭区间的值，要匹配的值处于这个区间，这个模式对应的分支就会得到执行
    // 注意：... 范围模式已经被废弃了，使用 ..= 来代替 ...
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else")
    }

    // 当 x 为 1、2、3、4、5 中的任意一个值，都会与第一个分支匹配
    // 1..=5 表示从 1 到 5 任意一个数字，这个语法在表达类似含义时要比使用|运算符更为方便
    // 对于某个区间来说，我们不需要将所有的值都列举出来

    // 范围模式只被允许使用数值或 char 值来进行定义，因为编译器需要在编译时确保范围的区间不为空
    // 而 char 和 数值正是 Rust 仅有的可以判断区间是否为空的类型
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else")
    }

    // x 为 c，和第一个分支匹配，所以打印输出：early ASCII letter

    // 5. 使用解构来分解值
    // 我们可以使用模式来分解结构体、枚举、元组或引用，从而使用这些值中的不同部分

    // 5.1 解构结构体
    // 可以使用解构的方式从结构体中获取字段值
    let point = Point {
        x: 0,
        y: 7
    };

    let Point {x: a, y :b} = point;

    assert_eq!(0, a);
    assert_eq!(7, b);

    // 将结构体中的字段解构为独立的变量
    println!("a is {}", a);
    println!("b is {}", b);

    // 上面这段代码创建了 a 和 b 两个变量，它们分别匹配了 p 结构体中字段 x 和 y的值
    // 这个例子说明模式中的变量名并不需要与结构体的字段名相同，x 和 y 只是用来匹配结构体中的字段，匹配成功后，将字段值赋值给新的变量
    // 实际上我们常常倾向于采用与字段名相同的变量名，因为这样可以方便我们记住哪一个变量来自哪一个字段

    let Point {x, y} = point;
    // 上面的解构语句是 let Point { x: x, y: y } = p 的简写

    // x is 0
    // y is 7
    println!("x is {}", a);
    println!("y is {}", b);

    // 除了为所有字段创建变量，我们还可以在结构体模式中使用字面量来进行解构
    // 这一技术使我们可以在某些字段符合要求的前提下再对其他字段进行解构

    // 使用 match 表达式根据结构体字段是否满足某些条件来决定是否进行解构
    match point {
        Point {x, y: 0} => println!("On the x axis at {}", x),
        Point {x: 0, y} => println!("On the y axis at {}", y),
        Point {x, y} => println!("On neither axis: ({}, {})", x, y)
    }
    // 第一个匹配条件：要求结构体中 y 为 0，x 任意，并在其作用域内创建了新的变量 x
    // 第二个匹配条件：要求结构体中 x 为 0，y 任意，并在其作用域内创建了新的变量 y
    // 第三个匹配条件：要求结构体中 x 和 y 为任意值，并在其作用域内创建了新的变量 x 和 y
    // 因为 point 中的 x 为 0，y 为 7，所以第二个匹配条件成立，执行代码，最终打印输出：On the y axis at 7


    // 5.2 解构枚举
    // 使用模式匹配来匹配枚举中的变体

    // let msg = Message::ChangeColor(0, 255, 255);
    // let msg = Message::Quit;
    let msg = Message::Move {x: 100, y: 90};

    match msg {
        Message::Write(text) => println!("Text message: {}", text),
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        },
        Message::Move {x, y} => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x,
                y
            );
        },
        Message::ChangeColor(r, g, b) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        },
    }

    // 对 Message 枚举的每个变体进行匹配
    // msg 的值是 Message 中的 ChangeColor 变体，所以会匹配最后一个条件，所以输出是：Change the color to red 0, green 255, and blue 255
    // 对于不含有数据的枚举变体而言，比如Message::Quit，我们无法从其内部进一步解构出其他值。因此，针对这种变体的模式不会创建出任何变量，它只能被用于匹配字面量Message::Quit的值

    // 对类似于结构体的枚举变体而言，比如 Message::Move，我们可以采用类似于匹配结构体的模式，在变体名后使用花括号包裹那些列出的字段变量名
    // 而这些分解出来的部分同样可以被用于匹配分支的代码块中

    // 对类似于元组的枚举变体而言，比如在元组中持有一个元素的 Message::Write， 以及在元组中持有 3 个元素的Message::ChangeColor
    // 我们使用的模式与匹配元组时用到的模式非常相似。模式中的变量数目必须与目标变体中的元素数目完全一致

    // 5.3 解构嵌套的结构体和枚举
    // 匹配语法除了用于单层的结构体或者，还可以被用于嵌套的结构中

    let new_msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));

    match new_msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b);
        },
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to hue {}, saturation {}, and value {}", h, s, v);
        },
        _ => ()
    }

    // 在这段 match 表达式中，第一个分支的模式匹配了含有 Color::Rgb 变体的 Message::ChangeColor 枚举变体，并绑定了 3 个内部的i32值
    // 第二个分支的模式则匹配了含有 Color::Hsv 变体的 Message::ChangeColor 枚举变体
    // 我们可以在单个 match 表达式中指定这些较为复杂的条件，即便它们需要同时匹配两个不同的枚举类型


    // 5.4 解构元组和结构体
    // 我们甚至可以按照某种更为复杂的方式来将模式混合、匹配或嵌套在一起。下面示例的元组中嵌套了结构体与其他元组，但我们依然
    // 能够同时解构出这个类型所有的基本元素
    let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });
    let ((a, b), Point {x, y}) = ((100, 200), Point {x: 50, y: 100});
    // feet is 3, inches is 10
    println!("feet is {}, inches is {}", feet, inches);

    // 6. 忽略模式中的值
    // 在某些场景下忽略模式中的值是有意义的，例如在 match 表达式的最后一个分支中，代码可以匹配剩余所有可能的值而又不需要执行什么操作
    // 有几种不同的方法可以让我们在模式中忽略全部或部分值：使用 _ 模式、在另一个模式中使用_模式、使用以下划线开头的名称，或者使用 .. 来忽略值的剩余部分

    // 6.1 使用_忽略整个值
    // 我们曾经将下划线_作为通配符模式来匹配任意可能的值而不绑定值本身的内容。虽然 _ 模式最常被用在match表达式的最后一个分支中，但实际上我们可以把它用于包括函数参数在内的一切模式中


    // 使用 _ 忽略函数的参数
    // This code only uses the y parameter: 4
    foo(3, 4);
    // 虽然我们传入了第一个参数，但是被忽略了
    // 当不再需要函数中的某个参数时，你可以修改函数签名来移除那个不会被使用的参数
    // 忽略函数参数在某些情形下会变得相当有用。
    // 例如，假设你正在实现一个 trait，而这个 trait 的方法包含了你不需要的某些参数
    // 在这种情形下，可以借助忽略模式来避免编译器产生未使用变量的警告

    // 6.2 使用嵌套的_忽略值的某些部分
    // 我们还可以在另一个模式中使用_来忽略值的某些部分
    // 例如，在要运行的代码中，当你需要检查值的某一部分且不会用到其他部分时，就可以使用这一模式
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
                setting_value = new_setting_value;
            }
    }

    // setting is Some(5)
    println!("setting is {:?}", setting_value);
    // 上面的 match 表达式中，我们只需要检查 setting_value, new_setting_value 是否是 Some 变体，而不关心其持有的值
    // 因此使用 _ 忽略 Some 变体中的值
    // 剩余所有的场景，也就是 setting_value 或 new_setting_value 中任意一个是 None 时，都可以与第二个分支中的模式 _ 匹配。我们希望在这个分支中将 setting_value 的值修改为 new_setting_value

    // 在一个模式中多次使用下划线来忽略多个特定的值
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }
    // 这段代码会忽略值 4 与 16，并打印出Some numbers: 2, 8, 32
    //

    // 6.3 通过以_开头的名称来忽略未使用的变量
    // Rust会在你创建出一个变量却又没有使用过它时给出相应的警告，因为这有可能是程序中的 bug
    // 但在某些场景下，创建一个暂时不会用到的变量仍然是合理的，比如进行原型开发时或开始一个新的项目时
    // 为了避免 Rust 在这些场景中因为某些未使用的变量而抛出警告，我们可以在这些变量的名称前添加下划线

    let y = 10;
    let _x = 5;

    // 编译这段代码会警告我们没有使用过变量 y，但却不会警告我们没有使用那个以下划线开头的变量 _x
    // 使用以下划线开头的变量名与仅仅使用_作为变量名存在一个细微的差别：_x 语法仍然将值绑定到了变量上，而_则完全不会进行绑定
    // 示例：
    // let s = Some(String::from("Hello!"));
    // if let Some(_s) = s {
    //     println!("found a string");
    // }
    // println!("{:?}", s);
    // 编译上面的代码，报错信息如下：
    // error[E0382]: borrow of partially moved value: `s`
    //    --> src\main-destructure:262:22
    //     |
    // 259 |     if let Some(_s) = s {
    //     |                 -- value partially moved here
    // ...
    // 262 |     println!("{:?}", s);
    //     |                      ^ value borrowed here after partial move
    // 以下划线开头的变量实际上还是一个变量，因此会发生所有权的转移
    // 由于变量 s 中的值被移动到了变量 _s 中，所以我们会在随后使用 s 时违背所有权规则
    let s = Some(String::from("Hello!"));
    if let Some(_) = s {
        println!("found a string");
    }
    // Some("Hello!")
    println!("{:?}", s);
    // 单独使用下画线不会绑定值
    // 由于我们没有将s绑定到任何物体上，所以这段代码可以正常地运行

    // 6.4 使用 .. 忽略值的剩余部分
    // 对于拥有多个部分的值，我们可以使用..语法来使用其中的某一部分并忽略剩余的那些部分
    // 这使我们不必为每一个需要忽略的值都添加对应的_模式来进行占位。..模式可以忽略一个值中没有被我们显式匹配的那些部分
    let origin = Point2 {
        x: 0, y: 0, z: 0
    };

    match origin {
        Point2 {x, ..} => {
            println!("x is {}", x)
        }
    }
    // 使用 .. 忽略 Point 中除 x 之外的所有字段
    // 这段代码在分支模式中首先列出了 x 变量，并接着列出了一个 .. 模式
    // 这种语法要比具体地写出y: _ 和z: _ 稍微便捷一些
    // 尤其是当我们需要操作某个拥有大量字段的结构体，却只需要使用其中的某一两个字段时

    // .. 语法会自动展开并填充任意多个所需的值
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
    // 只匹配元组中的第一个值和最后一个值，而忽略其他值这段代码使用 first 和 last 来分别匹配了元组中的第一个值和最后一个值
    // 而它们之间的 .. 模式则会匹配并忽略中间的所有值
    // 注意：使用 .. 必须不能出现任何歧义
    // 如果模式中需要匹配的值或需要忽略的值是无法确定的，那么 Rust 就会产生一个编译时错误
    // 示例：
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (.., second, ..) => {
            println!("Some numbers: {}", second)
        },
    }
    // 报错如下：error: `..` can only be used once per tuple pattern
    //    --> src\main-destructure:314:22
    //     |
    // 314 |         (.., second, ..) => {
    //     |          --          ^^ can only be used once per tuple pattern
    //     |          |
    //     |          previously used here

    // 虽然我们的本意是想匹配 numbers 中的第二个元素，但是 Rust 却无法确定 second 的位置，因此无法确定忽略的元素
}


struct Point2 {
    x: i32,
    y: i32,
    z: i32,
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}


enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct Point {
    x: i32,
    y: i32
}


enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32)
}
enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}