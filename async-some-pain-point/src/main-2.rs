// 2. async 函数和 Send trait

// 前面说过， Send trait 对于多线程间数据传递的重要性，对于 async fn 也是如此
// 它返回的 Future 能否在线程间传递的关键在于 .await 运行过程中，作用域中的变量类型是否是 Send

// 智能指针 Rc 类型无法在多线程环境下使用，因为它没有实现 send trait

use std::rc::Rc;

#[derive(Default)]
struct NotSend(Rc<()>);

async fn bar() {}

async fn foo() {
    NotSend::default();
    bar().await
}

async fn foo2() {
    let x = NotSend::default();
    bar().await
}

async fn foo3() {
    {
        let x = NotSend::default();
    }
    bar().await
}

fn required_send(_: impl Send) {}

fn main() {
    // 即使 foo 返回的 Future 是 Send
    // 但是在它内部短暂的使用 NotSend 依然是安全的，原因在于它的作用域并没有影响到 .await
    // 只是简单的调用 NotSend，并没有使用任何变量接收
    // 所以在 .await 的作用域内，没有任何没有使用 Send trait 的变量
    // required_send(foo());

    // 在 foo 函数中声明一个 NotSend 类型的变量 x
    // 此时在 required_send 中调用 foo2，就会发生编译错误：
    // error: future cannot be sent between threads safely
    //   --> src\main-2:35:19
    //    |
    // 35 |     required_send(foo2());
    //    |                   ^^^^^^ future returned by `foo2` is not `Send`
    //    |
    //    = help: within `impl Future<Output = ()>`, the trait `Send` is not implemented for `Rc<()>`
    // note: future is not `Send` as this value is used across an await
    //   --> src\main-2:22:10
    //    |
    // 21 |     let x = NotSend::default();
    //    |         - has type `NotSend` which is not `Send`
    // 22 |     bar().await
    //    |          ^^^^^^ await occurs here, with `x` maybe used later
    // 23 | }
    //    | - `x` is later dropped here
    // note: required by a bound in `required_send`
    //   --> src\main-2:25:26
    //    |
    // 25 | fn required_send(_: impl Send) {}
    //    |                          ^^^^ required by this bound in `required_send`
    // required_send(foo2());
    // 提示很清晰，.await在运行时处于 x 的作用域内
    // 前面有提到过，.await 有可能被执行器调度到另一个线程上运行，而 Rc 并没有实现 Send，因此编译器无情拒绝了我们
    //
    // 其中一个可能的解决方法是在 .await 之前就使用 std::mem::drop 释放掉 Rc，但是很可惜，截止今天，该方法依然不能解决这种问题

    // 也就是说，x 没有实现 Send trait，但是 .await 可能会被调度器调度到另外一个线程上运行，那么 x 是无法被转移到另外一个线程上，所以编译器就报错了

    // 目前的解决方法是：使用大括号 {...} 将 x 包裹起来，在大括号外使用 .await 执行异步任务
    // 当语句块结束时，变量会自动被 Drop，所以 x 也就被 drop
    required_send(foo3());
}