
// if let 
// if let 是一种赋值表达式
// 条件成立，将值赋给某个变量
// 实际上是只有一种匹配模式的语法糖

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

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... other state
}

// 背景：在 1999 年到 2008 年之间，美国在25美分硬币的一侧为50 个州采用了不同的设计
// 其他类型的硬币都没有类似的各州的设计，所以只有25美分拥有这个特点。我们可以通过在Quarter变体中添加一个UsState值
// UsState 也是一个枚举
enum Coin {
    // 1 美分
    Penny,
    // 5 美分
    Nickel,
    // 10 美分
    Dime,
    // 25 美分
    Quarter(UsState),
}

fn main() {
    // 如果只想匹配 MyNum 中的一种情况
    // 使用 match 可以这样写
    let y = MyNum::One;
     match y {
        MyNum::One => {
            println!("one");
        },
        // 为了满足match表达式穷尽性的需求
        // 我们不得不在处理完这唯一的变体后额外加上一句_ => ()，这显得十分多余
        _ => (),
    };

    // println!("current x is {}", x);

    // 只有一种情况的 match 表达式，可以等价为 if let
    // 
    // if let 后面跟着一对以 = 隔开的模式与表达式
    // 等号左边是模式，右边是表达式
    // 它们所起的 作用与 match 中的完全相同：表达式对应match中的输入，而模式则对应第一个分支

    if let MyNum::One = y {
        println!("one")
    }

    // 还可以在 if let 中搭配使用 else。else 所关联的代码块在 if let 语句中扮演的角色，
    // 就如同match中 _ 模式所关联的代码块一样

    // 还是以美分那个枚举做例子，我们只想处理与 25 美分相关的情况
    // 使用 match 就可以这样写

    let coin_1 = Coin::Quarter(UsState::Alabama);
    let coin_2 = Coin::Dime;
    let mut count:i32 = 0;
    // 需要指定 match 表达式的返回值类型
    // let y: i32 = match coin_1 {
    //     Coin::Quarter(state) => {
    //         println!("state quatar from {:?}", state);
    //         25
    //     },
    //     Coin::Dime => 10,
    //     _ => {
    //         0
    //     },
    // };

    // println!("y is {}", y)

    // 使用 if let与 else 表达式改写
    if let Coin::Quarter(state) = coin_1 {
        println!("state quatar from {:?}", state);
    } else {
        count += 1;
    }
    println!("count is {}", count);

    
}