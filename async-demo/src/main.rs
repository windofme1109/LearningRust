// Rust 异步编程的原理

// Rust 中，异步编程的核心是 Future trait
// Future 定义：能产出值的异步计算（该值有可能为空，例如()）


use std::pin::Pin;
use std::task::Context;

// 一个简化版的 Future 的定义
trait SimpleFuture {
    type Output;

    // Future 需要被执行器 poll （轮询）后才能运行
    // poll 函数就用来进行轮询，推进 Future 进一步执行，直到有结果为止
    // 因为 Future 不能保证在一次 poll 中就获得结果，所以需要不停的 poll
    // 例如异步地读取文件，因为我们不知道什么时候能读取完，所以需要不停地查询，直到完全读取完

    // 若在当前 poll 中， Future 可以被完成，则会返回 Poll::Ready(result) ，反之则返回 Poll::Pending， 并且安排一个 wake 函数
    // 当未来 Future 准备好进一步执行时，该函数会被调用，然后管理该 Future 的执行器(例如 block_on函数)会再次调用 poll 方法
    // 此时 Future 就可以继续执行
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;

    // 如果没有 wake 方法，那执行器无法知道某个 Future 是否可以继续被执行，除非执行器定期的轮询每一个 Future，确认它是否能被执行，但这种作法效率较低
    // 而有了 wake，Future 就可以主动通知执行器，然后执行器就可以精确的执行该 Future
    // 这种 事件通知 -> 执行 的方式要远比定期对所有 Future 进行一次全遍历来的高效

    // 类似于 JavaScript 中的 Generator 自动执行机

}

enum Poll<T> {
    Ready(T),
    Pending
}

// 示例 1：从 socket 中读取数据

// 过程：
// 1. 如果有数据，可以直接读取数据并返回 Poll::Ready(data)
// 2. 如果没有数据，Future 会被阻塞且不会再继续执行，此时它会注册一个 wake 函数
//    当 socket 数据准备好时，该函数将被调用以通知执行器：我们的 Future 已经准备好了，可以继续执行

pub struct SocketRead<'a> {
    socket: &'a Socket,
}

impl SimpleFuture for SocketRead<'_> {
    type Output = Vec<u8>;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if self.socket.has_data_to_read() {
            // socket有数据，写入buffer中并返回
            Poll::Ready(self.socket.read_buf())
        } else {

            // socket 中没有数据，那么注册一个 wake 函数
            // 当数据可用时，该函数就会被调用
            // 当前的 Future 会继续调用 poll 方法，此时就可以读到数据
            self.socket.set_readable_callback(wake);
            Poll::Pending
        }
    }
}


// 这里由一个疑问，set_readable_callback 方法到底是怎么工作的？怎么才能知道 socket 中的数据已经可以被读取了
// 其中一个简单粗暴的方法就是使用一个新线程不停的检查 socket 中是否有了数据
// 当有了后，就调用 wake() 函数。该方法确实可以满足需求，但是性能着实太低了，需要为每个阻塞的 Future 都创建一个单独的线程！

// 在现实世界中，该问题往往是通过操作系统提供的 IO 多路复用机制来完成
// 例如 Linux 中的 epoll，FreeBSD 和 macOS 中的 kqueue ，Windows 中的 IOCP, Fuchisa 中的 ports 等(可以通过 Rust 的跨平台包 mio 来使用它们)
// 借助 IO 多路复用机制，可以实现一个线程同时阻塞地去等待多个异步 IO 事件，一旦某个事件完成就立即退出阻塞并返回数据
// 其实现类似于下面的代码：
struct IoBlocker {
    /* ... */
}

struct Event {
    // Event的唯一ID，该事件发生后，就会被监听起来
    id: usize,

    // 一组需要等待或者已发生的信号
    signals: Signals,
}

impl IoBlocker {
    /// 创建需要阻塞等待的异步IO事件的集合
    fn new() -> Self { /* ... */ }

    /// 对指定的IO事件表示兴趣
    fn add_io_event_interest(
        &self,

        /// 事件所绑定的socket
        io_object: &IoObject,

        event: Event,
    ) { /* ... */ }

    /// 进入阻塞，直到某个事件出现
    fn block(&self) -> Event { /* ... */ }
}

let mut io_blocker = IoBlocker::new();

// 将 socket 于指定事件绑定
io_blocker.add_io_event_interest(
    &socket_1,
    Event { id: 1, signals: READABLE },
);

io_blocker.add_io_event_interest(
    &socket_2,
    Event { id: 2, signals: READABLE | WRITABLE },
);

let event = io_blocker.block();

// 当socket的数据可以读取时，打印 "Socket 1 is now READABLE"
println!("Socket {:?} is now {:?}", event.id, event.signals);

// 这样，我们只需要一个执行器线程，它会接收 IO 事件并将其分发到对应的 Waker 中
// 接着后者会唤醒相关的任务，最终通过执行器 poll 后，任务可以顺利地继续执行, 这种 IO 读取流程可以不停的循环，直到 socket 关闭





// 示例 2：运行多个 Future 或链式调用多个 Future ，也可以通过无内存分配的状态机实现

// Join 用来并发的执行两个 Future
// 之所以可以并发，是因为两个Future的轮询可以交替进行，一个阻塞，另一个就可以立刻执行
pub struct Join<FutureA, FutureB> {
    // 结构体的每个字段都包含一个 Future，可以运行直到完成
    // 等到 Future 完成后，字段会被设置为 None，这样 Future 完成后，就不会再被轮询
    a: Option<FutureA>,
    b: Option<FutureB>
}

