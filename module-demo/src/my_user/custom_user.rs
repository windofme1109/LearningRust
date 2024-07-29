

// 使用 my_user 模块之外（与 my_user 同级）的模块
use crate::my_print::custom_print::custom_print;

// use crate::service_log::log;
// 在某个模块内，使用 pub mod xxxx 声明某个外部模块，那么 rust 就会去在同级目录下，去寻找 xxxx.rs 或者是 xxxx/mod.rs
// pub mod my_log;

// 如上所示，声明了 my_log 这个模块，rust 并不会从根路径下开始查找，而是在 custom_user.rs 同一级别的目录下寻找：my_log.rs 或者是 my_log/mod.rs
// 希望的目录结构如下：
// src
// ├── my_user
//     ├── custom_user.rs
//     ├── mod.rs
//     ├── my_log.rs
// 或者是：
// src
// ├── my_user
//     ├── custom_user.rs
//     ├── mod.rs
//     ├── my_log
//         ├── mod.rs

// 如果 my_log 已经在 crate 其他地方声明了，有两种方式使用：
// 1. 需要使用  use crate::... 的方式将其引入，也就是绝对路径导入，此时的 crate 表示 lib（module_demo）节点
// 2. 使用外部属性 path 指定外部的模块路径：
//     #[path ="../service_log.rs"]
//     pub mod service_log;
#[path ="../service_log.rs"]
pub mod service_log;

#[path="../my_content/mod.rs"]
pub mod custom_content;

// 相对路径，其相对的起点是当前的文件所在的目录
#[path="private_user.rs"]
pub mod private_user;

// 使用 path 属性指定外部模块路径有以下几种方式：
// 1. 直接指定 rs 文件，即 rs 文件作为模块
// 2. 指定某个目录下的 mod.rs 文件

// path 一般使用相对路径，其相对的起点是当前的文件所在的目录
// 本例中，是 custom_user.rs 所在的目录

// 关于 path 属性的详解：https://doc.rust-lang.org/reference/items/modules.html#the-path-attribute

pub fn custom_user() {
    custom_print();
}

pub fn my_user() {
    service_log::log::my_log();
    custom_content::my_content();
    private_user::private_user()
}