// 基于引用计数的智能指针Rc<T>
// 所有权在大多数情况下都是清晰的：对于一个给定的值，你可以准确地判断出哪个变量拥有它。
// 但在某些场景中，单个值也可能同时被多个所有者持有。
// 例如，在图数据结构中，多个边可能会指向相同的节点，而这个节点从概念上来讲就同时属于所有指向它的边
// 一个节点只要在任意指向它的边还存在时就不应该被清理掉


// Rust 提供了一个名为 Rc<T> 的类型来支持多重所有权，它名称中的 Rc 是 Reference counting（引用计数）的缩写
// Rc<T> 类型的实例会在内部维护一个用于记录值引用次数的计数器
// 从而确认这个值是否仍在使用。如果对一个值的引用次数为零，那么就意味着这个值可以被安全地清理掉，而不会触发引用失效的问题
// 注意，Rc<T> 只能被用于单线程场景中

use std::rc::Rc;

use crate::List::{Cons, Nil};

fn main() {
    // 使用Rc<T>共享数据
    // let a = Cons(
    //     5, 
    //     Box::new(Cons(
    //             10, 
    //             Box::new(Nil)
    //         )
    //     )
    // );

    // // 让 b 和 c 都链接到 链接列表 a 上面
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));

    // 直接使用 Box 装箱的方式进行链接，编译不通过
    // 输出信息如下：
    // error[E0382]: use of moved value: `a`
//   --> src/main.rs:26:30
//   |
// 16 |     let a = Cons(
//   |         - move occurs because `a` has type `List`, which does not implement the `Copy` trait
// ...
// 25 |     let b = Cons(3, Box::new(a));
//   |                              - value moved here
// 26 |     let c = Cons(4, Box::new(a));
//   |                              ^ value used here after move

// 在 b 进行链接的时候，a 的所有权已经移动到 Box::new 函数中了
// 在 c 进行链接的时候，就不能重复使用了

// 一种比较好的解决方式是：
// 可以将List中的 Box<T> 修改为 Rc<T>
// 每个 Cons 变体都会持有一个值及一个指向List的Rc<T>
// 我们只需要在创建b的过程中 克隆 a 的 Rc<List> 智能指针
// 而不再需要获取a的所有权。这会使a和b可以共享 Rc<List> 数据的所有权
// 并使智能指针中的引用计数从 1 增加到 2。随后，我们在创建 c 时也会同样克隆 a 并将引用计数从 2 增加到 3
// 每次调用 Rc::clone 都会使引用计数增加，而 Rc<List> 智能指针中的数据只有在引用计数器减少到0时才会被真正清理掉

// 创建了一个包含 5 和 10 的列表，并将这个新建的Rc<List>存入了a
// 将 Cons 变体使用 Rc 指针进行包裹，这样 a 也是一个 Rc 指针，指向 Cons 变量数据
    let a = Rc::new(Cons(
        5, 
        Rc::new(Cons(
                10, 
                Rc::new(Nil)
            )
        )
    ));
// 创建 b 和 c 时调用的 Rc::clone 函数会接收 a 中 Rc<List> 的引用作为参数
// 让 b 和 c 都链接到 链接列表 a 上面
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    // Rc::clone 方法，接收一个 Rc 指针的引用，对这个这个指针进行复制，最后返回一个相同的 Rc 指针，这个指针也是指向作为参数的 Rc 指针指向的数据

    // 可以在这里调用 a.clone() 而不是 Rc::clone(&a) 来实现同样的效果
    // 但 Rust 的惯例是在此场景下使用 Rc::clone
    // 因为 Rc::clone 不会执行数据的深度拷贝操作，只复制 Rc 的指针，这与绝大多数类型实现的 clone 方法明显不同
    // 调用 Rc::clone 只会增加引用计数，而这不会花费太多时间
    // 但与此相对的是，深度拷贝则常常需要花费大量时间来搬运数据
    // 因此，在引用计数上调用 Rc::clone 可以让开发者一眼就区分开“深度拷贝”与“增加引用计数”这两种完全不同的克隆行为

    // 克隆Rc<T>会增加引用计数

    let d = Rc::new(Cons(
        5,
        Rc::new(
            Cons(
                10,
                Rc::new(Nil)
            )
        )
    ));


    // count after creating d = 1
    println!("count after creating d = {}", Rc::strong_count(&d));

    let e = Cons(3, Rc::clone(&d));
    // count after creating e = 2
    println!("count after creating e = {}", Rc::strong_count(&d));
    
    {
        let f = Cons(4, Rc::clone(&d));
        // count after creating f = 3
        println!("count after creating f = {}", Rc::strong_count(&d));
    }
    // count after c goes out of scope = 2
    println!("count after c goes out of scope = {}", Rc::strong_count(&d));
    
    // 我们在每一个引用计数发生变化的地方调用Rc::strong_count函数来读取引用计数并将它打印出来 。 
    // 这个函数之所以被命名为 strong_count（强引用计数）而不是count（计数），是因为 Rc<T> 类型还拥有一个 weak_count（弱引用计数）函数

    // 我们能够看到 d 存储的 Rc<List> 拥有初始引用计数 1，并在随后每次调用 clone 函数时增加1
    // 而当 f 离开作用域被丢弃时，引用计数减少 1。我们不需要像调用 Rc::clone 来增加引用计数一样手动调用某个函数来减少引用计数
    // Rc<T> 的 Drop 实现会在 Rc<T> 离开作用域时自动将引用计数减1
    // 
    // 我们没有在这段输出中观察到 e 和 d 在 main 函数末尾离开作用域时 的情形
    // 但它们会让计数器的值减少到 0 并使 Rc<List> 被彻底地清理掉
    // 使用 Rc<T> 可以使单个值拥有多个所有者，而引用计数机制则保证了这个值会在其拥有的所有者存活时一直有效，并在所有者全部离开作用域时被自动清理

}

enum List {
//    Cons(i32, Box<List>),
    Cons(i32, Rc<List>),
   Nil
}