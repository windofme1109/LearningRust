use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// 异步编程中的 Pin 与 Unpin

// 在 Rust 中，所有的类型可以分为两类:
// 1. 类型的值可以在内存中安全地被移动，例如数值、字符串、布尔值、结构体、枚举，总之你能想到的几乎所有类型都可以落入到此范畴内
// 2. 自引用类型，数据及其引用存放在一处。也就是说如果一个结构，引用或者指针指向了自己的数据，那么它就是一个自引用结构
//    关于自引用，可以参考：https://zhuanlan.zhihu.com/p/600784379
//    https://zhuanlan.zhihu.com/p/601908172                 
// 例如：
// 这里使用裸指针定义了一个自引用结构
struct Test {
    a: String,
    b: *const String,
}

// a 是一个字符串
// 而 b 则是这个字符串的原始指针
// a 的数据来自于 new 关联函数的参数 txt，b 在 new 的时候是一个空指针，在 init 的时候变成 a 字段的指针
// a 和 b 俩方法返回的数据理论上是一样的(b的错误场景不考虑)

// 注意：这里只能是原始指针，如果是引用指针就需要声明生命周期才行，但是这种自引用类型是没有办法的。



impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        // 获取 a 的原始指针
        let self_ref: *const String = &self.a;
        self.b = self_ref;
    }

    fn a(&self) -> &str {
        &self.a
    }

    fn b(&self) -> &String {
        assert!(!self.b.is_null(), "Test::b called without Test::init being called first");
        unsafe { &*(self.b) }
    }
}


// async / .await 是如何工作的

// 定义一个 async 代码块
// fut_one 和 fut_two 是两个 Future
// let fut_one = /* ... */; // Future 1
// let fut_two = /* ... */; // Future 2
// async move {
//     fut_one.await;
//     fut_two.await;
// }

// 在底层，async 会创建一个实现了 Future 的匿名类型，并实现 poll 方法

struct AsyncFuture {
    fut_one: FutOne,
    fut_two: FutTwo,
    state: State
}

enum State {
    AwaitingFutOne,
    AwaitingFutTwo,
    Done,
}

impl Future for AsyncFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            match self.state {
                State::AwaitingFutOne => match self.fut_one.poll() {
                    Poll::Ready(()) => self.state = State::AwaitingFutTwo,
                    Poll::Pending => return Poll::Pending
                },
                State::AwaitingFutTwo => match self.fut_two.poll() {
                    Poll::Ready(()) => self.state = State::Done,
                    Poll::Pending => return Poll::Pending
                },
                State::Done => {
                    Poll::Ready(())
                }
            }
        }
    }
}

// 当 poll 第一次被调用时，它会去查询 fut_one 的状态，若 fut_one 无法完成，则 poll 方法会返回
// 未来对 poll 的调用将从上一次调用结束的地方开始。该过程会一直持续，直到 Future 完成为止
// 去同一个位置去寻找

// 如果我们在 async 代码块中使用了引用类型，如下所示：
async {
    let mut x = [0; 128];
    let read_into_buf_fut = read_into_buf(&mut x);
    read_into_buf_fut.await;
    println!("{:?}", x);
}

// x 指向了一个缓冲区
// 上面的这段代码编译完以后，形式如下：
struct ReadIntoBuf<'a> {
    buf: &'a mut [u8], // 指向下面的`x`字段
}

struct AsyncFuture {
    x: [u8; 128],
    read_into_buf_fut: ReadIntoBuf<'what_lifetime?>,
}

// ReadIntoBuf 拥有一个引用字段，指向了结构体的另一个字段 x
// ReadIntoBuf.buf 是对 x 的一个引用
// 一旦 AsyncFuture 被移动，那 x 的地址也将随之变化，此时对 x 的引用就变成了不合法的
// 也就是 read_into_buf_fut.buf 会变为不合法的
//
// 若能将 Future 在内存中固定到一个位置，就可以避免这种问题的发生，也就可以安全的创建上面这种引用类型



// Unpin
// 事实上，绝大多数类型都不在意是否被移动(开篇提到的第一种类型)，因此它们都自动实现了 Unpin 特征
// 从名字上看，Pin 和 Unpin 应该是 trait，但实际上，Pin 不按套路出牌，它是一个结构体：
//
// pub struct Pin<P> {
//     pointer: P,
// }

