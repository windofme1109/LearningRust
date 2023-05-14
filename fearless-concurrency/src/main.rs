// 互斥体一次只允许一个线程访问数据

// 互斥体（mutex）是英文 mutual exclusion 的缩写
// 也就是说，一个互斥体在任意时刻只允许一个线程访问数据
// 为了访问互斥体中的数据，线程必须首先发出信号来获取互斥体的锁（lock）
// 锁是互斥体的一部分，这种数据结构被用来记录当前谁拥有数据的唯一访问权
// 通过锁机制，互斥体守护（guarding）了它所持有的数据。
//
// 互斥体是出了名的难用，因为你必须牢记下面两条规则：
// 1. 必须在使用数据前尝试获取锁
// 2. 必须在使用完互斥体守护的数据后释放锁，这样其他线程才能继续完成获取锁的操作

use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    //  使用关联函数 new 来创建 Mutex<T> 实例
    let m = Mutex::new(5);
    {
        // 为了访问 Mutex<T> 实例中的数据，我们首先需要调用它的 lock 方法来获取锁
        // 这个调用会阻塞当前线程直到我们取得锁
        let mut num = m.lock().unwrap();

        // 当前线程对于 lock 函数的调用会在其他某个持有锁的线程发生 panic 时失败
        // 实际上，任何获取锁的请求都会在这种场景里以失败告终，所以示例中的代码选择使用 unwrap 在意外发生时触发当前线程的panic

        // 一旦获取了锁，我们便可以将它的返回值 num 视作一个指向内部数据的可变引用
        // Rust 的类型系统会确保我们在使用 m 的值之前执行加锁操作：因为Mutex<i32>并不是i32的类型，所以我们必须获取锁才能使用i32值
        // 我们无法忘记或忽略这一步骤，因为类型系统并不允许我们以其他方式访问内部的i32值

        // Mutex<T>是一种智能指针。更准确地说，对 lock 的调用会返回一个名为 MutexGuard 的智能指针
        // 这个智能指针通过实现 Deref 来指向存储在内部的数据
        // 它还会通过实现 Drop 来完成自己离开作用域时的自动解锁操作
        // 这种释放过程会发生在内部作用域的结尾处，
        // 因此，我们不会因为忘记释放锁而导致其他线程无法继续使用该互斥体。锁的释放过程是自动发生的
        *num = 6;
    }

    // 输出的数据确实被修改了
    // m = Mutex { data: 6, poisoned: false, .. }
    println!("m = {:?}", m);


    // 首先创建了一个名为 counter 的变量来存储持有i32值的Mutex<T>

    // let counter = Mutex::new(0);
    // let mut handles = vec![];


    // for _ in 1..10 {
    //     // 我们通过迭代数字范围创建出了10个线程
    //     // 在调用 thread::spawn 创建线程的过程中，我们给所有创建的线程传入了同样的闭包
    //     // 这个闭包会把计数器移动至线程中，它还会调用 Mutex<T> 的 lock 方法来进行加锁
    //     // 并为互斥体中的值加 1
    //     // 而当线程执行完毕后，num 会在离开作用域时释放锁，从而让其他线程得到获取锁的机会
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;
    //     });
    //
    //     handles.push(handle);
    // }
    // // 在主线程中收集了所有的线程句柄，并通过逐一调用句柄的 join 方法来确保所有生成的线程执行完毕
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    // // 最后，主线程会获取锁并打印出程序的结果
    // println!("Result: {}", *counter.lock().unwrap());

    // 想法虽然美好，但是上面的代码无法通过编译，编译器输出如下：
    // error[E0382]: use of moved value: `counter`
    //   --> src\main.rs:47:36
    //    |
    // 43 |     let counter = Mutex::new(0);
    //    |         ------- move occurs because `counter` has type `Mutex<i32>`, which does not implement the `Copy` trait
    // ...
    // 47 |         let handle = thread::spawn(move || {
    //    |                                    ^^^^^^^ value moved into closure here, in previous iteration of loop
    // 48 |             let mut num = counter.lock().unwrap();
    //    |                           ------- use occurs due to use in closure

    // 使用了移动的值 counter
    // 报错信息不是很明显
    // 不使用 for 循环创建线程，我们手动创建两个线程，然后依旧将 counter 传入闭包中

    // let handle = thread::spawn(move || {
    //     let mut num = counter.lock().unwrap();
    //     *num += 1;
    // });
    //
    // handles.push(handle);
    //
    // let handle2 = thread::spawn(move || {
    //     let mut num2 = counter.lock().unwrap();
    //     *num2 += 1;
    // });
    // handles.push(handle2);
    //
    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // 上面的代码依旧编译不通过，报错信息如下：
    //     error[E0382]: use of moved value: `counter`
    //   --> src\main.rs:93:33
    //    |
    // 46 |     let counter = Mutex::new(0);
    //    |         ------- move occurs because `counter` has type `Mutex<i32>`, which does not implement the `Copy` trait
    // ...
    // 86 |     let handle = thread::spawn(move || {
    //    |                                ------- value moved into closure here
    // 87 |         let mut num = counter.lock().unwrap();
    //    |                       ------- variable moved due to use in closure
    // ...
    // 93 |     let handle2 = thread::spawn(move || {
    //    |                                 ^^^^^^^ value used here after move
    // 94 |         let mut num2 = counter.lock().unwrap();
    //    |                        ------- use occurs due to use in closure

    // 上面保报错的信息说明：counter 已经移动到 handle 线程的闭包中，因此，我们不应该在 handle2 中使用

    // 使用 Rc<T> 指针解决所有权问题

    // 使用 Rc 指针包裹 Mutex 实例
    // let counter = Rc::new(Mutex::new(0));
    //
    // let mut handles = vec![];
    //
    // for _ in 0..10 {
    //
    //     // 对 Rc<T> 实例进行复制，使得多个线程可以持有 counter 所有权
    //     let counter = Rc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    // println!("Result: {}", *counter.lock().unwrap());

    // 编译上面的代码，还是报错，报错信息如下：
    // error[E0277]: `Rc<Mutex<i32>>` cannot be sent between threads safely
    //    --> src\main.rs:134:36
    //     |
    // 134 |           let handle = thread::spawn(move || {
    //     |                        ------------- ^------
    //     |                        |             |
    //     |  ______________________|_____________within this `[closure@src\main.rs:134:36: 134:43]`
    //     | |                      |
    //     | |                      required by a bound introduced by this call
    // 135 | |             let mut num = counter.lock().unwrap();
    // 136 | |             *num += 1;
    // 137 | |         });
    //     | |_________^ `Rc<Mutex<i32>>` cannot be sent between threads safely
    //     |
    //     = help: within `[closure@src\main.rs:134:36: 134:43]`, the trait `Send` is not implemented for `Rc<Mutex<i32>>`
    // note: required because it's used within this closure
    //    --> src\main.rs:134:36
    //     |
    // 134 |         let handle = thread::spawn(move || {
    //     |                                    ^^^^^^^
    // note: required by a bound in `spawn`
    //    --> C:\Users\wzy\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\std\src\thread\mod.rs:712:8
    //     |
    // 712 |     F: Send + 'static,
    //     |

    // 编译出错的原因：    `Rc<Mutex<i32>>` cannot be sent between threads safely
    // Rc<Mutex<i32>> 类型不能在线程之间安全的传递
    // the trait `Send` is not implemented for `Rc<Mutex<i32>>`
    //     `Rc<Mutex<i32>> 类型没有实现 Send trait
    // 因为 Rc<T> 不能用于线程之间的数据传递
    // Rc<T>在跨线程使用时并不安全。当Rc<T>管理引用计数时，它会在每次调用clone的过程中增加引用计数
    // 并在克隆出的实例被丢弃时减少引用计数，但它并没有使用任何并发原语来保证修改计数的过程不会被另一个线程所打断
    // 这极有可能导致计数错误并产生诡异的bug，比如内存泄漏或值在使用时被莫名其妙地提前释放
    // 我们需要的是一个行为与Rc<T>一致，且能够保证线程安全的引用计数类型

    // 原子引用计数 Arc<T>
    // 幸运的是，我们拥有一种被称为 Arc<T> 的类型，它既拥有类似于 Rc<T> 的行为，又保证了自己可以被安全地用于并发场景
    // 它名称中的 A 代表着原子（ atomic ）， 表明自己是一个原子引用计数（atomically reference counted）类型
    // 原子是一种新的并发原语，我们可以参考标准库文档中的 std::sync::atomic 部分来获得更多相关信息
    // 我们现在只需要知道：原子和原生类型的用法十分相似，并且可以安全地在多个线程间共享

    // Arc<T> 与 Rc<T> 类型的用法一样
    let counter = Arc::new(Mutex::new(0));

        let mut handles = vec![];

        for _ in 0..10 {

            // 对 Rc<T> 实例进行复制，使得多个线程可以持有 counter 所有权
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }

        // Result: 10
        println!("Result: {}", *counter.lock().unwrap());

    // 虽然 counter 本身不可变，但我们仍然能够获取其内部值的可变引用
    // 这意味着，Mutex<T>与Cell系列类型有着相似的功能，它同样提供了内部可变性
    // RefCell<T> 来改变 Rc<T> 中的内容，而上面的方式使用 Mutex<T> 来改变 Arc<T> 中的内容
    //
    //
    // 另外还有一个值得注意的细节是，Rust 并不能使你完全避免使用Mutex<T>过程中所有的逻辑错误
    // 使用 Rc<T> 会有产生循环引用的风险。两个 Rc<T> 值在互相指向对方时会造成内存泄漏
    // 与之类似，使用 Mutex<T>也会有产生死锁 （deadlock）的风险
    // 当某个操作需要同时锁住两个资源，而两个线程分别持有其中一个锁并相互请求另外一个锁时，这两个线程就会陷入无穷尽的等待过程
}

