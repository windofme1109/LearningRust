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
    // 因为 a 中原来指向的 Nil 现在指向了 b 中存储的 Rc<List>，所以 b 表示的 Rc 指针次数加 1，现 在是 2
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

    // 在 main 函数结尾处，Rust 会首先释放 b，并使 b 存储的 Rc<List> 实例的引用计数减少 1
    // 但由于 a 仍然持有一个指向 b 中 Rc<List> 的引用， 所以这个 Rc<List> 的引用计数仍然是 1 而不是 0
    // 因此，该 Rc<List> 在堆上的内存不会得到释放,这块内存会永远以引用计数为1的状态保留在堆上
    //
    // 尝试打印循环引用：
    // 在尝试将这个循环引用打印出来的过程中反复地从 a 跳转至 b，再从 b 跳转至 a；
    // 整个程序会一直处于这样的循环中直到发生栈溢出为止
    // println!("a next item = {:?}", a。tail());

    // 尝试顺着指针往下寻找，打印出链路上的每一个节点

    // 在 Rust 中创建出循环引用并不是特别容易，但也绝非不可能：
    // 如果你的程序中存在 RefCell<T> 包含 Rc<T> 或其他联用了内部可变性与引用计数指针的情形
    // 那么你就需要自行确保不会在代码中创建出循环引用

    // 使用 Weak<T> 代替 Rc<T> 来避免循环引用
    // let leaf = Rc::new(Node {
    //     value: 3,
    //     children: RefCell::new(vec![]),
    // });
    // let branch = Rc::new(Node {
    //     value: 5,
    //     children: RefCell::new(vec![Rc::clone(&leaf)]),
    // });

    // 克隆了 leaf 的 Rc<Node> 实例，并将它存入 branch。这意味着 leaf 中的 Node 现在分别拥有了 leaf 与 branch 两个所有者
    // 我们可以使用 branch.children 来从 branch 访问 leaf

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    // 弱引用的 update 方法返回 弱引用存在时的值（Some），不存在返回 None
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // 首先通过 RefCell<Weak<Node>> 的 borrow_mut 方法取出 leaf 中的 parent 字段的可变借用
    // 调用 Rc::downgrade 函数来获取 branch 实例 Rc<Node> 的 Weak<Node> 引用
    // 并将这个引用赋值给 parent
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    // 此时打印 leaf 的 parent，便可以看到一个包含了 branch 实际内容的 Some 变体
    // 这意味着 leaf 现在可以访问父节点了
    // 此外现在打印 leaf 还可以避免因循环引用导致的栈溢出出故障
    // 因为 Weak<Node> 引用会被直接打印为 (Weak)
    // leaf parent = Some(Node { value: 5, parent: RefCell { value: (Weak) }, children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) }, children: RefCell { value:
    // [] } }] } })
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // 显示 strong_count 和 weak_count 计数值的变化

    let leaf2 = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    // 弱引用的 update 方法返回 弱引用存在时的值（Some），不存在返回 None
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf2),
        Rc::weak_count(&leaf2),
    );

    {
        let branch2 = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf2)]),
        });
        *leaf2.parent.borrow_mut() = Rc::downgrade(&branch2);

        // Rc::new 创建了一个强引用，因此 strong_count 次数为 1
        // Rc::downgrade 创建一个弱引用，因此 weak_count 为 1
        // branch2 strong = 1, weak2 = 1
        println!(
            "branch2 strong = {}, weak2 = {}",
            Rc::strong_count(&branch2),
            Rc::weak_count(&branch2),
        );

        // leaf2 被 Rc::clone 方法克隆，因此其强引用次数加 1，所以 strong_count 为 2
        // leaf2 没有弱引用，因此 weak_count 依旧为 0
        // leaf2 strong = 2, weak2 = 0
        println!(
            "leaf2 strong = {}, weak2 = {}",
            Rc::strong_count(&leaf2),
            Rc::weak_count(&leaf2),
        );
    }

    // 当内部作用域结束时，branch 会离开作用域并使 Rc<Node> 的强引用计数减为 0，从而导致该 Node 被丢弃
    // 虽然此时 branch 的弱引用计数因为 leaf.parent 的指向依然为 1，但这并不会影响到 Node 是否会被丢弃，这段代码没有产生任何内存泄漏

    // 因此打印 leaf2 的父节点，是 None
    println!("leaf2 parent = {:?}", leaf2.parent.borrow().upgrade());
    // 因为 branch2 节点被丢弃了，所以，leaf2 的强引用次数减 1，弱引用次数依旧为 0
    println!(
        "leaf2 strong = {}, weak2 = {}",
        Rc::strong_count(&leaf2),
        Rc::weak_count(&leaf2),
    );

    // 所有这些用于管理引用计数及值释放的逻辑都被封装到了 Rc<T> 与 Weak<T> 类型，以及它们对 Drop trait 的具体实现中
    // 通过在 Node 定义中将子节点指向父节点的关系定义为一个 Weak<T> 引用
    // 可以使父子节点在指向彼此的同时避免产生循环引用或内存泄漏
}