// 它包裹了一个指针，并确保该指针指向的数据不会被移动，例如 Pin<&mut T> , Pin<&T> , Pin<Box<T>> ，都能确保 T 不会被移动
// 而 Unpin 才是一个 trait，它表明一个类型可以随意被移动，那么问题来了，可以被 Pin 住的值，它有没有实现什么特征呢
// 答案是：可以被 Pin 住的值实现的 trait 是 !Unpin

// 这个 trait 以前没有见过，! 代表没有实现某个特征的意思，!Unpin 说明类型没有实现 Unpin 特征，那自然就可以被 Pin 了
// 也就是说，如果一个类型没有实现 Unpin trait，就可以被 pin 住

// 那么是不是说明，一个类型如果实现了 Unpin trait，就不能被 Pin 了？
//
// 其实，还是可以 Pin 的，毕竟它只是一个结构体，你可以随意使用，但是不再有任何效果而已，该值一样可以被移动！
//
// 例如 Pin<&mut u8> ，显然 u8 实现了 Unpin trait，它可以在内存中被移动
// 因此 Pin<&mut u8> 跟 &mut u8 实际上并无区别，一样可以被移动
//
// 因此，一个类型如果不能被移动，它必须实现 !Unpin 特征（不能实现 Unpin）

// 如果将 Unpin 与之前章节学过的 Send/Sync 进行下对比，会发现它们都很像：
//
// 1. 都是标记特征( marker trait )，该特征未定义任何行为，非常适用于标记
// 2. 都可以通过!语法去除实现
// 3. 绝大多数情况都是自动实现, 无需我们的操心

fn main() {
    // println!("Hello, world!");
    
    
    let mut test1 = Test::new("test1");
    let mut test2 = Test::new("test2");
    test1.init();
    test2.init();

    // a: test1, b: test1
    println!("a: {}, b: {}", test1.a(), test1.b());
    // a: test2, b: test2
    // println!("a: {}, b: {}", test2.a(), test2.b());

    // 自引用的问题

    // 交换 test1 和 test2 的值
    // swap 方法用来交换两个可变位置的值
    // 那么 test1 和 test2 指向的位置不变，但是值被交换了
    std::mem::swap(&mut test1, &mut  test2);
    // 交换以后，test2 指向的位置的值是：test1，理论上应该打印出：a: test1, b: test1
    // 但实际上，打印出的是：a: test1, b: test2
    println!("a: {}, b: {}", test2.a(), test2.b());
    // 出现上面的情况的原因是：
    // test2.b 指针依然指向了旧的地址，而该地址对应的值现在在 test1 里，最终会打印出意料之外的值
    // 我们可以做一个验证，修改 test1 中的 a 的值：
    test1.a = "I've totally changed now!".to_string();

    // a: test1, b: I've totally changed now!
    println!("a: {}, b: {}", test2.a(), test2.b());

    // test1 是一个结构体，里面由两个属性 a 和 b，因为 a 是字符串类型
    // 所以，a 存储的是一个内存地址，指向具体的字符串存放地址，而 b 是一个指向 a 的指针，所以，b 存储的就是 a 的内存地址
    // 假设 a 在栈内存的地址为 0x1002，其存储的值为 0x1111
    // 而 b 在栈内存的地址为 0x1003，其存储的值为 0x1002

    // 同理，test2 中 a 和 b 的位置以及存储的值的情况为：
    // a 在栈内存的地址为 0x2002，其存储的值为 0x2222
    // 而 b 在栈内存的地址为 0x2003，其存储的值为 0x2002

    // 交换 test1 和 test2，那么是对应属性的进行交换，也就是 a 和 a 交换，b 和 b 交换
    // 此时内存分布如下：
    // test1
    // a 在栈内存的地址为 0x1002，其存储的值为 0x2222
    // b 在栈内存的地址为 0x1003，其存储的值为 0x2002

    // test2
    // a 在栈内存的地址为 0x2002，其存储的值为 0x1111
    // b 在栈内存的地址为 0x2003，其存储的值为 0x1002

    // 由上面的指针分布情况可以看出，
    // test1.b 指向了 test2.a
    // test2.b 指向了 test1.a
    // 所以此时的 b 属性并没有指向属于自己结构体的 a 属性
    //
    // 所以，修改 test.a 的值，test2.b 也会跟着变化
}

// Pin 
// Pin 的本意是别针，那么其作用就很明显了，将某个类型别住，防止其移动
// 实际上，Pin 类型的主要作用是：防止一个类型在内存中被移动
// 上面的自引用类型存在的问题是，如果指针指向的内存地址变了，而指针依旧指向原来的内存地址，那么就会发生错误
