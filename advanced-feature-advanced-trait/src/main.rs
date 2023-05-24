// 高级 trait

// 1. 在 trait 的定义中使用关联类型指定占位类型
// 关联类型（associated type）是 trait 中的类型占位符
// 它可以被用于 trait 的方法签名中
// trait 的实现者需要根据特定的场景来为关联类型指定具体的类型
// 通过这一技术，我们可以定义出包含某些类型的 trait，而无须在实现前确定它们的具体类型是什么

// 标准库中的 Iterator trait 就包含关联类型
pub trait Iterator {
    // Item 就是 Iterator 的关联类型
    type Item;
    // 这里的类型 Item 是一个占位类型，而 next 方法的定义则表明它会返回类型为Option<Self::Item>的值
    // Iterator trait 的实现者需要为 Item 指定具体的类型，并在实现的 next 方法中返回一个包含该类型值的 Option
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
} 
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

// 关联类型与泛型的区别
// 从关联类型的用法来看，其与泛型类似，都是使用的时候指定具体的类型
// 那么关联类型与泛型的区别是什么呢
// 假设我们给 trait 添加泛型，如下所示：
// pub trait Iterator<T> {
//     fn next(&mut self) -> Option<T>;
// }

// 假设我们的 trait 是接收泛型参数，那么就需要在每次实现该 trait 的过程中标注类型
// 因为我们既可以实现 Iterator<String> for Counter，也可以实现其他任意的迭代类型
// 从而使得 Counter 可以拥有多个不同版本的 Iterator 实现

// 当trait拥有泛型参数时，我们可以为一个类型同时多次实现 trait，并在每次实现中改变具体的泛型参数
// 那么当我们在 Counter 上使用 next 方法时，也必须提供类型标注来指明想要使用的 Iterator 实现

// 借助关联类型，我们不需要在使用该 trait 的方法时标注类型
// 因为我们不能为单个类型多次实现这样的 trait
// 对于使用关联类型的 trait 定义，由于我们只能实现一次 impl Iterator for Counter
// 所以 Counter 就只能拥有一个特定的 Item 类型
// 我们不需要在每次调用 Counter 的 next 方法时来显式地声明这是一个 u32 类型的迭代器
//
// 也就是说，使用泛型，可以多次实现这个 trait，每次实现时泛型的类型都不同
// 那么当我们使用 next 方法时，必须为 next 函数提供类型标注，才能知道我们具体使用的是哪个 next 实现
//
// 而对于关联类型，则不存在这个问题
// 因为我们只能实现一次这个 Iterator trait，Counter 就只能拥有一个特定的 Item 类型
// 所以我们只需为这个 trait 指定一次关联类型的具体类型
// 当调用 next 方法时，我们就不需要显式标注 next 的类型了
//


// 2. 默认泛型参数和运算符重载
// 我们可以在使用泛型参数时为泛型指定一个默认的具体类型
// 当使用默认类型就能工作时，该 trait 的实现者可以不用再指定另外的具体类型
// 你可以在定义泛型时通过语法 <PlaceholderType=ConcreteType>来为泛型指定默认类型

// 默认泛型参数常常被应用在运算符重载中
// 运算符重载 (operator overloading) 使我们可以在某些特定的情形下自定义运算符(比如 +)的具体行为
// 虽然Rust不允许你创建自己的运算符及重载任意的运算符，但我们可以实现 std::ops 中列出的那些 trait 来重载一部分相应的运算符

