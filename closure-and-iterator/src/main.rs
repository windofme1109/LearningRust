fn main() {
    // 迭代器 Iterator
    // 迭代器模式允许你依次为序列中的每一个元素执行某些任务。迭代器会在这个过程中负责遍历每一个元素并决定序列何时结束

    // 在Rust中，迭代器是惰性的（layzy）
    // 这也就意味着创建迭代器后，除非你主动调用方法来消耗并使用迭代器否则它们不会产生任何的实际效果
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    // 使用 for in 循环消耗迭代器
    for val in v1_iter  {
        println!("val {}", val);
    }

    // 手动调用迭代器的 next 方法消耗迭代器中的元素
    let v2 = vec![1, 2, 3];
    // 使用迭代器的 next 方法，必须定义可变的迭代器
    let mut v2_iter = v2.iter();
    // next 方法默认返回的是对数组元素的不可变引用，所以预期相等就是 Some(&1)
    assert_eq!(v2_iter.next(), Some(&1));
    assert_eq!(v2_iter.next(), Some(&2));
    assert_eq!(v2_iter.next(), Some(&3));
    assert_eq!(v2_iter.next(), None);

    // 注意，这里的 v2_iter 必须是可变的，因为调用 next 方法改变了迭代器内部用来记录序列位置的状态
    // 换句话说，这段代码消耗或使用了迭代器，每次调用 next 都吃掉了迭代器中的一个元素
    // 在刚才的 for 循环中我们之所以不要求 v1_iter 可变，是因为循环取得了 v1_iter 的所有权并在内部使得它可变了
    //
    // 另外还需要注意到，iter 方法生成的是一个不可变引用的迭代器
    // 我们通过 next 取得的值实际上是指向动态数组中各个元素的不可变引用
    // 如果你需要创建一个取得 v2 所有权并返回元素本身的迭代器那么你可以使用 into_iter 方法
    // 类似地，如果你需要可变引用的迭代器，那么你可以使用iter_mut方法
    //

    let v3 = vec![1, 2, 3];
    // 使用迭代器的 next 方法，必须定义可变的迭代器
    let mut v3_iter = v3.into_iter();

    for val3 in v3_iter {
        println!("val3 {}", val3);
    }
    // 使用了 into_iter 方法后，返回一个获取 v3 所有权并返回元素本身的迭代器
    // 使用这个迭代器的时候，数组中的每个元素会从数组中移出，所以，下面就不能在使用 v3 了
    // error[E0382]: borrow of moved value: `v3`
    // println!("{:?}", v3);


    // 消耗迭代器的方法 - consuming adaptor
    // 标准库为 Iterator trait 提供了多种包含默认实现的方法，这些方法指的是除了 next 以外的方法
    // 这些方法中的一部分会在它们的定义中调用 next 方法，这也是我们需要在实现 Iterator trait 时手动定义 next 方法的原因
    // 这些调用 next 的方法也被称为消耗适配器 （ consuming adaptor）
    // 因为它们同样消耗了迭代器本身。以 sum 方法为例，这个方法会获取迭代器的所有权并反复调用 next 来遍历元素
    // 进而导致迭代器被消耗，在迭代过程中，它会对所有元素进行求和并在迭代结束后将总和作为结果返回

    let v4 = vec![1, 2, 3];
    let v4_iter = v4.iter();
    let v4_sum: i32 = v4_iter.sum();
    // v4_sum 6
    println!("v4_sum {}", v4_sum);

    // 调用完 迭代器的 sum 方法完以后，这个迭代器就不能用了，因为这个迭代器已经被消耗完了

    // 生成其他迭代器的方法 - iterator adaptor
    // Iterator trait 还定义了另外一些被称为迭代器适配器（iterator adaptor）的方法
    // 这些方法可以使你将已有的迭代器转换成其他不同类型的迭代器
    // 你可以链式地调用多个迭代器适配器完成一些复杂的操作，同时保持代码易于阅读
    // 但因为所有的迭代器都是惰性的，所以你必须调用一个消耗适配器的方法才能从迭代器适配器中获得结果

    // 链式调用不同的迭代器，以实现不同的功能
    let v5 = vec![1, 2, 3, 4];
    // 调用迭代器适配器 map 来创建新的迭代器
    // v5.iter().map(|x| {x + 1});
    // 只定义，不使用迭代器，编译器会给我们一个警告：
    //     warning: unused `Map` that must be used
    //   --> src\main.rs:71:5
    //    |
    // 71 |     v5.iter().map(|x| {x + 1});
    //    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
    //    |
    //    = note: iterators are lazy and do nothing unless consumed

    // 迭代器是惰性的，除非消费迭代器，否则它不会做任何事
    // 使用 collect 方法消耗迭代器
    // collect 方法会将 迭代器转换为多种集合，因此将需要指定 v6 的类型
    // _ 表示是占位符
    let v6: Vec<_> = v5.iter().map(|x| {x + 1}).collect();

    assert_eq!(v6, vec![2, 3, 4, 5]);


    // 使用 filter 迭代器
    filters_by_size();


    // 使用自定义迭代器
    // 使用迭代器会改变内部的状态，这里要将 counter 设置为可变的
    let mut counter = Counter::new();

    // 调用 next 方法消耗迭代器
    // assert_eq!(counter.next(), Some(1));
    // assert_eq!(counter.next(), Some(2));
    // assert_eq!(counter.next(), Some(3));
    // assert_eq!(counter.next(), Some(4));
    // assert_eq!(counter.next(), Some(5));
    // assert_eq!(counter.next(), None);

    // 调用其迭代器的其他方法

    // 我们只需要提供 next 方法的定义便可以使用标准库中那些拥有默认实现的 Iterator trait 方法，因为这些方法都依赖于 next 方法的功能
    // 例如： 我们希望将一个 Counter 实例产生的值与另一个 Counter 实例跳过首元素后的值一一配对
    // 接着将配对的两个值相乘，最后再对乘积中能被3整除的那些数字求和

    // 迭代器的 zip 方法将两个迭代器组合成一个迭代器，具体的说，就是分别将两个迭代器的第一个元素组合为一个元组，作为新的迭代器的第一个元素，以此类推
    // 只要有一个迭代器返回 None，zip 方法就返回 None，结束生成迭代器
    // 迭代器的 skip() 方法接收一个 n 参数，表示生成一个跳过前 n 个元素的新迭代器
    // Counter::new().zip(Counter::new().skip(1)) 生成的新的迭代器的元素为：
    // (1, 2) (2, 3) (3, 4) (4, 5)
    // 有一个迭代器返回 None，zip 方法就返回 None，因此不会生成 (5, None)
    let sum:u32 = Counter::new().zip(Counter::new().skip(1))
        .map(|(a, b)| {a * b})
        .filter(|x| {x % 3 == 0})
        .sum();

    // 因为我们指定了 next 方法的具体行为，而标准库又对其他调用 next 的方法提供了默认实现
    // 所以我们能够合法地使用所有这些方法
    // sum is 18
    println!("sum is {}", sum);


}


fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];
    let in_my_size = shoes_in_my_size(shoes, 10);
    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}

// Iterator trait 和 next 方法
// 所有的迭代器都实现了定义于标准库中的 Iterator trait
// 该 trait 的定义类似于下面这样：
// pub trait Iterator {
// type Item;
// fn next(&mut self) -> Option<Self::Item>;
// // 这里省略了由Rust给出的默认实现方法
// }

// 上面这个 trait 的定义 使用了两种新语法：type Item 和 Self::Item
// 它们定义了 trait 的关联类型（associated type）
// 现在，我们需要知道，这段代码表明，为了实现 Iterator trait、
// 我们必须要定义一个具体的 Item 类型，而这个 Item 类型会被用作 next 方法的返回值类型
// 换句话说，Item类型将是迭代器返回元素的类型

// Iterator trait 只要求实现者手动定义一个方法：next 方法，它会在每次被调用时返回一个包裹在Some中的迭代器元素
// 并在迭代结束时返回 None

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String
}


fn shoes_in_my_size(shoes_list: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // 使用迭代器后，一定要消耗迭代器，这里使用 collect 方法消耗迭代器，将迭代器转换为集合
    // 迭代器的 filter 方法会接收一个闭包作为参数，这个闭包会在遍历迭代器中的元素时返回一个布尔值
    // 而每次遍历的元素只有在闭包返回 true 时才会被包含在filter生成的新迭代器中

    // 首先调用了 into_iter 来创建可以获取动态数组所有权的迭代器
    // 接着调用 filter 来将这个迭代器适配成一个新的迭代器，新的迭代器只会包含闭包返回值为 true 的那些元素
    shoes_list.into_iter()
        .filter(|shoe| { shoe.size == shoe_size })  // 闭包可以捕获环境上下文中的变量
        .collect()
}


// 使用 Iterator trait 来创建自定义迭代器
// 我们可以通过实现 Iterator trait 来创建拥有自定义行为的迭代器
// 只需要提供一个 next 方法的定义即可实现 Iterator trait
// 一旦完成该方法定义 ， 我们就可以使用其他一切拥有默认实现的 Iterator trait 提供的方法

// 给结构体 Counter 实现一个 Iterator trait
struct Counter {
    count:u32
}
// Counter 结构体只有一个名为 count 的字段，它存储的 u32 值被用来记录迭代器从 1 遍历到 5 这个过程中的状态
// count 字段被设置为私有的，以便 Counter 能够独立管理其中的值
// new 函数则确保了任何一个新实例中的 count 字段的值都会从 0 开始
impl Counter {
    fn new() -> Counter {
        Counter {
            count: 0
        }
    }
}

// 为 Counter 实现自定义 Iterator
//
impl Iterator for Counter {

    // 迭代器的关联类型 Item 指定为了 u32，这也就意味着迭代器将返回一个 u32 序列
    type Item = u32;

    // 实现 next 方法
    fn next(&mut self) -> Option<Self::Item> {
        // count 字段从 0 开始
        // 因为我们向让迭代器从 1 开始输出
        // 所以这里是先加 1
        self.count += 1;

        if (self.count < 6) {
            // 迭代器从 1 输出到 5，所以 1 - 5 这范围内，要输出 Some 这个变体
            Some(self.count)
        } else {
            // 迭代器结束时，输出 None 变体
            None
        }
    }
}
