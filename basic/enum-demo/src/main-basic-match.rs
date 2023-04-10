
// match - 模式匹配

// 定义一个枚举
// 美国硬币规格
enum Coin {
    // 1 美分
    Penny,
    // 5 美分
    Nickel,
    // 10 美分
    Dime,
    // 25 美分
    Quarter,
}

fn value_in_cents(coin: Coin) -> i32 {
    // 以 Coin 这个枚举变体为 match 表达式
    // match 表达式中的每个分支被称为 arm

    // match 关键字后面会跟随一个表达式，也就是 coin
    // match 表达式与 if 表达式的使用十分相似，但这里有个巨大的区别:
    // 在 if 语句中，表达式需要返回一个布尔值，而这里的表达式则可以返回任何类型

    // 当这个 match 表达式执行时，它会将产生的结果值依次与每个分支中的模式相比较。
    // 假如模式匹配成功，则与该模式相关联的代码就会被继续执行
    match coin {
        // 这里是 match 的分支语句，每个分支由模式和它所关联的代码组成
        // 下面这个分支采用了值 Coin::Penny 作为模式，并紧跟着一个 => 运算符将模式和这个模式下要执行的代码分开
        // 每个分支下所关联的代码也是一个表达式， 这个表达式所执行的结构，会作为整个 match 表达式的值返回
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
}

fn value_in_cents_2(coin: Coin) -> i32 {
    match coin {
        // 如果匹配分支只有一行代码，那么我们可以不使用花括号包裹代码
        // 假如我们想要在一个匹配分支中包含多行代码，那么就可以使用花括号将它们包裹起来
        Coin::Penny => {
            println!("Lucky penny!");
            return 1;
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
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

enum Coin_2 {
    // 1 美分
    Penny,
    // 5 美分
    Nickel,
    // 10 美分
    Dime,
    // 25 美分
    Quarter(UsState),
}


fn value_in_cents_3(coin: Coin_2) -> i32 {
    match coin {
        Coin_2::Penny => {
            println!("Lucky penny!");
            return 1;
        },
        Coin_2::Nickel => 5,
        Coin_2::Dime => 10,
        // 绑定值的模式
        // 匹配分支可绑定被匹配对象的部分值，而这也正是我们用于从枚举变体中提取值的方法
        // 因为枚举可以绑定类型，绑定完类型以后，在使用过程中还可以传入具体的值，因此可以通过模式匹配获取和这个枚举绑定的值
        Coin_2::Quarter(state) => {
             println!("state quatar from {:?}", state);
             25
        },
    }
}

fn main() {
    let res_1 = value_in_cents(Coin::Dime);
    let res_2 = value_in_cents_2(Coin::Penny);
    println!("coin_1 {}", res_1);
    println!("coin_2 {}", res_2);

    // 调用带绑定值的模式匹配函数
    // Coin_2::Quarter(UsState::Alaska) 就会作为 coin 的值传入函数。
    // 这个值会依次与每个分支进行匹配，一直到 Coin::Quarter(state) 模式才会终止匹配。
    // 这时，值 UsState::Alaska 就会被绑定到变量 state上。 接着，我们就可以在println! 表达式中使用这个绑定了
    value_in_cents_3(Coin_2::Quarter(UsState::Alaska));
    value_in_cents_3(Coin_2::Quarter(UsState::Alabama));
}