// 实现 Add trait 重载 + 运算符
use std::ops::Add;
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl Add for Point {
    // Add trait 拥有一个名为Output的关联类型，它被用来确定add 方法的返回类型
    type Output = Point;

    // add 方法将两个 Point 实例的 x 与 y 分别相加来创建出一个新的 Point
    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

// Add trait 的定义如下：
// trait Add<RHS=Self> {
//     type Output;
//     fn add(self, rhs: RHS) -> Self::Output;
// }

// RHS=Self 就是所谓的默认类型参数 (default type parameter)
// 泛型参数 RHS (也就是 “right-handle side”的缩写) 定义了add方法中rhs参数的类型
// 假如我们在实现 Add trait 的过程中没有为 RHS 指定一个具体的类型
// 那么RHS的类型就会默认为 Self，也就是我们正在为其实现 Add trait 的那个类型 

// 因为我们需要将两个 Piont 相加，所以实现 add 的过程中，使用了默认的 RHS 类型，没有指定新的类型

// 示例：实现 Add trait 时自定义 RHS 的类型而不使用其默认类型


// 这里有两个以不同单位存放值的结构体：Millimeters 与 Meters
// 我们希望可以将毫米表示的值与米表示的值相加，并在 Add 的实现中添加正确的转换计算

#[derive(Debug, PartialEq)]
struct Millimeters(u32);
#[derive(Debug, PartialEq)]
struct Meters(u32);


// 为 Millimeters 实现 Add trait，从而使 Millimeters 和 Meters 可以相加
impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    // 为了将 Millimeters 和 Meters 的值加起来
    // 我们指定 impl Add<Meters> 来设置 RHS 类型参数的值，而没有使用默认的 Self
    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

// 默认类型参数主要被用于以下两种场景:
// 1. 扩展一个类型而不破坏现有代码
// 2. 允许在大部分用户都不需要的特定场合进行自定义

// 标准库中的 Add trait 就是第二种场景的例子: 通常你只需要将两 个同样类型的值相加
// 但 Add trait 也同时提供了自定义额外行为的能力
// 在 Add trait 的定义中使用默认类型参数意味着，在大多数情况下你都不需要指定额外的参数
// 换句话说，就是可以避免一小部分重复的代码模块，从而可以更加轻松地使用 trait

// 第一种场景与第二种场景有些相似，但却采用了相反的思路: 当你想要为现有的 trait 添加一个类型参数来扩展功能时，你可以给它设定一个默认值来避免破坏已经实现的代码

// 多数情况下，使用默认类型
// 特殊情况下，使用自定义类型


// 3. 用于消除歧义的完全限定语法：调用相同名称的方法
// Rust 既不会阻止两个 trait 拥有相同名称的方法，也不会阻止你为同一个类型实现这样的 两个 trait
// 你甚至可以在这个类型上直接实现 与 trait 方法同名的方法。
// 当我们调用这些同名方法时，需要明确地告诉 Rust 你期望调用的具体对象

// 定义了两个拥有同名方法 fly 的 trait，并为本就拥有 fly 方法的 Human 类型实现了这两个 trait
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

// 当我们拥有两种实现了同一 trait 的类型时，对于 fly 等需要接收 self 作为参数的方法，Rust 可以自动地根据 self 的类型推导出具体的 trait 实现

// 当 trait 中的关联函数没有 self 参数时，在同一作用域下有两个实现了此种 trait 的类型时，Rust无法推导出你究竟想要调 用哪一个具体类型

// 一个带关联函数的 trait 和一个带同名关联函数的类型，并且这个类型还实现了该 trait
trait Animal {
    // 此方法不接收任何 self 参数
    fn baby_name() -> String;
}
struct Dog;

// Dog 本身就有 baby_name 方法
// 并且实现了 Animal trait 中的 baby_name 方法
impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }

}
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}


// 4. 用于在 trait 中附带另外一个 trait 的功能的超 trait
// 有时，我们会需要在一个 trait 中使用另外一个 trait 的功能
// 在这 种情况下，我们需要使当前 trait 的功能依赖于另外一个同时被实现的 trait
// 这个被依赖的 trait 也就是当前 trait 的超 trait (supertrait)

// 例如，假设我们希望创建一个拥有 outline_print 方法的 OutlinePrint trait
// 这个方法会在调用时打印出带有星号框的实例值
// 换句话说，给定一个实现了 Display trait 的 Point 结构体
// 如果它会将自己的值显示为 (x, y)，那么当 x 和 y 分别是 1 和 3 时，调用 outline_print 就会打印出如下所示的内容:
// ********** 
// *        *
// * (1, 3) *
// *        *
// **********

// 由于我们想要在 outline_print 的默认实现中使用 Display trait 的功能
// 所以 OutlinePrint trait 必须注明自己只能用于那些提供了 Display 功能的类型
// 我们可以在定义 trait 时指定 OutlinePrint:Display 来完成该声明，这类似于为泛型添加 trait 约束

// OutlinePrint 需要依赖于 Display trait 的功能，因此必须在声明 OutlinePrint trait 时指定 Display trait

use std::fmt;
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        // 定义注明了 OutlinePrint 依赖于 Display trait，所以我们能够在随后的方法中使用 to_string 函数
        // 任何实现了 Display trait 的类型都会自动拥有这一函数
        // 如果你尝试去掉trait名后的冒号与 Display trait 并继续使用 to_string
        // 那么Rust 就会因为无法在 当前作用域中找到 &Self 的 to_string 方法而抛出错误
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
    
}