impl<FutureA, FutureB> SimpleFuture for Join<FutureA, FutureB>
where FutureA: SimpleFuture, FutureB: SimpleFuture {
    type Output = ();

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {

        // 尝试完成一个 Future a
        // 如果 self.a 是 Some 变体，我们就取出 Some 变体包裹的值 fa
        // 因为 fa 显然是 FutureA，那么我们继续嗲用 poll 方法，并传入 wake 函数
        // 然后继续判断 fa.poll 函数的返回值是不是 Poll::Ready
        // 如果是，证明 FutureA 执行完成，那么我们就调用 take 方法，将 self.a 置为 None
        // 如果 fa.poll 函数的返回值是 Poll::Pending，证明 FutureA 还未完成，因此跳出当前的执行过程，转而执行 FutureB
        if let Some(fa) = &mut self.a {
            if let Poll::Ready(()) = fa.poll(wake) {
                self.a.take()
            }
        }


        // 尝试完成一个 Future b
        // 如果 self.b 是 Some 变体，我们就取出 Some 变体包裹的值 fb
        // 因为 fb 显然是 FutureB，那么我们继续嗲用 poll 方法，并传入 wake 函数
        // 然后继续判断 fb.poll 函数的返回值是不是 Poll::Ready
        // 如果是，证明 FutureB 执行完成，那么我们就调用 take 方法，将 self.b 置为 None
        // 如果 fb.poll 函数的返回值是 Poll::Pending，证明 FutureB 还未完成，因此跳出当前的执行过程，转而执行 FutureA
        if let Some(fb) = &mut self.b {
            if let Poll::Ready(()) = fb.poll(wake) {
                self.b.take()
            }
        }

        // 如果 self.a 和 self.b 同时为 None，说明两个 Future 执行完成
        // 我们就返回 Poll::Ready(())
        // 否则至少还有一个 Future 没有完成任务，因此返回 Poll::Pending
        // 当该 Future 再次准备好时，通过调用`wake()`函数来继续执行就返回 Poll::Pending
        if self.a.is_none() && self.b.is_none() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }

    }
}

// 示例 3：多个Future也可以一个接一个的连续运行
// 注意: 由于本例子用于演示，因此功能简单， AndThenFut 会假设两个 Future 在创建时就可用了
// 而真实的 Andthen 允许根据第一个 Future 的输出来创建第二个 Future ，因此复杂的多。
pub struct AndThenFut<FutureA, FutureB> {
    first: Option<FutureA>,
    second: FutureB,
}

impl<FutureA, FutureB> SimpleFuture for AndThenFut<FutureA, FutureB>
where FutureA: SimpleFuture<Output = ()>,
      FutureB: SimpleFuture<Output = ()>,
{
    type Output = ();
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        // 尝试获取 first 这个 Future 的状态
        if let Some(f) = &mut self.first {
            // 模式匹配，first 的 poll 函数的执行结果
            match f.poll(wake) {
                // 如果是 Ready 变体，那么证明已经完成了第一个 Future， 可以将它移除， 然后准备开始运行第二个
                Poll::Ready(()) => {
                    self.first.take()
                },
                // 第一个 Future 还不能完成
                Poll::Pending => {
                    return Poll::Pending;
                }
            }
        }

        // 第一个 Future 执行完成
        // 开始执行第二个 Future
        self.second.poll(wake)
    }
}


// 真实的 Future trait
trait Future {
    type Output;
    fn poll(
        // 同 SimpleFuture 相比，self 的类型从 &mut self 变成了 Pin<&mut Self>
        self: Pin<&mut Self>,
        // wake: fn() 修改为 cx: &mut Context<'_>
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output>;
}

// 首先这里多了一个 Pin ，现在只需要知道使用它可以创建一个无法被移动的 Future，因为无法被移动，所以它将具有固定的内存地址，意味着我们可以存储它的指针(如果内存地址可能会变动，那存储指针地址将毫无意义！)
// 也意味着可以实现一个自引用数据结构: struct MyFut { a: i32, ptr_to_a: *const i32 }
// 而对于 async/await 来说，Pin 是不可或缺的关键特性
//
// 从 wake: fn() 变成了 &mut Context<'_>，意味着 wake 函数可以携带数据了（Context 提供对 waker 的访问）
// 为何要携带数据？考虑一个真实世界的场景，一个复杂应用例如 web 服务器可能有数千连接同时在线
// 那么同时就有数千 Future 在被同时管理着，如果不能携带数据，当一个 Future 调用 wake 后
// 执行器该如何知道是哪个 Future 调用了 wake ,然后进一步去 poll 对应的 Future？

// 正式场景要进行 wake，就必须携带上数据。而 Context 类型通过提供一个 Waker 类型的值，就可以用来唤醒特定的的任务

// 使用 Waker 来唤醒任务
//
// 对于 Future 来说，第一次被 poll 时无法完成任务是很正常的
// 但它需要确保在未来一旦准备好时，可以通知执行器再次对其进行 poll 进而继续往下执行，该通知就是通过 Waker 类型完成的。
//
// Waker 提供了一个 wake() 方法可以用于告诉执行器：相关的任务可以被唤醒了
// 此时执行器就可以对相应的 Future 再次进行 poll 操作
fn main() {

}