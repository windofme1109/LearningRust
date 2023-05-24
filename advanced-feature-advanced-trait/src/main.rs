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

fn main() {
    println!("Hello, world!");
}
