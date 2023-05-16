// 为共有行为定义一个 trait


// Rust有意避免将结构体和枚举称为“对象”，以便与其他语言中的对象概念区分开来
// 对于结构体或枚举而言，它们字段中的数据与 impl 块中的行为是分开的；而在其他语言中，数据和行为往往被组合在名为对象的概念中
// trait 对象则有些类似于其他语言中的对象，因为它也在某种程度上组合了数据与行为
// 但 trait 对象与传统对象不同的地方在于，我们无法为trait对象添加数据
// 由于 trait 对象被专门用于抽象某些共有行为，所以它没有其他语言中的对象那么通用

pub trait Draw {
    // 定义一个 draw 方法
    fn draw(&self);
}

// 注意：trait 对象必须保证对象安全
// 需要注意的是，你只能把满足对象安全（object-safe）的 trait 转换为 trait 对象
// Rust 采用了一套较为复杂的规则来决定某个 trait是否对象安全
// 但在实际应用中，我们只需要关注其中两条规则即可
// 如果一个trait中定义的所有方法满足下面两条规则，那么这个trait就是对象安全的：
// 1. 方法的返回类型不是 Self。
// 2. 方法中不包含任何泛型参数。

// 关键字 Self 是一个别名，它指向了实现当前 trait 或方法的具体类型

// trait 对象必须是对象安全的，因为 Rust 无法在我们使用 trait 对象时确定实现这个 trait 的具体类型究竟是什么
// 由于 trait 对象忘记了 Self 的具体类型，所以编译器无法在 trait 方法返回 Self 时使用原来的具体类型
// 对于 trait 方法中的泛型参数而言，我们会在使用时将具体类型填入泛型所处的位置，这些具体类型会被视作当前类型的一部分
// 由于trait对象忘记了类型信息，所以我们无法确定被填入泛型参数处的类型究竟是哪一个


// 标准库中的 Clone trait 就是一个不符合对象安全的例子
// Clone trait 中的 clone 方法拥有这样的签名：
// pub trait Clone {
//     fn clone(&self) -> Self;
// }

// 由于 String 类型实现了 Clone trait，所以我们可以在 String 实例上调用 clone 方法来获得一个新的 String 实例
// 类似地，我们也可以在 Vec<T> 实例上调用 clone 来获得新的 Vec<T> 实例
// clone方法的签名需要知道 Self 究竟代表了哪一种具体类型，因为这是它作为结果返回的类型


// 但是，如果我们将 Clone 作为 trait 对象，那么编译器会报错，指出：指出违反了对象安全规则的地方
// error[E0038]: the trait `Clone` cannot be made into an object
//   --> src/lib.rs:44:29
//   |
// 44 |     pub components: Vec<Box<dyn Clone>>
//   |                             ^^^^^^^^^ `Clone` cannot be made into an object

// 上面的报错信息指出，我们不能将 Clone trait 作为 trait 对象

// pub struct Screen3 {
//     pub components: Vec<Box<dyn Clone>>
// }


// 定义一个持有 components 动态数组的 Screen 结构体
// 这个动态数组的元素类型使用了新语法 Box<dynDraw> 来定义 trait 对象
// 它被用来代表所有被放置在 Box 中且实现了Draw trait 的具体类型
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>
}

impl Screen {
    pub fn run(&self) {

        // 获得 components 数组中的每一个元素，因为每个元素都实现了 Draw 这个 trait，所以我们可以直接调用 draw 方法
        for component in self.components.iter()  {
            component.draw();
        }
    }
}


// 当然，我们也可以使用泛型约束数组中的类型

// 使用泛型参数与 trait 约束定义
// 数组中的类型都是 T，同时要求 T 必须实现 Draw 这个 trait
pub struct Screen2<T: Draw> {
    components: Vec<T>
}

impl<T> Screen2<T> where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter()  {
            component.draw();
        }
    }
}

// 为了使用新定义的 Screen2 实例，我们必须在 components 这个列表中存储相同的类型，因为定义是 Vec<T>
// 所以如果我们需要的仅仅是同质集合 （homogeneous collection），那么使用泛型和trait约束就再好不过了
// 因为这段定义会在编译时被多态化以便使用具体类型

// 另一方面，借助于在方法中使用 trait 对象，Screen 实例持有的 list 可以同时不同的类型，因为他们使用了智能指针 Box
// Vec定义时，其类型是 Box 指针，而不是某个类型或者是泛型


// 定义一个 Button 类型
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}


// Button 这个类型实现 Draw 这个 trait
impl Draw for Button {
    fn draw(&self) {
        println!("draw button");
    }
}