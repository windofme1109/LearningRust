// 使用匹配守卫添加额外条件
// 匹配守卫（match guard）是附加在 match 分支模式后的 if 条件语句，分支中的模式只有在该条件被同时满足时才能匹配成功
// 相比于单独使用模式，匹配守卫可以表达出更为复杂的意图

// 匹配守卫的条件可以使用模式中创建的变量



fn main() {

    let num = Some(4);
    // let num = Some(10);
    match num {
        Some(x) if x < 5 => {
            println!("less than five: {}", x);
        },
        Some(x) => println!("{}", x),
        None => (),
    }

    // 上面的代码会在运行时打印出 less than five: 4
    // num能够与第一个分支中的模式匹配成功，因为 Some(4) 与 Some(x) 匹配
    // 随后的匹配守卫则会检查模式中创建的变量 x 是否小于 5
    // 由于 num 同样满足这一条件，所以我们最终执行了第一个分支中的代码

    // 假设 num 的值是 Some(10)，那么第一个分支中的匹配守卫则无法成立，因为 10 大于 5
    // Rust 会接着进入第二个分支继续比较并最终匹配成功。因为第二个分支中没有匹配守卫，所以它能够匹配包含任意值的 Some 变体

    // 使用匹配守卫解决模式中变量覆盖的问题
    // 如果匹配额表达式中，在其作用域内，引入了与外部同名的变量，那么在内部是无法使用外部的同名变量的，只能使用内部引入的变量
    // let x = Some(5);
    //     let y = 10;
    //     match x {
    //         Some(50) => println!("Got 50"),
    //         Some(y) => println!("Matched, y = {:?}", y),
    //         _ => println!("Default case, x = {:?}", x)
    //     }
    //
    //     println!("at the end: x = {:?}, y = {:?}", x, y);

    // 第二个表达式引入了变量 y，因此在其作用域内，这个 y 就会覆盖外面的 y，也就是说，Some 变体持有什么值，这个 y 就是什么值，我们无法在这作用域内使用外部的 y

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        Some(z) => println!("Matched, z = {:?}, x = {:?}, y = {}", z, x, y),
        _ => println!("Default case, x = {:?}", x),
    }
    // at the end: x = Some(5), y = 10
    println!("at the end: x = {:?}, y = {:?}", x, y);

    // 由于第二个分支的模式中没有引入新的变量 y，所以随后的匹配守卫可以正常地在条件判断中使用外部变量 y
    // 这个分支使用了 Some(n) 而不是 Some(y) 来避免覆盖 y 变量
    // 这里新创建出来的 n 变量不会覆盖外部的任何东西，因为 match 外部没有与 n 同名的变量
    // 匹配守卫 if n == y 不是一个模式，所以它不会引入新的变量
    // 因为这个条件中的 y 就是来自表达式外部的 y，而不是之前示例中覆盖后的 y，所以我们才能够比较 n 和 y 的值是否相同


    // 匹配守卫中使用或运算符 | 来指定多重模式
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => {
            println!("yes");

        },
        _ => println!("no")
    }

    // 第一个分支中的匹配条件要求 x 的值等于 4、5 或 6
    // 并且 要求 y 为 true，当你运行这段代码时，虽然 x 存储的 4 满足第一个分支中的模式要求
    // 但却无法满足匹配守卫的条件 if y，所以第一个分支的匹配失败。接着，代码会在第二个分支处匹配成功，并打印出 no
    // 之所以会出现这样的结果，是因为 if 条件对于整个模式 4 | 5 | 6 都是有效的，而不仅仅只针对最后的那个值 6
    // 优先级关系：
    // (4 | 5 | 6) if y => ...
    // 4 | 5 (6 if y) => ...

    // 先执行或表达式，在执行 if 表达式
    // 如果是第二种，那么由于 x 为 4，那第一个条件成立，就会进入第一个匹配表达式，进入打印出 yes，而实际上打印的是 no
}