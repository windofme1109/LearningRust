// 递归使用 async fn

// 在内部实现中，async fn 被编译成一个状态机，这会导致递归使用 async fn 变得较为复杂
// 因为编译后的状态机还需要包含自身

async fn step_one() {}
async fn step_two() {}

async fn foo() {
    step_one().await;
    step_two().await;
}

// 上面的 foo 函数会被编译为下面的代码：
// enum Foo {
//     First(StepOne),
//     Second(StepTwo)
// }
// 就是说，异步过程要有一个确定的状态
// 因此，一个递归的 async 函数，会被编译成如下形式：
// async fn recursive() {
//     recursive().await;
//     recursive().await;
// }

// 编译形式如下：
// enum Recursive {
//     First(Recursive),
//     Second(Recursive),
// }

// 编译过程中会发生错误，报错信息如下：
// error[E0733]: recursion in an `async fn` requires boxing
//   --> src\main-3:21:22
//    |
// 21 | async fn recursive() {
//    |                      ^ recursive `async fn`
//    |
//    = note: a recursive `async fn` must be rewritten to return a boxed `dyn Future`
//    = note: consider using the `async_recursion` crate: https://crates.io/crates/async_recursion

// 因为在递归额过程中还要包含自己，所以这是典型的动态大小类型，它的大小会无限增长，即 Rust 无法在编译时确定其大小，所以会报错
// 报错信息显示：在递归一个 async 函数过程中需要将其装箱
// 即使用 Box 将 async 函数放到堆上而不是栈上

// 注意：就算是使用 Box，这里也大有讲究
// 如果我们试图使用 Box::pin 这种方式去包裹是不行的
// 因为编译器自身的限制限制了我们
// 为了解决这种问题，我们只能将 recursive 转变成一个正常的函数，该函数返回一个使用 Box 包裹的 async 语句块
//
// 也就是说，不能直接使用 Box::pin 去包裹一个 async 函数
// 而是 使用 Box 包裹 async 代码块
//

use futures::future::{BoxFuture, FutureExt};
fn recursive() -> BoxFuture<'static, ()>{
    async move {
        recursive().await;
        recursive().await;
    }.boxed()
}

fn main() {

}


