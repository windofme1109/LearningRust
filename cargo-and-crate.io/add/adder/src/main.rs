use add_one;
use rand;

fn main() {
    let num = 5;
    println!("Hello, world! {} plus one is {}!", num, add_one::add_one(num));
    println!("{} + {} = {} ", 5, 10, add_one::add(5, 10));

}
