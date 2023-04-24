// rust 会自动的将库 crate 名称中的连字符（-）转换为下划线（_）
// 因此，即使我们将库 crate 命名为 add-three，引入是也必须是 add_three
use add_one;
// use add_two;
// use add_three;
// use rand;

fn main() {
    let num = 5;
    println!("Hello, world! {} plus one is {}!", num, add_one::add_one(num));
    // println!("Hello, world! {} plus two is {}!", num, add_two::add_two(num));
    // println!("Hello, world! {} plus three is {}!", num, add_three::addThree(num));
    // println!("{} + {} = {} ", 5, 10, add_one::add(5, 10));
    // println!("{} + {} = {} ", 5, 10, add_one::add(5, 10));

}
