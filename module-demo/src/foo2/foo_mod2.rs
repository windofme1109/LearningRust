pub fn hello_foo_mod2() {
    println!("foo2::foo_mod2::hello_foo_mod2()")
}

// 模块内的文件相互引用
// 现在想引用 foo_mod1.rs 下的内容
// 需要使用 super 关键字
// 因为二者的关系如下所示：
// src
// ├── foo2
//     ├── foo_mod1.rs
//     ├── foo_mod2.rs
//     ├── mod.rs

use super::foo_mod1;

