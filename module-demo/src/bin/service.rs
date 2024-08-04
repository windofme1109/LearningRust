
// 在 bin 目录下的 rs 文件中引用模块，有两种方式：
// 1. 绝对路径引用，根节点是 crate 名
// 2. 使用 path 属性，手动指定模块文件路径

use module_demo::foo2::foo_mod1;
use module_demo::foo;

use module_demo::foo;

#[path ="../my_content/mod.rs"]
mod my_content;

// use my_user::custom_user;

fn main() {
    foo_mod1::hello_foo_mod1();
    foo::hell_foo();
    my_content::custom_content::my_content();
    println!("hello service");
}