// 将固定的 Future 变为 Unpin
// 我们前面提到过 async 函数返回的 Future 默认就是 !Unpin 的
//
// 在实际应用中，一些函数会要求它们处理的 Future 是 Unpin 的
// 此时，若你使用的 Future 是 !Unpin 的，必须要使用以下的方法先将 Future 进行固定:
// 1. Box::pin， 创建一个 Pin<Box<T>>
// 2. pin_utils::pin_mut!，创建一个 Pin<&mut T>
//
// 固定后获得的 Pin<Box<T>> 和 Pin<&mut T> 既可以用于 Future ，又会自动实现 Unpin

// 举例如下：
// use pin_utils::pin_mut; // `pin_utils` 可以在crates.io中找到
//
// // 函数的参数是一个 Future，但是要求该 Future 实现 Unpin
// fn execute_unpin_future(x: impl Future<Output = ()> + Unpin) { /* ... */ }
//
//
//
// let fut = async { /* ... */ };
//
// // 下面代码报错: 默认情况下，`fut` 实现的是 !Unpin ，并没有实现 Unpin
// // execute_unpin_future(fut);
//
//
// // 使用`Box`进行固定
// let fut = async { /* ... */ };
// let fut = Box::pin(fut);
// execute_unpin_future(fut); // OK
//
//
// // 使用`pin_mut!`进行固定
// let fut = async { /* ... */ };
// pin_mut!(fut);
// execute_unpin_future(fut); // OK

// 总结
//
// 1. 若 T: Unpin ( Rust 类型的默认实现)，那么 Pin<'a, T> 跟 &'a mut T 完全相同，也就是 Pin 将没有任何效果, 该移动还是照常移动
// 2. 绝大多数标准库类型都实现了 Unpin ，事实上，对于 Rust 中你能遇到的绝大多数类型，该结论依然成立 ，其中一个例外就是：async/await 生成的 Future 没有实现 Unpin
// 3. 你可以通过以下方法为自己的类型添加 !Unpin 约束：
//    1. 使用文中提到的 std::marker::PhantomPinned
//    2. 使用 nightly 版本下的 feature flag
// 4. 可以将值固定到栈上，也可以固定到堆上
//    1. 将 !Unpin 值固定到栈上需要使用 unsafe，使用 std::pin::Pin 和 Pin::new_unchecked
//    2. 将 !Unpin 值固定到堆上无需 unsafe，可以通过 Box::pin 来简单的实现
// 5. 当固定类型 T: !Unpin 时，你需要保证数据从被固定到被 drop 这段时期内，其内存不会变得非法或者被重用


fn main() {

}