// 允许线程间转移所有权的 Send trait

// 只有实现了 Send trait 的类型才可以安全地在线程间转移所有权
// 除了 Rc<T> 等极少数的类型，几乎所有的Rust类型都实现了Send trait：如果你将克隆后的Rc<T>值的所有权转移到了另外一个线程中
// 那么两个线程就有可能同时更新引用计数值并进而导致计数错误。因此，Rc<T>只被设计在单线程场景中使用，它也无须为线程安全付出额外的性能开销

// 因此，Rust 的类型系统与 trait 约束能够阻止我们意外地跨线程传递 Rc<T> 实例
// 使用 Rc<T> 这类操作时立马触发了编译时错误： the trait Send is not implemented forRc&t;Mutex<i32>>
// 而当我们切换到实现了 Send 的 Arc<T> 后，那段代码就顺利地编译通过了

// 任何完全由 Send 类型组成的复合类型都会被自动标记为 Send。除了裸指针
// 几乎所有的原生类型都满足Send 约束


// 允许多线程同时访问的 Sync trait

// 只有实现了 Sync trait 的类型才可以安全地被多个线程引用
// 换句话说，对于任何类型 T，如果 &T（也就是T的引用）满足约束 Send，那么T就是满足 Sync 的
// 这意味着 T 的引用能够被安全地传递至另外的线程中
// 与 Send 类似，所有原生类型都满足 Sync 约束，而完全由满足 Sync 的类型组成的复合类型也都会被自动识别为满足 Sync 的类型
// 智能指针 Rc<T> 同样不满足Sync约束，其原因与它不满足 Send 约束类似
// RefCell<T> 类型及 Cell<T> 系列类型也不满足Sync 约束
// RefCell<T> 实现的运行时借用检查并没有提供有关线程安全的保证
// 智能指针 Mutex<T> 是 Sync 的，可以被多个线程共享访问


// 手动实现 Send 和 Sync 是不安全的

// 当某个类型完全由实现了 Send 与 Sync 的类型组成时，它就会自动实现 Send 与 Sync
// 因此，我们并不需要手动地为此种类型实现相关 trait
// 作为标签 trait，Send 与 Sync 甚至没有任何可供实现的方法

// 它们仅仅被用来强化与并发相关的不可变性。

// 手动实现这些 trait 涉及使用特殊的不安全Rust代码。
// 需要注意的是，当你构建的自定义并发类型包含了没有实现 Send 或 Sync 的类型时
// 你必须要非常谨慎地确保设计能够满足线程间的安全性要求
// Rust 官方网站中的 The Rustonomicon 文档详细地讨论了此类安全性保证及如何满足安全性要求的具体技术