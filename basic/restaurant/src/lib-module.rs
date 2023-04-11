
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
    pub mod serving {

        // 模块的子节点（child）
        fn take_order() {}
        pub fn serve_order() {}
        pub fn take_payment() {}

        pub mod back_of_house {
            fn fix_incorrect_order() {
                // back_of_house 模块是 serving 的子模块
                // 因此 fix_incorrect_order 函数 可以直接使用其同级的条目
                cook_order();
                // 如果想要使用其父级模块 serving 的相关条目，就不能直接使用了
                // 需要使用 super 关键字表示对父级模块的引用
                // 使用 super 关键字来跳转至 back_of_house 的父模块
                super::serve_order();
            }

            pub fn cook_order() {}
            

            // 定义一个结构体
            // 使用 pub 关键字声明其为公开的，这样外界可以访问到这个结构体
            // 但是仅仅是能访问到这个结构体，结构体内部的字段还是私有的，外界还是访问不到
            // 因此，如果希望能访问到结构体的某个字段，还需要使用 pub 修饰这个字段
            pub struct Breakfast {
                pub toast: String,
                seasonal_fruit: String 
            }
            // 需要注意的是，因为 back_of_house::Breakfast 拥有了一个私有字段
            // 所以这个结构体需要提供一个公共的关联函数来构造 Breakfast 的实例(
            // 也就是下面的 summer 函数，如果缺少了这样的函数，我们将无法在使用 Breakfast 的地方创建任何的 Breakfast 实例 
            //因为我们不能模块外面设置私有字段 seasonal_fruit 的值
            impl Breakfast {
                pub fn summer(toast: &str) -> Breakfast {
                    Breakfast { 
                        toast: String::from(toast), 
                        seasonal_fruit:  String::from("peaches")
                    }
                }
            }

            // 定义一个枚举
            // 使用 pub 关键字声明其为公开的，这样外界可以访问到这个枚举
            pub enum Appetizer {
                // 使用 pub 修饰枚举，那么枚举的所有字段都是公开的，无需 pub 进行修饰
                Soup,
                Salad,
            }
        }
    }


}


pub fn eat_at_restaurant() {

    
    let mut meal = front_of_house::serving::back_of_house::Breakfast::summer("Rye");

    println!("the summer breakfast is {}", meal.toast);
    meal.toast = String::from("wheat");
    println!("the summer breakfast is {}", meal.toast);

    // 使用模块中的枚举
    // 因为Appetizer枚举具有公共属性，所以我们能够在 eat_at_restaurant 中使用 Soup 与 Salad 变体
    // 枚举与结构体之所以不同，是由于枚举只有在所有变体都公共可用时才能实现最大的功效
    // 而必须为所有枚举变体添加 pub 则显得烦琐了一些，因此所有的枚举变体默认都是公共的
    // 对于结构体而言，即便部分字段是私有的也不会影响到它自身的使用
    // 所以结构体字段遵循了默认的私有性规则，除非被标记为pub，否则默认是私有的
    let order1 = front_of_house::serving::back_of_house::Appetizer::Soup;
    let order2 = front_of_house::serving::back_of_house::Appetizer::Salad;

    // 修改私有字段 seasonal_fruit 无法通过编译，因此，私有字段不对外暴露，我们也不能尝试修改
    // error[E0616]: field `seasonal_fruit` of struct `Breakfast` is private
    // meal.seasonal_fruit = String::from("blueberries");


    // 绝对路径调用某个模块中的 item
    // 使用单元包名或字面量crate从根节点开始的绝对路径，类似于操作系统中的通过路径定位某个文件
    // 标识符之间使用双冒号（::）分隔
    // 下面的这种绝对路径，可以类比一个拥有相同结构的文件系统
    // 这个过程类似于指定路径 /front_to_house/hosting/add_to_waitlist 来运行 add_to_waitlist 程序
    // 使用 crate 从根节点开始类似于在shell中使用 / 从文件系统根开始
    // crate::front_of_house::hosting::add_to_waitlist();

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