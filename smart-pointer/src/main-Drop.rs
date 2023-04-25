// 借助Drop trait在清理时运行代码

// Drop trait 对智能指针也十分重要
// 它允许我们在变量离开作用域时执行某些自定义操作
// 你可以为任意类型实现一个 Drop trait，它常常被用来释放诸如文件、网络连接等资源

// 通过实现 Drop trait 来指定值离开作用域时需要运行的代码
// Drop trait 要求实现一个接收self可变引用作为参数的drop函数
fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");

    // 打印信息如下:
    //     CustomSmartPointers created.
    // Droppong CustomSmartPointer with data `other stuff`
    // Droppong CustomSmartPointer with data `my stuff`

    // 当两个CustomSmartPointer实例离开作用域时
    // Rust会自动调用我们在drop方法中放置的代码来打印出最终的信息，而无须显式地调用drop方法
    // 因为变量的丢弃顺序与创建顺序相反，所以 d 在 c 之前被丢弃
    // 因此是先调用 d 的 drop 方法，再调用 c 的 drop 方法

    // 使用 std::mem::drop 提前丢弃值
    // 有一些场景下，需要我们提前释放某些值：
    // 例如使用智能指针来管理锁时：你也许会希望强制运行 drop 方法来提前释放锁，从而允许同一作用域内的其他代码来获取它
    // Rust并不允许我们手动调用 Drop trait 的 drop 方法
    // 但是，你可以调用标准库中的std::mem::drop函数来提前清理某个值
    // 

    // 可以尝试手动调用 drop 方法
    // c.drop();
    // 直接调用 drop 方法会报错：
    // error[E0040]: explicit use of destructor method
//   --> src/main.rs:32:7
//   |
// 32 |     c.drop();
//   |     --^^^^--
//   |     | |
//   |     | explicit destructor calls not allowed

    // 这条错误提示信息表明我们不能显式地调用 drop。信息中使用了一个专有名词析构函数 （destructor），
    // 这个通用的编程概念被用来指代可以清理实例的函数，它与创建实例的构造函数（constructor）相对应
    // 而 Rust 中的 drop 函数正是这样一个析构函数。
    // 因为 Rust 已经在 main 函数结尾的地方自动调用了 drop
    // 所以它不允许我们再次显式地调用 drop
    // 这种行为会试图对同一个值清理两次而导致重复释放（double free）错误。
    // 总结：我们既不能在一个值离开作用域时禁止自动插入 drop，也不能显式地调用 drop 方法
    
    // 可以调用 std::mem::drop 来清理某个值
    // drop函数不同于 Drop trait 中的 drop 方法。我们需要手动调用这个函数，并将需要提前丢弃的值作为参数传入
    // 因为该函数被放置在了预导入模块中，所以可以直接调用
    println!("CustomSmartPointer Created.");

    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");

    // 上面的代码输出如下：
    // CustomSmartPointer Created.
    // Droppong CustomSmartPointer with data `my stuff`
    // CustomSmartPointer dropped before the end of main.

    // 上面的输出表明，确实在预期位置清理掉了实例 c
}

struct CustomSmartPointer {
    data: String,
}

// CustomSmartPointer 实现 Drop 这个 trait 
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        // 当 CustomSmartPointer 的实例离开作用域的时候，调用 drop 方法
        // 实际上，任何你想要在类型实例离开作用域时运行的逻辑都可以放在 drop 函数体内
        println!("Droppong CustomSmartPointer with data `{}`", self.data);
    }
}