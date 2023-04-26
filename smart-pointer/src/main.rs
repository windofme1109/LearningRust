// RcCell<T>

// 内部可变性 （interior mutability）是Rust的设计模式之一，它允许我们在只持有不可变引用的前提下对数据进行修改
// 通常而言，类似的行为会被借用规则所禁止。为了能够改变数据，内部可变性模式在它的数据结构中使用了unsafe（不安全）代码来绕过Rust正常的可变性和借用规则

// RcCell<T> 会跳过编译期的借用规则检查，而是在运行期检查借用规则，一旦出现违反借用规则的情况下，就会触发panic来提前中止程序

// 与 Rc<T> 相似，RefCell<T> 只能被用于单线程场景中。强行将它用于多线程环境中会产生编译时错误

// 下面是选择使用 Box<T>、Rc<T> 还是 RefCell<T> 的依据：
// 1. Rc<T> 允许一份数据有多个所有者，而 Box<T> 和 RefCell<T> 都只有一个所有者。
// 2. Box<T> 允许在编译时检查的可变或不可变借用，Rc<T> 仅允许编译时检查的不可变借用，RefCell<T> 允许运行时检查的可变或不可变借用
// 3. 由于 RefCell<T> 允许我们在运行时检查可变借用，所以即便 RefCell<T> 本身是不可变的，我们仍然能够更改其中存储的值


// Rc<T> 和 RefCell<T> 结合使用来实现一个拥有多重所有权的可变数据

// 将 RefCell<T> 和 Rc<T> 结合使用是一种很常见的用法。Rc<T> 允许多个所有者持有同一数据
// 但只能提供针对数据的不可变访问。如果我们在 Rc<T> 内存储了 RefCell<T>，那么就可以定义出拥有多个所有者且能够进行修改的值了

#[derive(Debug)]
enum List {
    // 之前定义 Cons 变体，存储的数据是 i32 类型
    // 使用 Rc<T> （Rc<List>）来让多个列表共享同一个列表的所有权
    // 由于 Rc<T> 只能存储不可变值，所以列表一经创建，其中的值就无法被再次修改了
    // 也就是说，可以通过 Rc<T> 指针找到列表数据，但是我们无法修改列表的数据
    //
    // 为了可以修改列表中的数据，我们使用 Rc<T> 与 RefCell<T> 包裹我们的数据
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil
}

use std::cell::RefCell;
use std::rc::Rc;

use crate::List::{Cons, Nil};



fn main() {
    // 内部可变性：可变地借用一个不可变的值
    // 借用规则的一个推论是，你无法可变地借用一个不可变的值
    // error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
    //     let x = 5;
    //     let y = &mut x;

    // 然而，在某些特定情况下，我们也会需要一个值在对外保持不可变性的同时能够在方法内部修改自身
    // 除了这个值本身的方法，其余的代码则依然不能修改这个值
    // 使用 RefCell<T> 就是获得这种内部可变性的一种方法
    // RefCell<T> 并没有完全绕开借用规则：我们虽然使用内部可变性通过了编译阶段的借用检查
    // 但借用检查的工作仅仅是被延后到了运行阶段。如果你违反了借用规则，那么就会得到一个 panic! 而不再只是编译时的错误。

    // 创建了一个 Rc<RefCell<i32>> 实例，并将它暂时存入了value变量中以便之后可以直接访问

    let value = Rc::new(RefCell::new(5));
    // 我们使用含有 value 的 Cons 变体创建一个List类型的 a 变量
    // 为了确保 a 和 value 同时持有内部值 5 的所有权，克隆了value，而不仅只是将 value 的所有权传递给 a，或者让 a 借用 value
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    // 为了 使得 b 和 c 同时能指向 a，这里也对 a 进行复制指针
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));


    // 通过调用 borrow_mut 函数来将 value 指向的值增加10
    // 注意，这里使用了自动解引用功能来将 Rc<T> 解引用为 RefCell<T>
    // borrow_mut 方法会返回一个 RefMut<T> 智能指针，我们可以使用解引用运算符来修改其内部值
    *value.borrow_mut() += 10;

    // 观察下面打印的值，value 中的数值变成了 15
    // a after = Cons(RefCell { value: 15 }, Nil)
    // b after = Cons(RefCell { value: 6 }, Cons(RefCell { value: 15 }, Nil))
    // c after = Cons(RefCell { value: 10 }, Cons(RefCell { value: 15 }, Nil))
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    // 通过使用 RefCell<T>，我们拥有的List保持了表面上的不可变状态
    // 并能够在必要时借由 RefCell<T> 提供的方法来修改其内部存储的数据。运行时的借用规则检查同样能够帮助我们避免数据竞争
    // 在某些场景下为了必要的灵活性而牺牲一些运行时性能也是值得的
}




