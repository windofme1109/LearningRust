fn main() {
    // 引用与借用
    // 由于将值传入函数，会发生所有权的转移，即值的移动
    // 如果我们还想继续使用传入函数的值，那么我们不得不在函数中，将这个参数返回（将其放入元组中）
    // 这样的作法非常繁琐


    // Rust 提供了一种叫做引用的方式，使得我们仅仅是引用值，而不是移动值，或者说是转移它的所有权

    let s1 = String::from("hello world");
    // &s1 表示对 s1 的引用，不会发生所有权的转移
    let s1_length = calculate_length(&s1);
    // s1 hello world, len 11
    println!("s1 {}, len {}", s1, s1_length);

    // 默认情况下，我们不能对应用的数据进行修改

    // let s2 = String::from("hello world2");
    // change 尝试修改引用的 s2 的值，结果报错：
    // cannot borrow `*some_string` as mutable, as it is behind a `&` reference
    // 就是说，不能对引用的值进行修改
    // change(&s2);

    // 如果想要对引用的值进行修改，那么首先要声明其是可修改的
    // 同时在引用的时候，也要使用 mut 关键字，表示其是可以修改的
    let mut s2 = String::from("hello");
    // 定义函数的时候，定义参数是可变引用：&mut
    change(&mut s2);
    // s2 hello, world
    println!("s2 {}", s2);

    // 可变引用在使用上有一个很大的限制：
    // 对于特定作用域中的特定数据来说，一次只能声明一个可变引用
    let mut s3 = String::from("hello");
    // r1 和 r2 都是对 s3 的可变引用，Rust 不允许我们这样做
    // 这样也比较号理解，就是我们在同一个作用域，只能有声明一个可变引用，也就是只能有一个变量可以对 s3 进行修改
    // 这样使得对 s3 的修改是可控的，因为 s3 发生了变化，一定是这个唯一的可变引用修改的
    // let r1 = &mut s3;
    // let r2 = &mut s3;
    // error[E0499]: cannot borrow `s3` as mutable more than once at a time
    // println!("{}, {}", r1, r2)
    // 在Rust中遵循这条限制性规则可以帮助我们在编译时避免数据竞争
    // 数据竞争 （data race）与竞态条件十分类似，它会在指令满足以下3种情形时发生：
    // 1. 两个或两个以上的指针同时访问同一空间。
    // 2. 其中至少有一个指针会向空间中写入数据。
    // 3. 没有同步数据访问的机制
    // 存在数据竞争的代码连编译检查都无法通过

    // 在不同的作用域内，可以声明多个可变引用
    // 下面的代码允许声明多个可变引用，但是会发生编译错误
    // error[E0499]:cannot borrow `s3` as mutable more than once at a time
    // 原因是发生了数据竞争：满足了数据竞争的三个条件，因此编译报错
    // let r1 = &mut s3;
    // {
    //     let r2 = &mut s3;
    //     r2.push_str(", world");
    //     println!("r2 {}", r2);
    // }
    // r1.push_str(", world2");
    // println!("s3 {}", s3);


    // 关于引用，还有一条规则就是我们不能在拥有不可变引用的同时创建可变引用，如下所示
    // let r1 = &s3;
    // let r2 = &s3;
    // let r3 = &mut s3;
    //  cannot borrow `s3` as mutable because it is also borrowed as immutable
    // println!("{}, {}, {}", r1, r2, r3);

    // 这条规则也比较好理解，因为如果声明了不可变引用，表示我们不希望对这个数据进行变动
    // 因此不能在声明一个可变引用了
    // 存在多个不可变引用是合理合法的，对数据的只读操作不会影响到其他读取数据的用户
    //

    // 使用拥有指针概念的语言会非常容易错误地创建出悬垂指针
    // 这类指针指向曾经存在的某处内存地址，但该内存已经被释放掉甚至是被重新分配另作他用了。
    // 而在 Rust 语言中，编译器会确保引用永远不会进入这种悬垂状态。
    // 假如我们当前持有某个数据的引用，那么编译器可以保证这个数据不会在引用被销毁前离开自己的作用域
    // error[E0106]: missing lifetime specifier
    //  this function's return type contains a borrowed value, but there is no value for it to be borrowed from
    // let reference_to_nothing = dangle();
}

// & 表示这里是一个引用，它们允许你在不获取所有权的前提下使用值
// some_string 表示的是对传入的 String 类型的栈内存的引用
// 换句话说，就是 some_string 指向的是传入的 String 类型的栈内存
// 传入的 String 类型的值并不会发生移动，即其栈内存中的数据不会移动到 some_string 这里
// 调用 calculate_length 函数时，传入的变量 也要使用 &表示引用
fn calculate_length(some_string: &String) -> usize {
    // 此处，变量some_string的有效作用域与其他任何函数参数一样
    // 唯一不同的是，它不会在离开自己的作用域时销毁其指向的数据
    // 因为它并不拥有该数据的所有权。当一个函数使用引用而不是值本身作为参数时
    // 我们便不需要为了归还所有权而特意去返回值，毕竟在这种情况下，我们根本没有取得所有权
    let len = some_string.len();

    return len;
}

// 尝试对引用的值进行修改
// fn change(some_string: &String) {
//     // Cannot borrow immutable local variable `some_string` as mutable
//     some_string.push_str(", world");
// }

fn change(some_string: &mut String) {
    // Cannot borrow immutable local variable `some_string` as mutable
    some_string.push_str(", world");
}

// 由于变量 s 创建在函数dangle内，所以它会在 dangle 执行完毕时随之释放。
// 但是，我们的代码依旧尝试返回一个指向 s 的引用，
// 这个引用指向的是一个无效的 String，这个操作十分危险
// 而 Rust 成功地拦截了我们的危险代码
// fn dangle() -> &String {
//     let s = String::from("hello world");
//     return &s;
// }

// 解决上面的问题的方法是：直接返回 s 即可，这样将 s 的所有权转移函数外
fn dangle() -> String {
    let s = String::from("hello world");
    return s;
}
