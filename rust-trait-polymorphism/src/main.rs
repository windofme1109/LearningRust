
// Rust 中不提供继承
// 因为使用继承意味着你会在无意间共享出比所需内容更多的代码
// 子类并不应该总是共享父类的所有特性，但使用继承机制却会始终产生这样的结果，进而使程序设计缺乏灵活性
// 子类在继承的过程中有可能会引入一些毫无意义甚至根本就不适用于子类的方法
// 另外，某些语言强制要求子类只能继承自单个父类
// 这进一步限制了程序设计的灵活性，考虑到这些弊端，Rust选择了trait对象来代替继承


// Rust 使用 trait 实现继承

use rust_trait_polymorphism::{Draw, Screen, Button};

// 定义一个 SelectBox 类型
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

// SelectBox 也实现了 Draw 这个 trait
impl Draw for SelectBox {
    fn draw(&self) {
        println!("draw SelectBox");
    }
}

fn main() {
    // println!("Hello, world!");

    // 创建 Screen 这个实例

    let screen = Screen {
        components: vec![
            Box::new(
                Button {
                    width: 50,
                    height: 10,
                    label: String::from("OK")
                }
            ),
            Box::new(
                SelectBox {
                    width: 75,
                    height: 10,
                    options: vec![
                        String::from("Yes"),
                        String::from("Maybe"),
                        String::from("No")
                    ]
                }
            )
        ]
    };


    // run 方法中的代码只关心值对行为的响应，而不在意值的具体类型
    // 这一概念与动态类型语言中的“鸭子类型”（duck typing）十分
    // 相似：如果某个东西走起来像鸭子，叫起来也像鸭子，那么它就是一只鸭子！
    // 在实现 run 方法的过程中并不需要知晓每个组件的具体类型，它仅仅调用了组件的draw方法，而不会去检查某个组件究竟是 Button 实例还是 SelectBox 实例
    // 通过在定义动态数组 components 时指定 Box<dyn Draw> 元素类型，Screen实例只会接收那些能够调用draw方法的值

    // 使用trait对象与类型系统来实现“鸭子类型”有一个明显的优势：我们永远不需要在运行时检查某个值是否实现了指定的方法
    // 或者担心出现“调用未定义方法”等运行时错误。Rust 根本就不会允许这样的代码通过编译
    screen.run();



    // 给 Screen 中的 components 传入一个 String，因为 String 没有实现 Draw 这个 trait，所以下面的代码无法通过编译
    // 报错信息如下：
    // error[E0277]: the trait bound `String: Draw` is not satisfied
    //     --> src\main.rs:72:13
    //     |
    //     72 |             Box::new(String::from("Hi"))
    //     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Draw` is not implemented for `String`
    // |

    // let screen2 = Screen {
    //     components: vec![
    //         Box::new(String::from("Hi"))
    //     ]
    // };

    // 从上面的例子中，可以看出，只有实现了 Draw 这个 trait 的类型，才能作为 components 这个数组的元素
}
