
// trait（特征）被用来向Rust编译器描述某些特定类型拥有的且能够被其他类型共享的功能
// 它使我们可以以一种抽象的方式来定义共享行为。我们还可以使用 trait 约束来将泛型参数指定为实现了某些特定行为的类型

// trait 可以理解为是其他语言的接口（interface），但是功能页不尽相同

// 定义一个 trait

use std::fmt::{Display, Debug};

// 使用 trait 关键字声明一个 trait
// Summary trait 声明为公共的才能被其他库用于具体实现
pub trait Summary {
    // trait 中只定义函数的名称、参数和返回值，没有具体实现
    // 具体的函数实现，要放到使用这个 trait 的类型中进行
    // 具体而言，就是任何想要实现这个 trait 的类型都需要为给 summarize 提供自定义行为
    // 编译器会确保每一个实现了Summary trait 的类型都定义了与这个签名完全一致的 summarize 方法
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// 实现（implements ）一个 trait
impl Summary for NewsArticle {
    //  Summary trait 方法中 summarize 
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// impl Display for Tweet {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
//     }
// }

// 注意，实现 trait 有一个限制：只有当 trait 或类型定义于我们的库中时，我们才能为该类型实现对应的 trait
// Summary trait 和 Tweet 类型定义在一个模块中，Tweet 就可以实现 Summer 这个 trait
// 而自定义类型 Tweet，也实现标准库中的Display trait
// 标准库中的 Vec<T> 也能实现自定义的 Summary trait

// 我们不能为外部类型实现外部 trait。例如，我们不能在现在的这个库（package）内为 Vec<T> 实现 Display trait
// 因为 Display 与 Vec<T> 都被定义在标准库中，而没有定义在 aggregator 库中
// 这个限制被称为孤儿规则 （orphan rule）
// 之所以这么命名是因为它的父类型没有定义在当前库中
// 这一规则也是程序一致性 （coherence）的组成部分
// 它确保了其他人所编写的内容不会破坏到你的代码
// 反之亦然。如果没有这条规则，那么两个库可以分别对相同的类型实现相同的 trait，Rust将无法确定应该使用哪一个版本

// 示例如下
// error[E0117]: only traits defined in the current crate can be implemented for types defined outside of the crate
// impl<T> Display for Vec<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
//     }
// }

// 报错信息显示：只有定义在当前 crate 下的 trait，才能被外部 crate 中的类型实现

// 要么外部类型实现当前 crate 下定义的 trait
// 要么当前 crate 下定义的类型实现外部 trait
// 或者是当前 crate 下定义的类型实现当前 crate 下定义的 trait



// 带有默认行为的 trait
pub trait Summary_2 {
    // 定义函数过程中，给函数增加行为，这个就是 summarize_2 的默认行为
    // 实现了这个 trait 的类型，如果没有覆写这个方法，那么这个 summarize_2 函数就是要默认行为
    fn summarize_2(&self) -> String {
        String::from("(Read more...)")
    }

    // 默认实现中调用相同 trait 中的其他方法，哪怕这些方法没有默认实现。
    // 基于这一规则，trait可以在只需要实现一小部分方法的前提下，提供许多有用的功能

    fn summarize_author(&self) -> String;

    fn summarize_3(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary_2 for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

impl Summary_2 for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}


// 将 trait 作为参数
// 参数 item 可以是任何实现了Summary trait 的类型
pub fn notify(item: impl Summary) {
    // Summary trait 的任何方法
    println!("Breaking news! {}", item.summarize());
}

// 上面定义函数的方式，实际上是一种语法糖，完整使用 trait 作为参数，形式如下
// 完整形式可以理解为是对泛型 T 使用 Summary 进行约束
// pub fn notify<T: Summary>(item: T) {
//     println!("Breaking news! {}", item.summarize());
// }

// 什么时候使用完整形式，什么时候使用语法糖形式呢
// 举个例子：假设我们需要接收两个都实现了 Summary trait 的参数，那么使用 impl Trait 的写法如下所示：
// pub fn notify(item1: impl Summary, item2: impl Summary) {}
// 只要 item1 和 item2 可以使用不同的类型（ 同时都实现了Summary），
// 如果想让 item1 和 item2 只能接收同一类型，那么我们就得使用完整形式；
// pub fn notify<T: Summary>(item1: T,item2: T) {}

// 通过+语法来指定多个trait约束
// 假如 notify 函数需要在调用 summarize 方法的同时显示格式化后的item
// 那么 item 就必须实现两个不同的 trait：Summary 和 Display。我们可以使用+语法来实现
pub fn notify(item: impl Summary + Display) {}
// 完整形式（泛型约束）
pub fn notify<T: Summary + Display>(item: T) {}

// 使用where从句来简化trait约束

// 使用过多的 trait 约束也有一些缺点：
// 因为每个泛型都拥有自己的 trait 约束，定义有多个泛型参数的函数可能会有大量的 trait 约束信息需要被填写在函数名与参数列表之间
// 这往往会使函数签名变得难以理解。为了解决这一问题，Rust提供了一个替代语法
// 使我们可以在函数签名之后使用 where 从句来指定trait约束
// 可以使用 where 进行改写下面的函数
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}

// 使用where语句改写为：
fn some_function<T, U>(t: T, u: U) -> i32
where T: Display + Clone, U: Clone + Debug {}

// 返回实现了 trait 的类型
// 我们同样可以在返回值中使用 impl Trait 语法，用于返回某种实 现了 trait 的类型

// 通过在返回类型中使用 impl Summary，
// 我们指定 returns_summariazable 函数返回一个实现了 Summary trait 的类型作为结果，
// 而无须显式地声明具体的类型名称
// 在本例中 ，returns_summarizable 返回了一个 Tweet，但调用者却无法知晓这一信息
// 指定了返回值为 trait，那么只要返回实现了这个 trait 的类型就可以，而不用管具体是什么类型

fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know,people"),
        reply: false,
        retweet: false,
    }
}

// 你只能在返回一个类型时使用impl Trait

fn main() {
    let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know,people"),
            reply: false,
            retweet: false,
        };
        println!("1 new tweet: {}", tweet.summarize());


        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best
            hockey team in the NHL."),
        };
        // 调用 trait 的默认实现方法
        println!("New article available! {}", article.summarize_2());

        // 调用 trait 的默认实现方法
        println!("1 new tweet: {}", tweet.summarize_3());

        // trait 作为参数
        // 需要传入实现了 Summary 这个 trait 的类型
        notify(article);
}