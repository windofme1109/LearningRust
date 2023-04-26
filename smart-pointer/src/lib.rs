// 内部可变性的应用场景：模拟对象
// 测试替代（test double）是一个通用的编程概念，它代表了那些在测试工作中被用作其他类型替代品的类型
// 而模拟对象（mock object）则指代了测试替代中某些特定的类型，它们会承担起记录测试过程的工作。我们可以利用这些记录来断言测试工作的运行是否正确

// Rust没有和其他语言中类似的对象概念，也同样没有在标准库中提供模拟对象的测试功能
// 但是，我们可以自行定义一个结构体来实现与模拟对象相同的功能

// 设计的测试场景如下：我们希望开发一个记录并对比当前值与最大值的库，它会基于当前值与最大值之间的接近程度向外传递信息
//
// 例如，这个库可以记录用户调用不同 API 的次数，并将它们与设置的调用限额进行比较

// 我们只会在这个库中记录当前值与最大值的接近程度，以及决定何时显示何种信息
// 使用库的应用程序需要自行实现发送消息的功能，例如在应用程序中打印信息、发送电子邮件、发送文字短信等
// 我们会提供一个 Messenger trait 供外部代码来实现这些功能，而使库本身不需要关心这些细节

pub trait Messenger {
    // 一般来说，trait 中第一个参数 self 都是不可变引用
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T> where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }


    // 记录当前值与最大值的接近程度并根据不同的程度输出不同的警告信息
    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        // as 表示断言
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod Test {
    use std::cell::RefCell;
    use super::*;

    // MockMessenger 结构体拥 有一个 sent_messages 字段，它用携带 String 值的动态数组来记录所有接收到的信息
    struct MockMessenger {
        // sent_messages: Vec<String>,

        // 在RefCell<T> 中存入 sent_messages，send 方法就可以修改 sent_messages 来存储我们看到的信息
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        // 定义了关联函数 new 来方便地创建一个不包含任何消息的新 MockMessenger 实例

        fn new() -> MockMessenger {
            // MockMessenger { sent_messages: vec![] }
            // 在 new 函数中，我们使用了一个空的动态数组来创建新的 RefCell<Vec<String>> 实例
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    // 为 MockMessenger 实现了 Messenger trait，从而使它可以被用于创建 LimitTracker
    // 在 send 方法的定义中， 参数中的消息文本会被存入 sent_messages 的 MockMessenger 列表
    impl Messenger for MockMessenger {
        // fn send(&mut self, message: &str) {
        //     // 因为 &self 是不可变引用，所以这里我们不能改变 sent_messages 的值
        //     // 如果把 send 函数中的 &self 改为 &mut self，那么此时 send 函数的签名和 trait 中定义的 send 函数的签名不同：
        //     // error[E0053]: method `send` has an incompatible type for trait
        //
        //     // error[E0596]: cannot borrow `self.sent_messages` as mutable, as it is behind a `&` reference
        //     self.sent_messages.push(String::from(message));
        // }

        // 对于 send 方法的实现，其第一个参数依然是 self 的不可变借用，以便与 trait 的定义维持一致
        // 随后的代码调用了 RefCell<Vec<String>> 类型的 self.sent_messages 的borrow_mut 方法
        // 来获取 RefCell<Vec<String>> 内部值（也就是动态数组）的可变引用
        // 我们便可以在动态数组的可变引用上调用 push 方法来存入数据，从而将已发送消息记录下来
        fn send(& self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }

    }

    // 在测试函数中，我们希望检查 LimitTracker 在当前值 value 超过最大值max的75%时的行为
    // 函数体中的代码首先创建了一个信息列表为空的 MockMessenger 实例，并使用它的引用及最大值 100 作为参数来创建LimitTracker
    // 随后，我们调用了 LimitTracker 的 set_value 方法，并将值 80 传入该方法
    // 这个值超过了最大值 100 的 75%。最后，我们断言 MockMessenger 的信息列表中存在一条被记录下来的信息

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        // assert_eq!(mock_messenger.sent_messages.len(), 1);

        // 先调用 RefCell<Vec<String>> 的 borrow 方法来取得动态数组的不可变引用
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }


    struct MockMessenger2 {
        // sent_messages: Vec<String>,

        // 在RefCell<T> 中存入 sent_messages，send 方法就可以修改 sent_messages 来存储我们看到的信息
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger2 {
        // 定义了关联函数 new 来方便地创建一个不包含任何消息的新 MockMessenger 实例

        fn new() -> MockMessenger2 {
            // MockMessenger { sent_messages: vec![] }
            // 在 new 函数中，我们使用了一个空的动态数组来创建新的 RefCell<Vec<String>> 实例
            MockMessenger2 { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger2 {
        fn send(&self, message: &str) {
            // RefCell<T> 的 borrow_mut 方法可以获取其内部值的可变引用
            // 而对内部值的引用，同样要遵循基本的借用规则，所以如果我们使用 borrow_mut 方法两次
            // 相当于在同一个作用域内获取了内部值的两次可变引用，因此，虽然能通过编译，但是运行时会触发程序的 panic
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();
            one_borrow.push(String::from(message));
            two_borrow.push(String::from(message));
        }
    }



    // send 函数在实现过程中，违反了运行时的借用规则，因此触发了程序 panic
    // failures:
    //
    // ---- Test::it_sends_an_over_75_percent_warning_message_2 stdout ----
    // thread 'Test::it_sends_an_over_75_percent_warning_message_2' panicked at 'already borrowed: BorrowMutError', src\lib.rs:138:53
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    #[test]
    fn it_sends_an_over_75_percent_warning_message_2() {
        let mock_messenger2 = MockMessenger2::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger2, 100);
        limit_tracker.set_value(80);
        // assert_eq!(mock_messenger.sent_messages.len(), 1);

        // 先调用 RefCell<Vec<String>> 的 borrow 方法来取得动态数组的不可变引用
        assert_eq!(mock_messenger2.sent_messages.borrow().len(), 1);
    }
}

// RefCell<T> 的工作原理

// 我们会在创建不可变和可变引用时分别使用语法 & 与 &mut
// 对于 RefCell<T> 而言，我们需要使用 borrow 与 borrow_mut 方法来实现类似的功能
// borrow 与 borrow_mut 这两者都被作为 RefCell<T> 的安全接口来提供给用户
// borrow 方法和 borrow_mut 方法会分别返回 Ref<T> 与 RefMut<T> 这两种智能指针
// 由于这两种智能指针都实现了 Deref，所以我们可以把它们当作一般的引用来对待

// RefCell<T> 会记录当前存在多少个活跃的 Ref<T> 和 RefMut<T> 智能指针
// 每次调用 borrow 方法时，RefCell<T> 会将活跃的不可变借用计数加1，并且在任何一个 Ref<T> 的值离开作用域被释放时，不可变借用计数将减1
// RefCell<T>会基于这一技术来维护和编译器同样的借用检查规则：在任何一个给定的时间里，它只允许你拥有多个不可变借用或一个可变借用。
// 也就是说，RefCell<T> 根据活跃的 Ref<T> 和 RefMut<T> 智能指针的数量来判断同一时间内可变引用的数量和不可变引用的数量
//  Ref<T> 数量大于 1  => 可变引用的数量大于 1
//  RefMut<T> 数量等于 1  => 可变引用的数量等于 1

// Ref<T> 的数量大于 1，不违反借用规则
// RefMut<T> 的数量大于 1，违反借用规则

// 当我们违背借用规则时，相比于一般引用导致的编译时错误，RefCell<T>的实现会在运行时触发panic
use std::cell::RefCell;