struct Point2 {
    x: i32,
    y: i32
}

// impl OutlinePrint for Point {
//     fn outline_print(&self) {
        
//     }
// }

// OutlinePrint 依赖了 Display trait
// 而我们需要在 Point 上实现这个 OutlinePrint trait
// 如果 Point 没有实现 Display trait，那么再编译过程中就会报错：
// error[E0277]: `Point` doesn't implement `std::fmt::Display`
// --> src/main.rs:247:23
// |
// 247 | impl OutlinePrint for Point {
// |                       ^^^^^ `Point` cannot be formatted with the default formatter
// |
// = help: the trait `std::fmt::Display` is not implemented for `Point`
// = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
// note: required by a bound in `OutlinePrint`
// --> src/main.rs:226:21
// |
// 226 | trait OutlinePrint: fmt::Display {
// |                     ^^^^^^^^^^^^ required by this bound in `OutlinePrint`

// 也就是说，我们必须为 Point 实现 Display trait

// 为 Point 实现 Display trait
impl fmt::Display for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// 为 Point 实现了 Display trait 后
// 就可以为 Point 实现 OutlinePrint trait
impl OutlinePrint for Point2 {
    
}

// 如果一个 trait 依赖了另外一个 trait，那么可以在定义时，添加对该 trait 的依赖
// 为某个类型实现该 trait 的时候，必须要同时实现两个 trait

// 

fn main() {
    // println!("Hello, world!");
    // 运算符重载 -自定义 + 运算符的行为，实现结构体 Point 之间的相加
    // assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 }, Point { x: 3, y: 3 });

    // assert_eq!(Millimeters(100) + Meters(1), Millimeters(1100));

    // 当我们在Human的实例上调用 fly 时，编译器会默认调用直接实现 在类型上的方法
    let person = Human;
    // *waving arms furiously*
    person.fly();

    // 为了调用实现在 Pilot trait 或 Wizard trait 中的 fly 方法，我们需要使用更加显式的语法来指定具体的 fly 方法
    // This is your captain speaking.
    Pilot::fly(&person);
    // Up!
    Wizard::fly(&person);
    // 在方法名的前面指定 trait 名称向 Rust 清晰地表明了我们想要调用哪个 fly 实现
    // 另外，你也可以使用类似的 Human::fly(&person)语句，它与 person.fly() 在行为上等价，但会稍微冗长一些


    // A baby dog is called a Spot
    println!("A baby dog is called a {}", Dog::baby_name());

    // 直接调用 baby_name 方法，执行的是 Dog 结构体上的方法，而不是 Animial trait 上面的方法
    // 而我们希望调用 Animal trait 上面的方法
    // 尝试指定 Animal trait，如下所示
    // Animal::baby_name();
    // 因为 Rust 不知道我们应该使用 baby_name 方法的那个实现，所以会报错：
    // error[E0790]: cannot call associated function on trait without specifying the corresponding `impl` type
    //    --> src/main.rs:232:5
    //    |
    // 185 |     fn baby_name() -> String;
    //    |     ------------------------- `Animal::baby_name` defined here
    // ...
    // 232 |     Animal::baby_name();
    //    |     ^^^^^^^^^^^^^^^^^ cannot call associated function of trait
    //    |
    // 由于Animal::baby_name是一个没有self参数的关联函数而不是方法
    // 所以 Rust 无法推断出我们想要调用哪一个 Animal::baby_name 的实现


    // 为了消除歧义并指示 Rust 使用 Dog 为 Animal trait 实现的 baby_name 函数，我们需要使用完全限定语法（fully qualified syntax）
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    // 这段代码在尖括号中提供的类型标 <Dog as Animal> 注表明我们希望将 Dog 类型视作 Animal
    // 并调用 Dog 为 Animal trait 实现的 baby_name 函数

    // 一般来说，完全限定语法被定义为如下所示的形式:
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
    // 对于关联函数而言，上面的形式会缺少 receiver 而只保留剩下的参数列表

    // 你可以在任何调用函数或方法的地方使用完全限定语法， 而 Rust 允许你忽略那些能够从其他上下文信息中推导出来的部分
    // 只有当代码中存在多个同名实现，且 Rust 也无法区分出你期望调用哪个具体实现时，你才需要使用这种较为烦琐的显式语法



    // 4. 用于在 trait 中附带另外一个 trait 的功能的超 trait
    let p = Point2 {
        x: 1,
        y: 3
    };

    p.outline_print()
}
