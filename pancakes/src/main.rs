use std::fmt::{Display, Formatter, write};

// 需要实现的 trait
use hello_macro::HelloMacro;
// 自定义的 HelloMacro 宏
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;


fn main() {
    // println!("Hello, world!");
    Pancakes::hello_macro();

    let f = Fruit {
        apple: Apple {apple: String::from("红富士")}
    };
}


struct Apple {
    apple: String
}

impl Display for Apple {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.apple)
    }
}

struct Fruit {
    apple: Apple
}