// Rust 高级特性
// Unsafe Rust
// 不安全的 Rust
// 1. 不安全的 Rust 存在的意义
// 不安全 Rust 之所以存在是因为静态分析从本质上讲是保守的
// 当编译器在判断一段代码是否拥有某种安全保障时，它总是宁可错杀一些合法的程序也不会接受可能非法的代码
// 尽管某些代码也许是安全的，但目前的Rust编译器却可能会做出相反的结论
// 在这种情况下， 你可以使用不安全代码来告知编译器:“相信我，我知道自己在干些什么。”
// 这样做的缺点在于你需要为自己的行为负责:如果你错误地使用了不安全代码，那么就可能会引发不安全的内存问题，比如空指、针解引用等

// 另外一个需要不安全Rust的原因在于底层计算机硬件固有的不安全性。如果 Rust 不允许进行不安全的操作，那么某些底层任务可能根本就完成不了

// 总的来说，不安全的 Rust 存在的原因主要是包含两点：
// 1. 让开发者自己保证内存的安全
// 2. 提供使用底层不安全性操作的能力

// 提供四种在安全的 Rust 中不被允许的操作：
// 1. 解引用裸指针
// 2. 调用不安全的函数或方法
// 3. 访问或修改可变的静态变量
// 4. 实现不安全trait

// 使用 unsafe 关键字即可定义一段不安全的 Rust 代码

// 使用了 unsafe 标记代码，不意味代码一定是危险的、不安全的
// 它仅仅是将责任转移到了程序员的肩上，我们需要手动确定 unsafe 块中的代码会以合法的方式访问内存

// 为了尽可能地隔离不安全代码，你可以将不安全代码封装在一个安全的抽象中并提供一套安全的API，
// 某些标准库功能同样使用了审查后的不安全代码，并以此为基础提供了安全的抽象接口
// 这种技术可以有效地防止unsafe代码泄漏到任何调用它的地方，因为使用安全抽象总会是安全的

