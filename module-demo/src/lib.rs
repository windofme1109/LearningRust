

// 将不同的 module 放在 src/ 的不同文件中，此时对应的文件名即是 module 名，并在 src/lib.rs 做相应的声明
// 声明其他文件定义的模块
pub mod bar;
pub mod foo;

fn use_foo_and_bar() {
    crate::foo::hell_foo();
    crate::bar::hello_bar();
}

pub mod foo2;

fn use_foo2() {
    crate::foo2::foo_mod1::hello_foo_mod1();
    crate::foo2::foo_mod2::hello_foo_mod2();
}
// pub mod foo 声明的 foo module 中其实是包含了 foo_mod1 和 foo_mod2 两个 module
// 此时对应的文件目录为
// src
// ├── foo
// │   ├── foo_mod1.rs
// │   ├── foo_mod2.rs
// │   └── mod.rs
// └── lib.rs
// 
// 除了在目录内定义 mod.rs 这种方式，还可以声明与模块目录名称相同且与模块目录同级的 rs 文件来定义模块
// 新建一个目录 bar2，并在 bar2 目录下声明两个模块文件：
// bar_mod1.rs
// bar_mod2.rs
// 然后再与 bar2 目录同级的目录 src 下，定义 bar2.rs 文件，并声明 bar2 目录下的两个模块：
// 
// pub mod bar_mod1;
// pub mod bar_mod2;

// 在 src/lib.rs 中，声明 bar2 模块
// pub mod bar2;
// 此时对应的文件目录为
// src
// ├── bar2
// │   ├── bar_mod1.rs
// │   ├── bar_mod2.rs
// │—— bar2.rs
// └── lib.rs

pub mod bar2;
pub mod my_print;
pub mod my_user;



fn use_bar2() {
    crate::bar2::bar_mod1::hello_bar_mod1();
    crate::bar2::bar_mod2::hello_bar_mod2();
}






// Rust 将会默认将对应文件名作为 module 名，而无需显式使用 mod {} 语句
// 此时的文件组织形式为：
// src
// ├── bar.rs
// ├── foo.rs
// └── lib.rs

// 这种定义模块的方式需要注意两点：
// 1. 模块文件必须直接在 src 目录下
// 2. 在 lib.rs 中声明的模块名称必须与文件名一样

pub mod bar_mod {
    pub fn bar() {
        println!("bar_mod::bar()");
    }

    pub fn call_bar() {

        // 在 lib.rs 中定义的模块 bar_mod，使用绝对路径引用其中的方法：bar
        // 此时 crate 表示的根路径就是 lib.rs
        crate::bar_mod::bar();


        // 虽然我们同时有 src/main.rs 和 src/lib.rs
        // 但是这两个文件都各自有其同名根节点 crate，所以 call_bar() 在 src/lib.rs 中的用法没有问题
    }

    // 模块嵌套
    pub mod deeply {
        pub mod nested {
            // 在 lib.rs 中，引用 hello() 的绝对路径为：crate::bar_mod::deeply::nested::hello()
            // 将 crate 根节点类比为 / ，则这这个绝对路径就可以为 /bar_mod/deeply/nested/hello()，这和文件目录树是非常类似的
            pub fn hello() {}
        }


        pub fn call_hello() {
            // 绝对路径引用
            crate::bar_mod::deeply::nested::hello();
            
            // 相对路径引用
            nested::hello();
        }

        // 类比于文件目录树中的 . 和 .. 这两个特殊目录
        // Rust 提供了 self 和 super 这两个特殊的关键字来分别表示当前 module 和上一层 module
        pub fn use_self_and_super() {
            // self 表示 deeply 模块，相当于 ./
            // 因为 use_self_and_super 定义在 deeply 这个模块中，self 表示当前模块
            // 可以通过 self 调用 deeply 模块中的其他方法
            self::call_hello();

            // super 表示 bar_mod 模块，相当于 ../
            // 因为 use_self_and_super 定义在 deeply 这个模块中，super 表示上一级模块
            // 所以 super 表示 bar_mod
            super::call_bar();
        }
    }
}

// 在 lib.rs 中，module tree 为：
// crate (src/lib.rs)
// ├── bar_mod
//     ├── call_bar()


// 当 package 或者 crate 作为 lib 发布时，其根入口总是 src/lib.rs
// 我们将会在 lib.rs 中定义或者声明其他 module。所以，当我们使用某一个 lib 的时候
// 可以优先先阅读 src/lib.rs，也就是说，一个 lib 对外暴露的 item 总是在 src/lib.rs 中

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