use std::rc::{Rc, Weak};
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

// 使用 Weak<T> 代替 Rc<T> 来避免循环引用

// 目前，我们已经知道了如何通过调用 Rc::clone 来增加 Rc<T> 实例的strong_count引用计数
// 并指出 Rc<T> 实例只有在 strong_count 为 0 时才会被清理
//
// 除此之外，我们还可以通过调用 Rc::downgrade 函数来创建出 Rc<T> 实例中值的弱引用
// 使用 Rc<T> 的引用来调用 Rc::downgrade 函数会返回一个类型为 Weak<T> 的智能指针
// 这一操作会让 Rc<T> 中 weak_count 的计数增加 1，而不会改变 strong_count的状态
// Rc<T> 类型使用 weak_count 来记录当前存在多少个 Weak<T> 引用，这与 strong_count 有些类似
// 它们之间的差别在于，Rc<T> 并不会在执行清理操作前要求 weak_count 必须减为0
// 强引用可以被我们用来共享一个 Rc<T> 实例的所有权，而弱引用则不会表达所有权关系
// 一旦强引用计数减为 0，任何由弱引用组成的循环就会被打破。因此，弱引用不会造成循环引用

// 由于我们无法确定 Weak<T> 引用的值是否已经被释放了，所以我们需要在使用 Weak<T> 指向的值之前确保它依然存在
// 我们可以调用 Weak<T> 实例的 upgrade 方法来完成这一验证
// 此函数返回的 Option<Rc<T>> 会在 Rc<T> 值依然存在时表达为 Some，而在 Rc<T> 值被释放时表达为 None
// 由于 upgrade 返回的是 Option<T> 类型，所以Rust能够保证 Some 和 None  两个分支都得到妥善的处理，而不会产生无效指针之类的问题

// 创建一棵树，它的每个节点都能够指向自己的父节点与全部的子节点

// 创建树状数据结构体：带有子节点的 Node
// 首先，我们会创建出一个能够指向子节点的Node结构体，它可以存储一个 i32 值及指向所有子节点的引用
#[derive(Debug)]
struct Node {
    // 希望Node持有自身所有的子节点并通过变量来共享它们的所用权，从而使我们可以直接访问树中的每个 Node
    // 因此，我们将 Vec<T> 的元素定义为 Rc<Node> 类型的值
    // 由于我们还希望能够灵活修改节点的父子关系，所以我们在 children 字段中使用 RefCell<T> 包裹 Vec<Rc<Node>> 来实现内部可变性
    value: i32,
    // Rc<T> 这种类型肯定不是一个好的选择，因为它会创建出循环引用：两个节点，一个节点为另外一个的子节点，另外一个则是其父节点，显然这样会造成循环引用
    // 仔细思考一下：父节点自然应该拥有子节点的所有权，因为当父节点被丢弃时，子节点也应当随之被丢弃
    // 但子节点却不应该拥有父节点，父节点的存在性不会因为丢弃子节点而受到影响。这正是应当使用弱引用的场景
    // 父与子之间的联系使用强引用，子与父之间的联系使用弱引用
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}