// use 关键字的使用
// 无论是使用相对路径还是绝对路径引入模块
// 存在一个问题，就是我们使用模块中的任意一个 item，都需要写一个路径：
// front_of_house::serving::back_of_house::cook_order()
// front_of_house::serving::back_of_house::::Breakfast::summer("Rye")
// 需要指定路径上的每一个节点，比较麻烦
// 幸运的是，Rust 提供了 use 关键字来解决这个问题
// use 关键字来将路径引入作用域，并像使用本地条目一样来调用路径中的条目
// 换句话说，就是将某个模块引入到当前作用域，然后我们在当前作用域直接使用这个模块内的条目，而无需写路径

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// 使用 use 将模块引入作用域，此时的路径是绝对路径
// use crate::front_of_house::hosting;

// 使用相对路径
// 需要注意的是：我们必须在传递给 use 的路径的开始处使用关键字 self
// 而不是从当前作用域中可用的名称开始，这点和我们使用相对路径引入 item 是不一样的
use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // 因为已经在当前作用域引入了这个模块，所以这里直接使用模块中的 item，而无需写路径
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

}

// 两个使用 use 引入的惯例
// 1. 对于函数，我们更倾向于引入函数的父模块，而不是直接引入具体的函数，这样我们在调用函数的时候，必须指定其父模块
//    从而更清晰地表明当前函数没有被定义在当前作用域中
// use crate::front_of_house::hosting;
// fn test() {
//     hosting::add_to_waitlist();
// }

// 2. 对于结构体、枚举和其他条目引入作用域时， 我们习惯于通过指定完整路径的方式引入，即将具体的条目引入当前作用域中
// 引入 HashMap
// use std::collections::HashMap;
// fn main() {
//     let mut map = HashMap::new();
//     map.insert(1, 2);
// }

// 如果我们需要引入两个拥有相同名称的 item，那么应该是引入它们的父模块，以示区分
// 不能直接引入具体的 item，因为 Rust 并不支持同时引入两个同名的 item
// use std::fmt;
// use std::io;

// 通过不同的父模块来区别同名的 Item
// fn function1() -> fmt::Result {}
// fn function2() -> io::Result<String> {}


// 使用 as 为引入的 item 取别名
// 使用 as 为引入的 item 取别名，这样可以解决引入的 item 同名的问题，给每个 item 取个别名

use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;

// 通过不同的父模块来区别同名的 Item
// fn function1() -> FmtResult {}
// fn function2() -> IoResult {}


// 使用 pub use 进行重新导出
// 当我们使用use关键字将名称引入作用域时，
// 这个名称会以私有的方式在新的作用域中生效
// 为了让外部代码能够访问到这些名称，我们可以通过组合使用 pub 与 use 实现
// 这项技术也被称作重导出(re-exporting)，因为我们不仅将条目引入了作用域
// 而且使该条目可以被外部代码从新的作用域引入自己的作用域
// pub use self::front_of_house::hosting;
// pub use 类似于 javascript 中的 export from，即实现了一个中转导出

// 使用嵌套的路径来清理众多的 use 语句
// 当我们想要使用同一个包或同一个模块内的多个条目时，将它们 逐行列出会占据较多的纵向空间，如下所示：
use std::cmp::Ordering;
use std::io;

// 可以在同一行内使用嵌套路径来将上述条目引入作用域：
// 1. 指定路径的相同部分，再在后面跟上两个冒号，
// 2. 用一对花括号包裹路径差异部分的列表
// 示例如下：
use std::{cmp::Ordering, io};

// 还有一种形式，就是我们想引入父模块的同时，还想引入子模块或者 item，如下所示：
use std::io;
use std::io::Write;

// 此时我想将其两个 use 语句合并为一个，可以这样写
use std::io::{self, write};

// self 表示对 std::io 的引用，即对公共部分的引用

// 假如你想要将所有定义在某个路径中的公共 item 都导入作用域
// 那么可以在指定路径时在后面使用 * 通配符
use std::collections::*;

// 上面这行use语句会将定义在 std::collections 内的所有公共 item 都导入当前作用域
// 注意，请谨慎使用这个特性，因为通配符会让我们难以确定当前作用域中存在哪些名称，以及某个名称的具体定义位置