// 将模块拆分为不同的文件
// 之前的的模块都写在一个文件中
// 当模块规模逐渐增大时，我们可以将它们的定义移动至新的文件，从而使代码更加易于浏览

// mod front_of_house 后使用分号而不是代码块会让Rust前往与当前模块同名的文件中加载模块内容
// 注意，如果在 lib.rs 或者是 main.rs 中定义模块（如下所示），那么其位置是与定义模块的的文件同级的位置
mod front_of_house;
mod back_of_house;

use crate::front_of_house::hosting;
use crate::back_of_house::cooking;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

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
