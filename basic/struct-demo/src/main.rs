#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32
}

 // 结构体添加方法
// 使用 impl 关键字给结构体添加方法
 impl Rectangle {
     // 结构体方法，第一个参数永远都是 self，用于指代调用该方法的结构体实例
     // 这里使用了参数定义使用了结构体实例的不可变引用
     fn area(&self) -> u32 {
         self.width * self.length
     }

     // 结构体的方法也可以接收除了 self 以外的参数
     // can_hold 方法可以检测当前的 Rectangle 实例是否能完整包含传入的另外一个Rectangle实例，
     // 如果是的话就返回true，否则返回false
     fn can_hold(&self, rect: &Rectangle) -> bool {
         self.width > rect.width && self.length > rect.length
     }
 }


// 定义关联函数
// 关联函数指的是不用接收 self 作为参数的函数
// 由于这类函数与结构体相互关联，所以被称为关联函数（associated function）
// 我们将其命名为函数而不是方法，是因为它们不会作用于某个具体的结构体实例
// 关联函数通常被用作构造器返回结构体的新实例
// 同时关联函数不需要通过实例调用，而是通过结构体调用，类似与其他语言的类的静态方法
// 形式：Rectangle::square(50)
// 在结构体名称后面加上两个冒号（::）调用关联函数
impl Rectangle {
    fn square(width: u32) -> Rectangle {
        let s = Rectangle {
            width,
            length: width
        };

        return s;
    }
}

fn main() {
    let rect_1 = Rectangle {
        width: 30,
        length: 50
    };

    let rect_2 = Rectangle {
        width: 10,
        length: 20
    };


    // 结构体方法的第一个参数是 self，表示是实例，但是我们不需要显式传入
    // 只需要传入 self 后面的参数即可，Rust 会帮助我们自动传入第一个参数
    println!("the area is {}", rect_1.area());
    println!(" hold {}", rect_1.can_hold(&rect_2));

    // 调用关联函数
    let square = Rectangle::square(100);

    println!("the square is {:?}", square);
}