fn main() {
    // println!("Hello, world!");
    // 1. 裸指针 Raw Pointer
    // 不安全Rust的世界里拥有两种类似于引用的新指针类型，它们都被叫作裸指针 (raw pointer)
    // 与引用类似，裸指针 要么是可变的，要么是不可变的，它们分别被写作*const T和*mut T
    // 这里的星号是类型名的一部分而不是解引用操作，在裸指针的上下文中，不可变意味着我们不能直接对解引用后的指针赋值
    // 裸指针与引用、智能指针的区别在于:
    // 1. 允许忽略借用规则，可以同时拥有指向同一个内存地址的可变和不可变指针，或者拥有指向同一个地址的多个可变指针
    // 2. 不能保证自己总是指向了有效的内存地址
    // 3. 允许为空
    // 4. 没有实现任何自动清理机制
    // 在避免 Rust 强制执行某些保障后，你就能够以放弃安全保障为代价来换取更好的性能
    // 或者换取与其他语言、硬件进行交互的能力 (Rust的保障在这些领域本来就不起作用)


    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // 注意，我们没有在这段代码中使用 unsafe 关键字,你可以在安全代码内合法地创建裸指针
    // 但不能在不安全代码块外解引用裸指针

    // 在创建裸指针的过程中，我们使用了 as 来分别将不可变引用和可变引用强制转换为了对应的裸指针类型
    // 由于这两个裸指针来自有效的引用，所以我们能够确认它们的有效性。但要记住，这一假设并不是对任意一个裸指针都成立


    // 创建一个指向任意地址的裸指针
    // let address = 0x012345usize;
    // let r = address as *const i32;
    // 尝试使用任意内存地址的行为是未定义的：这个地址可能有数据，也可能没有数据，编译器可能会通过优化代码来去掉该次内存访问操作，否则程序可能会在运行
    // 时出现段错误（segmentation fault）。我们一般不会编写出如示例上面这样的代码，但它确实是合法的语句

    // 可以创建裸指针，但是不能在安全的代码块中直接解引用
    // error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
    //   --> src\main.rs:65:26
    //    |
    // 65 |     println!("r1 is {}", *r1);
    //    |
    // println!("r1 is {}", *r1);

    // 想要解引用裸指针，必须在 unsafe 代码块中
    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }


    // 2. 调用不安全的函数或方法
    // 第二种需要使用不安全代码块的操作便是调用不安全函数（unsafe function）
    // 除了在定义前面要标记 unsafe，不安全函数或方法看上去与正常的函数或方法几乎一模一样
    // 此处的 unsafe 关键字意味着我们需要在调用该函数时手动满足并维护一些先决条件
    // 因为Rust无法对这些条件进行验证。通过在 unsafe 代码块中调用不安全函数
    // 我们向Rust表明自己确实理解并实现了相关的约定

    // 使用 unsafe 关键字定义函数
    // 在 unsafe 代码块中调用 unsafe 函数

    unsafe {
        dangerous()
    }

    // 在安全代码块中调用 unsafe 函数，会报错：
    // error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
    //   --> src\main.rs:94:5
    //    |
    // 94 |     dangerous();
    //    |     ^^^^^^^^^^^ call to unsafe function
    // dangerous();

    let mut v = vec![1, 2, 3, 4, 5, 6];

    // 获得数组切片
    let r = &mut v[..];
    // 数组的 split_at_mut 被定义在可变切片上：它接收一个切片并从给定的索引参数处将其分割为两个切片
    let (a, b) = r.split_at_mut(3);
    // a 和 b 是两个可变数组切片
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // 我们无法仅仅使用安全 Rust 来实现这个函数，为了简单起见
    // 我们将 split_at_mut 实现为函数而不是方法，并只处理特定类型 i32 的切片而非泛型 T 的切片

    // 因为代码没有将 split_at_mut 函数标记为 unsafe，所以我们可以在安全 Rust 中调用该函数
    // 我们创建了一个对不安全代码的安全抽象
    // 并在实现时以安全的方式使用了 unsafe 代码，因为它仅仅创建了指向访问数据的有效指针
    split_at_mut(r, 3);

    // 如果我们没有对不安全的代码进行抽象，并且没有保证以安全的方式使用不安全的代码，则程序可能会发生崩溃：
    // let addr = 0x1234usize;
    // let r = addr as *mut i32;
    // let s1 : &[i32] = unsafe {
    //     slice::from_raw_parts_mut(r, 1000)
    // };

    // 由于我们不拥有这个随意地址的内存，所以就无法保证这段代码的切片中包含有效的 i32 值，尝试使用该slice会导致不确定的行为


    // 使用 extern 关键字定义外部的编程语言的函数
    // 因为定义咋 extern 代码块中的函数也是不安全的，所以要执行这些函数，也必须放在 unsafe 代码块中
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }


    // 3. 访问或者修改一个可变的静态变量
    // Rust确实是支持全局变量（global variable）
    // 但在使用它们的过程中可能会因为 Rust 的所有权机制而产生某些问题。如果两个线程同时访问一个可变的全局变量，那么就会造成数据竞争
    //  Rust 中，全局变量也被称为静态（static）变量
    // 静态变量类似于常量
    // 静态变量的名称会约定俗成地被写作 SCREAMING_SNAKE_CASE 的形式，并且必须要标注变量的类型(也就是本例中的&'static str)
    // 静态变量只能存储拥有 'static 生命周期的引用
    // 这意味着 Rust 编译器可以自己计算出它的生命周期而无须手动标注。访问一个不可变静态变量是安全的

    // 常量和不可变静态变量看起来可能非常相似，但它们之间存在一个非常微妙的区别:
    // 静态变量的值在内存中拥有固定的地址，使用它的值总是会访问到同样的数据
    // 与之相反的是，常量则允许在任何被使用到的时候复制其数据
    // 也就是说，任何时候访问到的静态变量，都是同一个数据
    // 而访问常量，就不一定了

    // 常量和静态变量之间的另外一个区别在于静态变量是可变的
    // 需要注意的是，访问和修改可变的静态变量是不安全的

    add_to_count(3);
    unsafe {
        // COUNTER: 3
        println!("COUNTER: {}", COUNTER)
    }

    // 实现不安全trait

    // 最后一个只能在 unsafe 中执行的操作是实现某个不安全 trait
    // 当某个 trait 中存在至少一个方法拥有编译器无法校验的不安全因素时，我们就称这个 trait 是不安全的
    // 你可以在 trait 定义的前面加上 unsafe 关键字来声明一个不安全 trait
    // 同时该 trait 也只能在 unsafe 代码块中实现
}

struct TestAr {

}

// 定义一个不安全的 trait
unsafe trait Foo {
    // 定义方法
}
// 通过使用unsafe impl，我们向Rust保证我们会手动维护好那些编译器无法验证的不安全因素
unsafe impl Foo for TestAr {
    // 对应方法的实现
}


// 声明一个可变的静态变量
// 和正常变量一样，我们使用 mut 关键字来指定静态变量的可变性
static mut COUNTER: u32 = 0;


fn add_to_count(inc: u32) {
    // 访问和修改可变的静态变量是不安全的
    // 所以需要将其包裹在 unsafe 代码块中
    unsafe {
        COUNTER += inc;
    }
}

// 声明一个静态变量
static HELLO_WORLD: &str = "hello, world";

unsafe fn dangerous() {

}


