
// 使用 cargo new restaurant --lib 生成一个库 package
// 这个 package 是作为库存在的，因此其入口文件是 src/lib.rs，而不是 src/main.rs
// 初始的 lib.rs 的内容如下：
// 包含一个测试用例模块以及一个对外暴露的函数 add
// 因为没有 main.rs，所以 cargo run 命令没什么可执行的，所以我们使用 cargo build 命令进行构建
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


// 使用 mod 关键字定义一个模块
mod front_of_house {

    // 模块内还可以继续定义模块
    pub mod hosting {
        // 模块内可以包含其他条目的定义，比如 结构体、枚举、常量、trait 函数等
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }

    // 模块的父节点（parent）
    mod serving {

        // 模块的子节点（child）
        fn take_order() {}
        pub fn serve_order() {}
        pub fn take_payment() {}
    }


}


pub fn eat_at_restaurant() {
    // 绝对路径调用某个模块中的 item
    // 使用单元包名或字面量crate从根节点开始的绝对路径，类似于操作系统中的通过路径定位某个文件
    // 标识符之间使用双冒号（::）分隔
    // 下面的这种绝对路径，可以类比一个拥有相同结构的文件系统
    // 这个过程类似于指定路径 /front_to_house/hosting/add_to_waitlist 来运行 add_to_waitlist 程序
    // 使用 crate 从根节点开始类似于在shell中使用 / 从文件系统根开始
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径调用模块中的 item
    // 从 front_of_house 开始
    // 与 eat_at_restaurant 定义的模块树级别相同的那个模块名称开始
    // 类似于文件系统中 front_of_house/hosting/add_to_waitlist
    front_of_house::hosting::add_to_waitlist();

    // 以 crate 开头的是绝对路径
    // 以模块名称开头的表示路径是绝对的
}

// 默认情况下，模块和模块内的 item （函数、方法、结构体、枚举、模块及常量）都是私有的
// 即使我们拥有指向 hosting 模块及 add_to_waitlist 函数的正确路径，
// 但由于缺少访问私有域的权限，所以Rust依然不允许我们访问它们

// 父级模块中定义的 item 无法使用子模块中定义的私有 item
// 子模块中的私有 item 可以使用其所有祖先模块中 item

// 如果想要对外暴露某些模块或者是模块内的 item，我们需要使用 pub 关键字，表示其是公开的，是可以被外界访问的
// 对模块使用 pub 关键字，仅仅表示我们拥有访问这个模块的权利，但是模块内部的 item，还是私有的，我们依旧无法访问
// 所以，需要给可以被外界访问的 item 加上 pub 关键字，这样模块内部的 item 才可以被外界访问

// 虽然 front_of_house 模块并没有被公开，但是因为 eat_at_restaurant 函数
// 被定义在与 front_of_house 相同的模块中，也就是说 eat_at_restaurant 与 front_of_house 属于同级节点，
// 所以我们可以直接在 eat_at_restaurant 中引用 front_of_house
// 就是说，与 模块同级的函数，可以直接引用为公开的模块


// 通过使用模块，我们可以将相关的定义分到一组，并根据它们的关系指定有意义的名称。
// 开发者可以轻松地在此类代码中找到某个定义
// 因为他们可以根据分组来进行搜索而无须遍历所有定义
// 开发者可以把新功能的代码按这些模块进行划分并放入其中，从而保持程序的组织结构不变

// src/main.rs 与src/lib.rs 被称作单元包的根节点
// 因为这两个文件的内容各自组成了一个名为 crate 的模块
// 并位于单元包模块结构的根部。这个模块结构也被称为模块树（module tree）


// crate
// └── front_of_house
//     ├── hosting
//     │   ├── add_to_waitlist
//     │       └── seat_at_table
//     └── serving
//         ├── take_order
//         ├── serve_order
//         └── take_payment