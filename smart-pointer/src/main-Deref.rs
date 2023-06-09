// Deref trait

// 实现 Deref trait 使我们可以自定义解引用运算符（dereference operator）*的行为（这一符号也同时被用作乘法运算符和通配符）
// 通过实现 Deref，我们可以将智能指针视作常规引用来进行处理


use std::ops::Deref;

fn main() {

    // 使用解引用运算符跳转到指针指向的值
    // 常规引用就是一种类型的指针。你可以将指针形象地理解为一个箭头，它会指向存储在别处的某个值
    let x = 5;
    let y = &x;

    assert_eq!(x, 5);
    assert_eq!(*y, 5);

    // 变量x存储了一个 i32 值 5，并在变量 y 中存储了 x 的引用。我们可以直接断言，这里的 x与 5 相等
    // 但是，当你想要断言变量 y 中的值时，我们就必须使用 *y 来跟踪引用并跳转到它指向的值（也就是解引用）
    // 在对 y 进行了解引用后，我们才可以得到 y 指向的整数值，并将它与 5 进行比较

    // 直接将 y 与 5 进行比较：
    // error[E0277]: can't compare `&{integer}` with `{integer}`
    // assert_eq!(y, 5);
    // 上面的报错信息显示：由于数值和引用是两种不同的类型，所以你不能直接比较这两者
    // 我们必须使用解引用运算符来跳转到引用指向的值


    // 把Box<T>当成引用来操作

    let x = 5;
    let y = Box::new(x);

    // 将 y 设置为了一个指向 x 值的装箱指针而不是一个指向x值的引用
    assert_eq!(5, x);
    // 我们依然可以使用解引用运算符来跟踪装箱指针
    assert_eq!(5, *y);

    // 使用自定义的智能指针
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    // 在没有 Deref trait 的情形下，编译器只能对 & 形式的常规引用执行解引用操作
    // deref 方法使编译器可以从任何实现了 Deref trait 的类型中获取值，并能够调用 deref 方法来获得一个可以进行解引用操作的引用
    assert_eq!(5, *y);

    // *y 等同于 *(y.deref())
    // Rust 使用 * 运算符来替代 deref 方法和另外一个朴素的解引用操作
    // 这样我们就不用考虑是否需要调用 deref 方法了。这一特性使我们可以用完全相同的方式编写代码来处理常规引用及实现了 Deref trait的类型。

    // 所有权系统决定了 deref 方法需要返回一个引用，而*(y.deref())的最外层依然需要一个朴素的解引用操作
    // 假设 deref 方法直接返回了值而不是指向值的引用，那么这个值就会被移出self
    // 在大多数使用解引用运算符的场景下，我们并不希望获得MyBox<T>内部值的所有权。

    // 需要注意的是，这种将 * 运算符替换为 deref 方法和另外一个朴素 * 运算符的过程，对代码中的每个 * 都只会进行一次。因为 * 运算符的替换不会无穷尽地递归下去，所以我们才能在代码中得到i32类型的值

    // 朴素的解引用操作，指的是对 & 形式的常规引用执行解引用操作
    // 所有权系统决定了 deref 方法需要返回一个引用，实际上，y.deref 函数返回的是一个常规引用，为了获取引用指向的值，我们依旧需要使用解引用操作
    // 进而获取了引用指向的值




    // 函数和方法的隐式解引用转换
    // 解引用转换（deref coercion）是Rust为函数和方法的参数提供的一种便捷特性
    // 当某个类型 T 实现了 Deref trait 时，它能够将T的引用转换为 T 经过 Deref 操作后生成的引用
    // 当我们将某个特定类型的值引用作为参数传递给函数或方法，但传入的类型与参数类型不一致时解引用转换就会自动发生
    // 编译器会插入一系列的 deref 方法调用来将我们提供的类型转换为参数所需的类型

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // 上面的示例中将参数 &m 传入了 hello 函数，而 &m 正是一个指向 MyBox<String> 值的引用
    // 因为 MyBox<T> 实现了 Deref trait，所以 Rust 可以通过调用 deref 来将 &MyBox<String> 转换为 &String
    // 因为标准库为 String 提供的 Deref 实现会返回字符串切片
    // 所以Rust可以继续调用 deref 来将 &String 转换为 &str，并最终与 hello 函数的定义相匹配

    // 如果没有自动解引用的功能，我们需要对参数做如下转换
    hello(&(*m)[..]);

    // *m 表示将 MyBox<String> 解引用，转换为 String
    // & 和 [..] 操作将 String 转换为字符串切片，这样才能匹配 hello 函数的参数类型

    // 只要代码涉及的类型实现了Deref trait，Rust 就会自动分析类型并不断尝试插入Deref::deref来获得与参数类型匹配的引用
    // 因为这一分析过程会在编译时完成，所以解引用转换不会在运行时产生任何额外的性能开销！


    // 解引用转换与可变性

    // 使用 Deref trait 能够重载不可变引用的 * 运算符
    // 使用 DerefMut trait 能够重载可变引用的 * 运算符

    // Rust 会在类型与 trait 满足下面 3 种情形时执行解引用转换：
    // 1. 当 T: Deref<Target=U> 时，允许 &T 转换为 &U
    // 2. 当 T: DerefMut<Target=U> 时，允许 &mut T 转换为 &mut U
    // 3. 当 T: Deref<Target=U> 时，允许 &mut T转换为 &U
    // 将一种类型转换为另外一种类型，这个转换可以在 deref 方法中进行

    // 情形一意味着，如果 T 实现了类型 U 的 Deref trait，那么 &T 就可以被直接转换为 &U
    // 情形二意味着，同样的解引用转换过程会作用于可变引用。
    // 情形三则有些微妙：Rust 会将一个可变引用自动地转换为一个不可变引用。但这个过程绝对不会逆转
    // 也就是说不可变引用永远不可能转换为可变引用。
    // 因为按照借用规则，如果存在一个可变引用，那么它就必须是唯一的引用，不能即使可变的，同时还是不可变的引用
    // 将一个可变引用转换为不可变引用肯定不会破坏借用规则
    // 但将一个不可变引用转换为可变引用则要求这个引用必须是唯一的
    // 而借用规则无法保证这一点。因此，Rust 无法将不可变引用转换为可变引用视作一个合理的操作

}


fn hello(name: &str) {
    println!("Hello, {} !", name);
}

// 定义我们自己的智能指针

// 定义了一个名为 MyBox 的结构体。结构体的定义中附带了泛型参数 T，因为我们希望它能够存储任意类型的值
// MyBox是一个拥有 T 类型单元素的元组结构体
struct MyBox<T>(T);

impl<T> MyBox<T> {
    // 它的关联函数 MyBox::new 接收一个T类型的参数，并返回一个存储有传入值的 MyBox 实例作为结果
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// 为了使得 MyBox 类型能被解引用（使用 * 操作符），我们必须实现 Deref trait
impl<T> Deref for MyBox<T> {
    // 定义了 Deref trait 的一个关联类型
    type Target = T;
    // 标准库中的 Deref trait则要求我们实现一个 deref 方法，该方法会借用 self 并返回一个指向内部数据的引用
    fn deref(&self) -> &Self::Target {
        // 因为 MyBox 是元组结构体，且只有一个元素，所以这里直接返回 &self.0，即对第一个元素的引用，进而允许调用者通过 * 运算符访问值
        &self.0
    }
}