// 手写 split_at_mut
// 无法通过编译版本
// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//
//     // 获取数组长度
//     let len = slice.len();
//
//     // 通过断言检查给定的参数是否小于或等于当前切片的长度
//     // 如果给定的参数大于切片的长度，那么函数就会在尝试使用该索引前触发 panic
//     assert!(mid <= len);
//
//     // 因为 split_at_mut 需要返回两个可变的字符串切片，所以我们直接对 slice 的可变引用使用切片
//     // 一个从原切片的起始位置到 mid 索引的位置，另一个则从mid索引的位置到原切面的末尾
//     // 但是这样会报错，违反了 Rust 的借用规则：创建了多个对 slice 的可变引用
//     // 因为 Rust 的借用检查器无法理解我们正在借用一个切片的不同部分，它只知道我们借用了两次同一个切片
//     // 借用一个切片的不同部分从原理上来讲应该是没有任何问题的，因为两个切片并没有交叉的地方，
//     // 但 Rust 并没有足够智能到理解这些信息。当我们能够确定某段代码的正确性而Rust却不能时，不安全代码就可以登场了
//
//     // 报错信息
//     // error[E0499]: cannot borrow `*slice` as mutable more than once at a time
//     //    --> src\main.rs:123:30
//     //     |
//     // 119 | fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     //     |                        - let's call the lifetime of this reference `'1`
//     // ...
//     // 123 |     (&mut slice[..mid], &mut slice[mid..])
//     //     |     -------------------------^^^^^--------
//     //     |     |     |                  |
//     //     |     |     |                  second mutable borrow occurs here
//     //     |     |     first mutable borrow occurs here
//     //     |     returning this value requires that `*slice` is borrowed for `'1`
//     (&mut slice[..mid], &mut slice[mid..])
//
// }

use std::slice;

fn split_at_mut(s: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    // 获取切片的长度
    let len = s.len();
    // as_mut_ptr 方法获取切片的裸指针，s 的类型为 i32 类型的切片，因此 as_mut_ptr 返回一个类型为 *mut i32 类型的裸指针
    let ptr = s.as_mut_ptr();

    // 通过断言检查给定的参数是否小于或等于当前切片的长度
    // 如果给定的参数大于切片的长度，那么函数就会在尝试使用该索引前触发 panic
    assert!(mid <= len);


    unsafe {

        // slice::from_raw_parts_mut 函数接收一个裸指针和长度来创建一个可变切片
        // 这里的代码使用该函数从 ptr 处创建了一个拥有 mid 个元素的切片

        // offset 方法可以计算得到以当前指针为起始位置偏移 count 个位置后的新指针
        // 也就是说，将指针偏移 count 个位置，从而指向新的元素
        // 接着我们又在 ptr 上使用 mid 作为偏移量参数调用 offset 方法得到了一个从 mid 处开始的裸指针
        // 此时裸指针指向 mid 处的元素
        // 基于它创建了另外一个起始于 mid 处且拥有剩余所有元素的切片

        // 由于函数 slice::from_raw_parts_mut 接收一个裸指针作为参数并默认该指针的合法性，所以它是不安全的
        // 裸指针的 offset 方法也是不安全的，因为它必须默认此地址的偏移量也是一个有效的指针
        // 换句话说，偏移量不能超过指针的有效位置

        // 因此，我们必须在 unsafe 代码块中调用 slice::from_raw_parts_mut 和 offset函数
        // 通过审查代码并添加 mid 必须小于等于 len 的断言，我们可以确认 unsafe 代码块中的裸指针都会指向有效的切片数据且不会产生任何的数据竞争
        (
           slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid)
            )
    }
}


// 使用extern函数调用外部代码
// 在某些场景下，你的Rust代码可能需要与另外一种语言编写的代码进行交互
// Rust为此提供了 extern 关键字来简化创建和使用外部函数接口 (Foreign Function Interface，FFI)的过程
// FFI是编程语言定义函数的一种方式，它允许其他(外部的)编程语言来调用这些函数
// 下面的例子展示了如何调用 C 语言中的 abs 函数

// 任何在 extern 块中声明的函数都是不安全的
// 因为其他语言并不会强制执行Rust遵守的规则
// 而Rust又无法对它们进行检查，所以在调用外部函数的过程中，保证安全的责任也同样落在了开发者的肩上

// 下面的这段代码 extern "C" 块中列出了我们想要调用的外部函数名称及签名，
// 其中 extern 后面跟着的 "C" 指明了外部函数使用的应用二进制接口 (Application Binary Interface，ABI): 它被用来定义函数在汇编层面的调用方式。
// 我们使用的 "C" ABI 正是 C 编程语言的ABI，它也是最常见的ABI格式之一
extern "C" {
    fn abs(input: i32) -> i32;
}


// 在其他语言中调用 Rust 函数
// 我们同样可以使用 extern 关键字来创建一个允许其他语言调用 Rust 函数的接口
// 但不同于使用 extern 标注的代码块，我们需要将 extern 关键字及对应的 ABI 添加到函数签名的 fn 关键字前，并为该函数添加 #[no_mangle] 注解来避免 Rust 在编译时改变它的名称
// Mangling 是一个特殊的编译阶段，在这个阶段，编译器会修改函数名称来包含更多可用于后续编译步骤的信息
// 但通常也会使得函数名称难以阅读，几乎所有程序语言的编译器都会以稍微不同的方式来改变函数名称，为了让其他语言正常地识别 Rust 函数
// 我们必须要禁用 Rust 编译器的改名功能

// 定义一个可以在编译并链接后被 C 语言代码访问的 call_from_c 函数
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}