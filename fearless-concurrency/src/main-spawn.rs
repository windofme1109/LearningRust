
use std::thread;
use std::time::Duration;
fn main() {

    // 调用thread::spawn函数来创建线程，它接收一个闭包作为参数
    // 该闭包会包含我们想要在新线程（生成线程）中运行的代码

    // handler 接收 thread::spawn 返回的结果，thread::spawn 返回的是 JoinHandle
    // join 句柄提供了一个 [join] 方法，可用于连接生成的线程
    // let handler = thread::spawn(|| {
    //     for i in 1..10 {
    //         // 在新线程（非主线程）中执行
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // 在主线程打印信息之前调用 join 方法 
    // handler.join().unwrap();

    // 输出如下：
    // hi number 1 from the spawned thread!
    // hi number 2 from the spawned thread!
    // hi number 3 from the spawned thread!
    // hi number 4 from the spawned thread!
    // hi number 5 from the spawned thread!
    // hi number 6 from the spawned thread!
    // hi number 7 from the spawned thread!
    // hi number 8 from the spawned thread!
    // hi number 9 from the spawned thread!
    // hi number 1 from the main thread!
    // hi number 2 from the main thread!
    // hi number 3 from the main thread!
    // hi number 4 from the main thread!

    // 主线程会等待新线程执行完毕后才开始执行自己的 for 循环，所以它的输出将不再出现交替的情形

    // 调用了 join 方法，那么主线程一定会等到新线程执行完毕再结束，而二者的执行顺序取决于 jion 的调用位置

    // main 运行在主线程中
    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // 需要注意的是，只要这段程序中的主线程运行结束，创建出的新线程就会相应停止
    // 而不管它的打印任务是否完成。每次运行这段程序都有可能产生不同的输出

    // 调用 thread::sleep 会强制当前的线程停止执行一小段时间，并允许一个不同的线程继续运行
    // 这些线程可能会交替执行，但我们无法对它们的执行顺序做出任何保证
    // 执行顺序由操作系统的线程调度策略决定在上面这次运行中
    
    // 使用 join 句柄等待所有线程结束

    // 由于主线程的停止，上面的代码会在大部分情形下提前终止新线程，它甚至不能保证新线程一定会得到执行
    // 这同样是因为我们无法对线程的执行顺序做出任何保证而导致的！
    
    // 我们可以通过将 thread::spawn 返回的结果保存在一个变量中，来避免新线程出现不执行或不能完整执行的情形
    // thread::spawn 的返回值类型是一个自持有所有权的 JoinHandle，调用它的 join 方法可以阻塞当前线程直到对应的新线程运行结束
   
    // 线程句柄上调用 join 函数会阻塞当前线程，直到句柄代表的线程结束
    // 阻塞线程意味着阻止一个线程继续运行或使其退
    // handler.join().unwrap();

    // 使用了 join 方法后，打印的信息如下：
    // hi number 1 from the main thread!
    // hi number 1 from the spawned thread!
    // hi number 2 from the main thread!
    // hi number 2 from the spawned thread!
    // hi number 3 from the main thread!
    // hi number 3 from the spawned thread!
    // hi number 4 from the main thread!
    // hi number 4 from the spawned thread!
    // hi number 5 from the spawned thread!
    // hi number 6 from the spawned thread!
    // hi number 7 from the spawned thread!
    // hi number 8 from the spawned thread!
    // hi number 9 from the spawned thread!

    // 由于我们将 join 函数放置到了主线程的 for 循环之后
    // 所以主线程的 for 循环的代码正常执行
    // 交替打印主线程和新线程的东西
    // 等到主线程打印完成，就专注打印新线程的东西
    // 主线程只会在新线程运行结束后退出


    // 在线程中使用move闭包
    // move 闭包常常被用来与 thread::spawn 函数配合使用，它允许你在某个线程中使用来自另一个线程的数据
    // 你可以在闭包的参数列表前使用 move 关键字来强制闭包从外部环境中捕获值的所有权
    // 这一技术在我们创建新线程时尤其有用，它可以跨线程地传递某些值的所有权
    
    // let v = vec![1, 2, 3];

    // let handle = thread::spawn(|| {
    //     println!("Here's a vector: {:?}", v);
    // });

    // handle.join().unwrap();
    // 由于代码中的闭包使用了 v，所以它会捕获v并使其成为闭包环境的一部分
    // 又因为 thread::spawn 会在新线程中运行这个闭包，所以我们应当能够在新线程中访问 v
    // 但实际上，会编译出错：
    //  closure may outlive the current function, but it borrows `v`, which is owned by the current function
    // 上面的报错信息说的是：闭包可能在当前的函数之外存活，但是它借用的 v 却是被当前函数拥有

    // Rust 不知道新线程会运行多久，所以它无法确定 v 的引用是否一直有效

    // 下面的代码展示了在新线程中引用了 v，但是 v 在线程开始运行前被丢弃了
    // let v = vec![1, 2, 3];
    // let handle = thread::spawn(|| {
    //     println!("Here's a vector: {:?}", v);
    // });
    // drop(v); // 情况不妙
    // handle.join().unwrap();

    // 如果Rust允许我们运行这段代码，那么新线程有极大的概率会在创建后被立即置入后台，不再被执行
    // 此时的新线程在内部持有了 v 的引用，但主线程却已经通过 drop 函数将 v 丢弃了
    // 当新线程随后开始执行时，v 和指向它的引用全部失效了
    
    // 那么怎么解决这个问题呢，答案是使用 move 关键字定义闭包
    // 然后将外界变量的所有权转移到闭包内
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    drop(v); // 情况不妙
    handle.join().unwrap();
    // 使用，那么随后在主线程中调用 drop 会发生什么呢？而添加 move 又是否能够修复该示例中的编译错误呢？
    // 遗憾的是，当我们在闭包上添加 move 后，调用 drop 函数删除 v，同样会编译失败
    // 这是因为 move 将 v 移动到了闭包的环境中，所以我们无法在主线程中继续使用它来调用 drop 函数了

    // 输出的错误信息如下：
    // error[E0382]: use of moved value: `v`
    //    --> src/main-spawn:126:10
    //    |
    // 122 |     let v = vec![1, 2, 3];
    //    |         - move occurs because `v` has type `Vec<i32>`, which does not implement the `Copy` trait
    // 123 |     let handle = thread::spawn(move || {
    //    |                                ------- value moved into closure here
    // 124 |         println!("Here's a vector: {:?}", v);
    //    |                                           - variable moved due to use in closure
    // 125 |     });
    // 126 |     drop(v); // 情况不妙
    //    |          ^ value used here after move

    // 因为 Rust 只会在新线程中保守地借用 v，这也就意味着主线程可以从理论上让新线程持有的引用失效
    // 通过将 v 的所有权转移给新线程，我们就可以向 Rust 保证主线程不会再次使用 v
    // 如果我们在主线程中使用 v，那么就违反所有权规则
}
