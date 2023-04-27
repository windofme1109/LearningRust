// 循环引用会造成代码泄漏
fn main() {

    // 构造一个循环列表
    // 创建出一个 Rc<List> 实例并存储至变量 a，其中的 List 被赋予了初始值5, Nil
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    // 初始情况下，a 表示的 Rc 指针被引用的次数是 1
    println!("a initial rc count = {}", Rc::strong_count(&a));
    // a 最后一个元素是 Some 变体
    println!("a next item = {:?}", a.tail());

    // 创建出一个 Rc<List> 实例并存储至变量 b，其中的 List 包含数值 10 及指向列表 a 的指针
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    
    // 列表 b 引用了 a，所以，a 表示的 Rc 指针次数加 1，现在是 2
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    // 列表 b 表示的 Rc 指针次数为 1
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    // 将 a 指向的下一个元素Nil修改为 b 来创建出循环
    // 调用 tail 方法来得到 a 的 RefCell<Rc<List>> 值的引用并将它暂存在link变量中
    // 接着，我们使用 RefCell<Rc<List>> 的 borrow_mut 方法来将 Rc<List> 中存储的值由 Nil 修改为 b 中存储的 Rc<List>
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    // 因为 a 中原来指向的 Nil 现在指向了 b 中存储的 Rc<List>，所以 b 表示的 Rc 指针次数加 1，现在是 2
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // 打印的结果是： 
    // a initial rc count = 1
    // a next item = Some(RefCell { value: Nil })
    // a rc count after b creation = 2
    // b initial rc count = 1
    // b next item = Some(RefCell { value: Cons(5, RefCell { value: Nil }) })
    // b rc count after changing a = 2
    // a rc count after changing a = 2

    // 
    // println!("a next item = {:?}", a.tail());
}

use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    // Cons 变体的第二项元素变为了 RefCell<Rc<List>>，意味着我们现在可以灵活修改 Cons 变体指向的下一个 List 值
    Cons(i32, RefCell<Rc<List>>),
    Nil
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None
        }
    }
}