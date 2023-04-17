pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(x: i32) -> i32 {
    x + 2
}

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
} 
impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    // 使用 assert_eq! 和 assert_ne! 宏
    // assert_eq! 和assert_ne!。这两个宏分别用于比较并断言两个参数相等或不相等
    // 在断言失败时，它们还可以自动打印出两个参数的值，从而方便我们观察测试失败的原因
    // 相反，使用assert! 宏则只能得知==判断表达式失败的事实，而无法知晓被用于比较的值

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    // 构建一个会失败的测试用例，查看 assert_eq! 的输出结果
    // #[test]
    // fn it_adds_two_failed() {
    //     assert_eq!(5, add_two(2));
    // }
    // it_adds_two_failed 这个测试用例会失败，看一下输出结果：
    // failures:

    // ---- tests::it_adds_two_failed stdout ----
    // thread 'tests::it_adds_two_failed' panicked at 'assertion failed: `(left == right)`
    //   left: `5`,
    //  right: `4`', src/lib.rs:37:9
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    
    // 失 败 原 因 assertion failed: `(left ==
    // right)`，还将对应的参数值打印了出来：left为 5，right为 4。
    // 这样的日志可以帮助我们立即开始调试工作： 
    // 它意味着传递给 assert_eq! 宏的l eft 参数为 5，而 right 参数，也就是 add_two(2) 的值，是 4
    
    //  相对应地，assert_ne! 宏在两个值不相等时通过，相等时失败。、
    // 当我们无法预测程序的运行结果，却可以确定它绝不可能是某些值的时候，就可以使用这个宏来进行断言
    // 例如，假设被测试的函数保证自己会以某种方式修改输入的值，但这种修改方式是由运行代码时所处的日期来决定的，
    // 那么在这种情形下最好的测试方式就是断言函数的输出结果和输入的值不相等
    
    #[test]
    fn not_equal() {
        assert_ne!(2, add_two(2))
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        // can_hold 方法返回 布尔值，因此可以作为 assert! 这个宏的参数
        // can_hold 为 true，表示当前的长方形可以容纳作为参数的长方形
        // 如果通过测试，表示我们上面的结论是正确的
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        // 这个测试用例是用来测试 smaller 这个长方形不能装下larger 这个长方形的
        // 所以 can_hold 方法返回 false，而为了 assert! 这个宏，我们需要对结果取反：!
        assert!(!smaller.can_hold(&larger));
    }

    // 定义一个会失败的测试用例
    // #[test]
    // fn smaller_can_hold_larger() {
    //     let larger = Rectangle { length: 8, width: 7 };
    //     let smaller = Rectangle { length: 5, width: 1 };

    //     // smaller 肯定装不下 larger，所以 can_hold 返回 false
    //     // assert! 这个宏会触发 panic，从而使得测试失败
    //     assert!(smaller.can_hold(&larger));
    // }

    // 上面的失败的测试用例输出信息如下：
    // failures:
    // ---- tests::smaller_can_hold_larger stdout ----
    // thread 'tests::smaller_can_hold_larger' panicked at 'assertion failed: smaller.can_hold(&larger)', src/lib.rs:50:9
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace








    // Rust中的测试就是一个标注有test属性的函数
    // 属性 （attribute）是一种用于修饰Rust代码的元数据；我们在
    // 我们只需要将 #[test] 添加到关键字fn的上一行便可以将函数转变为测试函数
    // 当测试编写完成后，我们可以使用 cargo test 命令来运行测试
    #[test]
    fn exploration() {
        let result = add(2, 2);
        // assert_eq! 是一个断言的宏，用于判断 某个结果是不是与指定的值相等
        assert_eq!(result, 4);
    }
    

    // 执行 cargo test，输出如下：
// Compiling adder v0.1.0 (/Users/bytedance/myWork/myFrontEnd/LearningRust/basic/adder)
// Finished test [unoptimized + debuginfo] target(s) in 0.86s
// Running unittests src/lib.rs (target/debug/deps/adder-7606ee6a2938c22b)

// running 1 test
// test tests::it_works ... ok

// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// Doc-tests adder

// running 0 tests

// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// 对输出结果进行解读：
// Cargo成功编译并运行了这段测试。在运行结果的Compiling、Finished、Running这3行后面紧接着输出了running 1 test
// 表示当前正在执行 1 个测试

// 接下来显示的是所生成的测试函数名称 it_works，以及相对应的测试结果ok。
// 再下一行是该测试集的摘要：test result: ok. 表示该集合中的所有测试均成功通过
// 1 passed; 0 failed 则统计了通过和失败的测试总数
// 由于我们没有将任何测试标记为忽略，所以摘要中出现了信息 0 ignored
// 由于我们没有对运行的测试进行过滤，所以摘要的末尾处输出了 0 filtered out

// 另外一处信息 0 measured 则统计了用于测量性能的测试数量。
// 接下来以 Doc-tests adder 开头的部分是文档测试（documentation test）的结果
// 虽然我们还未编写过这种测试，但是要知道 Rust 能够编译在API文档中出现的任何代码示例。这一特性可
// 以帮助我们保证文档总会和实际代码同步


    // 在Rust中，一旦测试函数触发panic，该测试就被视作执行失败
    // 每个测试在运行时都处于独立的线程中，主线程在监视测试线程时，一旦发现测试线程意外终止，就会将对应的测试标记为失败
    // #[test]
    // fn another() {
    //     // 主动触发 panic，从而导致测试失败
    //     panic!("Make this test fail")
    // }

    // 再次执行 cargo test，输出结果如下：
    // Compiling adder v0.1.0 (/Users/bytedance/myWork/myFrontEnd/LearningRust/basic/adder)
    // Finished test [unoptimized + debuginfo] target(s) in 0.47s
    // Running unittests src/lib.rs (target/debug/deps/adder-7606ee6a2938c22b)

    // running 2 tests
    // test tests::exploration ... ok
    // test tests::another ... FAILED

    // failures:

    // ---- tests::another stdout ----
    // thread 'tests::another' panicked at 'Make this test fail', src/lib.rs:22:9
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // failures:
        //  tests::another

    // test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    
    // 与之前不同，结果中 test tests::another 字段后出现了 FAILED 而不是 ok。另外
    // 在测试结果与摘要之间新增了两段信息。第一段展示了每个测试失败的详细原因
    // 在本例中，测试 another '内部主动调用了 panic! 这个宏而导致失败
    // 第二段则列出了所有失败测试的名称，它可以帮助我们在输出的众多信息中定位到具体的失败测试
    // 我们可以通过指定名称来单独运行对应的测试，以便更容易地定位错误
    // 测试摘要依旧显示在输出结尾处：总的来说，我们的测试结果为FAILED：其中有 1 个测试成功，1 个测试失败

    // 有失败的测试用例，就会终止测试进程，并输出失败的测试用例结果

    // assert! 宏
    // assert! 宏由标准库提供，它可以确保测试中某些条件的值为true
    // assert! 宏可以接收一个能够被计算为布尔类型的值作为参数
    // 当这个值为 true 时，assert! 宏什么都不用做并正常通过测试
    // 而当值为false时，assert! 宏就会调用 panic! 宏，进而导致测试失败
    // 使用 assert! 宏可以检查代码是否按照我们预期的方式运行


}


