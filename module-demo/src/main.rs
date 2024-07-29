// 使用 mod 关键字定义一个模块
mod foo_mod {
    pub fn foo() {
        println!("foo_mod::foo()");
    }

    
}

mod main_bar_mod {
    pub fn bar() {
        println!("bar_mod::bar()");
    }
}

// 在 main.rs 中引用在 main.rs 之外声明的其他模块，如 bar2
// 因为 crate 表示main.rs 这个根节点
// 所以，我们只能使用 crate 名（module_demo）作为根节点
use module_demo::bar2::bar_mod1;

// 还可以在 main.rs 中声明外部模块
pub mod foo2;
pub mod foo;
pub mod bar;

pub mod my_print;



use foo::hell_foo;

fn main() {

    bar_mod1::hello_bar_mod1();

    bar::hello_bar();

    hell_foo();

    // 以 crate 开头的路径表示绝对路径
    crate::foo_mod::foo();
    crate::main_bar_mod::bar();


    // 如果想使用 crate 开头的绝对路径方式调用 lib.rs 中模块，会编译不通过
    // failed to resolve: unresolved import
    // crate::bar_mod::call_bar();
    
    // 报错的原因是：
    // crate 所表示的根节点指的是 main.rs，那么在 main.rs 中是没有 bar_mod 这个模块的
    // 此时，我们想引用 lib.rs 中的模块，只能使用 crate 名，即项目名称：module_demo（如果原来 package 命名中带有 -，将会默认转换为下划线 _）
    // 使用 crate 名 module_demo 作为当前的根节点
    module_demo::bar_mod::call_bar();
    // crate 名其实也是 package 名。由于一个 package 只能有一个 lib crate
    // 所以此时 package 名指向的就是其 lib（即 lib.rs），不会产生二义性
    // 除此之外，当我们在其他 crate 中使用这个 lib crate 时候，也必须使用 crate 名（因为 crate:: 根仅对当前 crate 有效）
    // 也就是说 crate 名是 lib crate 对外的根节点
}

// Rust 采用了一种类似于文件系统目录树（module tree）的形式来管理 module 的引用路径
// module 之间通过使用其路径来使用 module

// 绝对路径的起点可以有两种：

// 1. 名为 crate 的根节点
// src/main.rs 和 src/lib.rs 可分别形成 crate 根。
// 比如在当前的 main.rs 中，module tree 即为：
//
//
// crate (src/main.rs)
// ├── foo_mod
//     ├── foo()
// 
//