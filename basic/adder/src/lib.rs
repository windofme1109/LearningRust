pub fn add(left: usize, right: usize) -> usize {
    left + right
}


pub fn greeting(name: &str) -> String {
    format!("Hello {} !", name)
}

pub fn greeting_2(name: &str) -> String {
    String::from("Hello")
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

struct Guess {
    value: i32
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value > 100 || value < 1 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    fn new_2(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
    fn new_3(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        }

        Guess {
            value
        }
    }
    fn new_4(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value);

        }

        Guess {
            value
        }
    }
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

// Rust 中涉及到的测试组织架构主要是单元测试（unit test）和集成测试（integration test）
// 单元测试小而专注，每次只单独测试一个模块或私有接口
// 而集成测试完全位于代码库之外，和正常从外部调用代码库一样使用外部代码， 只能访问公共接口，并且在一次测试中可能会联用多个模块

// 单元测试
// 单元测试的目的在于将一小段代码单独隔离出来，从而迅速地确定这段代码的功能是否符合预期
// 我们一般将单元测试与需要测试的代码存放在src 目录下的同一文件中
// 同时也约定俗成地在每个源代码文件中都新建一个 tests 模块来存放测试函数
// 并使用cfg(test)对该模块进行标注：
// #[cfg(test)]
// 模块名

// 在 tests 模块上标注 #[cfg(test)]可以让Rust只在执行cargo test 命令时编译和运行该部分测试代码
// 而在执行cargo build时剔除它们。这样就可以在正常编译时不包含测试代码，从而节省编译时间和产出物所占用的空间

// 我们不需要对集成测试标注#[cfg(test)]，因为集成测试本身就放置在独立的目录中
// 但是，由于单元测试是和业务代码并列放置在同一文件中的，所以我们必须使用 #[cfg(test)] 进行标注才能将单元测试的代码排除在编译产出物之外

// cfg 属性是配置（configuration）一词的英文缩写，它告知Rust接下来的条目只有在处于特定配置时才需要被包含进来。
// 下面的指定的 test 就是Rust中用来编译、运行测试的配置选项
// 通过使用cfg属性，Cargo 只在运行 cargo test 时才会将测试代码纳入编译范围
// 这一约定不止针对那些标注了 #[test] 属性的测试函数，还针对该模块内的其余辅助函数 



fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn add_two_2(a: i32) -> i32 {
    internal_adder(a, 2)
}


#[cfg(test)]
mod tests {
    use super::*;

    // 测试私有函数
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2))
    }

    // 注意：上面代码中的internal_adder函数没有被标注为pub，
    // 因为测试本身就是 Rust 代码，并且 tests 模块就是 Rust 模块
    // 所以你可以正常地将 internal_adder 导入测试作用域并调用它
    // 也就是说，在同一个父模块下，我们可以测试私有函数


    // 控制测试的运行方式
    // 我们可以通过指定命令行参数来改变 cargo test 的默认行为。例如，cargo test 生成的二进制文件默认会并行执行所有的测试，并截获测试运行过程中产生的输出来让与测试结果相关的内容更加易读


    // 我们既可以为 cargo test 指定命令行参数，也可以为生成的测试二进制文件指定参数
    // 为了区分两种不同类型的参数，你需要在传递给 cargo test 的参数后使用分隔符 --，并在其后指定需要传递给测试二进制文件的参数
    // 例如，运行 cargo test --help 会显示出cargo test 这个命令的可用参数
    // 而运行cargo test -- --help 则会显示出所有可以用在 -- 之后的参数
    // 例如限制线程的数量的命令；cargo test -- --test-threads=1
    // --test-threads 参数就是传给二进制文件的

    // 1. 并行或串行地进行测试
    // 当我们在运行多个测试时，Rust 会默认使用多线程来并行执行它们
    // 这样可以让测试更快地运行完毕，从而尽早得到代码是否能正常工作的反馈
    // 但由于测试是同时进行的，所以开发者必须保证测试之间不会互相依赖，或者依赖到同一个共享的状态或环境上，例如当前工作目录、环境变量等

    // 如果我们像串行的进行测试，或者希望精确地控制测试时所启动的线程数量
    // 那么可以通过给测试二进制文件传入 --test-threads 标记及期望的具体线程数量来控制这一行为

    // 命令为 cargo test -- --test-threads=1

    // 上面的命令是将将线程数量限制为 1，这也就意味着程序不会使用任何并行操作
    // 使用单线程执行测试会比并行执行花费更多的时间，但顺序执行的测试不会再因为共享状态而出现可能的干扰情形了


    // 显示函数输出
    // 默认情况下，Rust 的测试库会在测试通过时捕获所有被打印至标准输出中的消息
    // 例如，即便我们在测试中调用了 println!，但只要测试顺利通过，它所打印的内容就无法显示在终端上；我们只能看到一条用于指明测试通过的消息
    // 只有在测试失败时，我们才能在错误提示信息的上方观察到打印至标准输出中的内容
    //
    // 默认情况下，Rust 不会输出出通过测试的打印到标准输出的信息


    // #[test]
    // fn this_test_will_pass() {
    //     let result = prints_and_returns_10(4);
    //     assert_eq!(10, result);
    // }


    // #[test]
    // fn this_test_will_fail() {
    //     let result = prints_and_returns_10(8);
    //     assert_eq!(5, result);
    // }

    // 上面的两个测试用例，this_test_will_pass 测试会通过，而 this_test_will_fail 测试会失败
    // 失败信息如下：
    // failures:
    //
    // ---- tests::this_test_will_fail stdout ----
    // I got the value 8
    // thread 'tests::this_test_will_fail' panicked at 'assertion failed: `(left == right)`
    // left: `5`,
    // right: `10`', src\lib.rs:132:9
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    //
    //
    // failures:
    // tests::this_test_will_fail


    // 注意，我们无法在这段输出中找到信息 I got the value 4，即是 this_test_will_pass 的输出
    // 它虽然在通过的测试样例中被打印了出来，但却被Rust截获后丢弃了
    // 而 I got the value 8，即是 this_test_will_fail 的输出，则正常出现在了测试失败的摘要中
    // 和测试失败的原因一起显示出来了

    // 如果我们希望测试通过时也将值打印出来，那么可以传入 --nocapture 标记来禁用输出截获功能
    // 命令：cargo test -- --nocapture

    // 执行了 cargo test -- --nocapture
    // 输出如下：
    // running 2 tests
    // I got the value 8
    // I got the value 4
    // thread 'tests::this_test_will_failtest tests::this_test_will_pass ... ' panicked at 'okassertion failed: `(left == right)`
    //   left: `5`,
    //  right: `10`
    // ', src\lib.rs:132:9
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // test tests::this_test_will_fail ... FAILED
    //
    // failures:
    //
    // failures:
    //     tests::this_test_will_fail
    //
    // test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

    // 通过的用例中的打印信息也被输出了
    // 我们发现：测试的输出和测试结果相互交叉在了一起；这是因为测试是并行运行的。
    // 如果使用 --test-threads=1 将线程数量限制为一个，那么输出如下：
    // running 2 tests
    // test tests::this_test_will_fail ... I got the value 8
    // thread 'tests::this_test_will_fail' panicked at 'assertion failed: `(left == right)`
    //   left: `5`,
    //  right: `10`', src\lib.rs:132:9
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // FAILED
    // test tests::this_test_will_pass ... I got the value 4
    // ok
    //
    // failures:
    //
    // failures:
    //     tests::this_test_will_fail
    //
    // test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

    // 先输出的 this_test_will_fail 的打印值与测试结果，再输出的 this_test_will_pass 的打印值与测试结果
    // 两个结果并没有混合



    // 只运行部分特定名称的测试
    // 执行全部的测试用例有时会花费很长时间。而在编写某个特定部分的代码时
    // 你也许只需要运行和代码相对应的那部分测试。我们可以通过向 cargo test 中传递测试名称来指定需要运行的测试
    // #[test]
    // fn add_two_and_two() {
    //     assert_eq!(4, add_two(2));
    // }
    // #[test]
    // fn add_three_and_two() {
    //     assert_eq!(5, add_two(3));
    // }

    // #[test]
    // fn one_hundred() {
    //     assert_eq!(102, add_two(100));
    // }

    // 上面定义了三个测试用例
    // 执行 cargo test，会执行三个用例
    // 如果我们只想执行其中的一个，可以再 cargo test 后面添加 测试用例的名字：
    // cargo test one_hundred

    // 结果如下：running 1 test
    // test tests::one_hundred ... ok
    //
    // test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

    //  只执行了 one_hundred 这个用例，而 add_two_and_two 和 add_three_and_two 都没有执行
    // 测试输出还通过摘要一行中的 2 filtered out表明部分测试被过滤掉了

    // 注意：我们不能指定多个参数来运行多个测试；只有传递给 cargo test 的第一个参数才会生效

    // 我们可以通过过滤名称来运行多个测试
    // 实际上，我们可以指定测试名称的一部分来作为参数，任何匹配这一名称的测试都会得到执行
    // 例如，因为如果测试用例中都包含 add 这个字段，所以我们可以通过命令 cargo test add 来运行它们
    // 命令：cargo test add
    // 输出结果如下：
    // running 2 tests
    // test tests::add_three_and_two ... ok
    // test tests::add_two_and_two ... ok

    // test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

    // 如果想执行多个测试用例，但不是全部的测试用例，可以在测试用例的名称中添加共同的关键词
    // 这样就可以通过 执行 cargo test 指定关键词来执行多个测试用例
    // 注意，测试所在的模块的名称也是测试名称的一部分，所以我们可以通过模块名来运行特定模块内的所有测试


    // 通过显式指定来忽略某些测试
    // 有些测试执行起来比较耗时，所以想要在大部分的cargo test命令中忽略它们
    // 除了手动将想要运行的测试列举出来，你也可以使用 ignore 属性来标记这些耗时的测试
    // 将这些测试排除在正常的测试运行之外
    // 测试时，会忽略这些标记为 ignore 属性的测试用例


    #[test]
    fn it_works() {
        assert_eq!(2 + 2, add_two(2));
    }

    // 在我们想要忽略的测试，在 #[test] 标记的下方添加 #[ignore] 属性
    // 当我们运行测试时，就会忽略 expensive_test 这个测试用例
    #[test]
    #[ignore]
    fn expensive_test() {
        // 耗时很长
    }

    //     running 2 tests
    // test tests::expensive_test ... ignored
    // test tests::it_works ... ok

    // test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

    //    Doc-tests adder

    // running 0 tests

    // test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

    // 执行 cargo test，只执行了一个测试用例： it_works
    // expensive_test 这个用例被忽略了
    // 在测试结果汇总中，1 ignored 表示有一个测试用例被忽略

    // 如果想要执行标记为 ignore 的测试用例，可以使用下面这个命令：

    //  cargo test -- --ignored
    // 
    // 输出如下：running 1 test

    // test tests::expensive_test ... ok

    // test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

    //    Doc-tests adder

    // running 0 tests

    // test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

    // 只执行了 expensive_test 这个标记为 ignore 的测试用例






    // 使用 Result 枚举来进行测试

    // 使用 Result 枚举来进行测试，当测试运行失败时返回一个 Err 值而不是触发 panic
    // #[test]
    // fn it_works() -> Result<(), String> {
    //     if 2 + 2 == 4 {
    //         Ok(())
    //     } else {
    //         Err(String::from("two plus two does not equal four"))
    //     }
    // }
    // it_works 函数现在会返回一个 Result<(), String> 类型的值
    // 在函数体中，我们不再调用 assert_eq! 宏
    // 而是在测试通过时返回 Ok(())，在失败时返回一个带有 String 的 Err 值
    // 像这样编写返回 Result<T, E> 的测试，就可以在测试函数体中使用问号运算符了
    // 这样可以方便地编写一个测试，该测试在任意一个步骤返回 Err 值时都会执行失败

    // 这样就可以实现链式调用

    // 注意：不要在使用 Result<T, E> 编写的测试上标注 #[should_panic]，在测试运行失败时，我们应当直接返回一个Err值


    // 使用 should_panic 属性
    // 标记了这个属性的测试函数会在代码发生 panic 时顺利通过，而在代码不发生 panic 时执行失败
    // 应用场景就是：有些情况下，我们需要主动触发 panic，而 should_panic 属性就是用来测试某些条件下，会不会如期触发 panic
    // #[test]
    // #[should_panic]
    // fn greater_than_100() {
    //     // 显然输入的 200 不合法，会触发程序 panic，因此断言通过
    //     Guess::new(200);
    // }


    // #[test]
    // #[should_panic]
    // fn greater_than_100_no_panic() {
    //     // new_2 方法中，只有一个判断条件：小于 1，就是只有小于 1 的时候，才会触发程序的 panic，因此，输入 200 不会触发程序的 panic
    //     // 所以这个测试用例失败了，提示信息如下：
    //     // failures:
    //     //
    //     // ---- tests::greater_than_100_no_panic stdout ----
    //     // note: test did not panic as expected
    //     //
    //     // failures:
    //     //     tests::greater_than_100_no_panic
    //
    //     // 并没有像预期一样发生 panic
    //     // Guess::new_2(200);
    // }


    // should_panic 添加 expect 参数
    // 使用 should_panic 进行的测试可能会有些含糊不清，因为它们仅仅能够说明被检查的代码会发生 panic
    // 即便函数中发生 panic 的原因与我们预期的不同，使用 should_panic 进行的测试也会顺利通过
    // 为了让 should_panic 测试更加精确一些，我们可以在 should_panic 属性中添加可选参数 expected 添加自定义的错误提示信息
    // 它会检查 panic 发生时输出的错误提示信息是否包含了指定的文字
    // #[test]
    // #[should_panic(expected = "Guess value must be less than or equal to 100")]
    // fn greater_than_100_panic() {
    //
    //     // 输入的值为 200，因此会触发条件大于 100 情况下的  panic
    //     // 而这个 panic 发生时输出的文本是：Guess value must be less than or equal to 100, got 200.
    //     // 包含了 should_panic 中指定的文本，因此可以通过测试
    //     // 我们也由此可以推断出，是预期的 panic
    //     // 一般来说，expected参数中的内容既取决于panic信息是明确的还是易变的，也取决于测试本身需要准确到何种程度
    //     Guess::new_3(200);
    // }


    // #[test]
    // #[should_panic(expected = "Guess value must be less than or equal to 100")]
    // fn greater_than_100__no_expected_panic() {
    //
    //     // new_4 函数中，交换了小于 1 和大于 100 的条件下触发 panic 的文本
    //     // 当输入为 200 时，会触发大于 100 时的 panic，提示信息为：Guess value must be greater than or equal to 1, got 200.
    //     // 此时，panic 输出的信息与 should_panic 期望的信息不同，因此测试不会通过，失败信息如下：
    //     // failures:
    //     //
    //     // ---- tests::greater_than_100__no_expected_panic stdout ----
    //     // thread 'tests::greater_than_100__no_expected_panic' panicked at 'Guess value must be greater than or equal to 1, got 200.', src\lib.rs:68:13
    //     // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    //     // note: panic did not contain expected string
    //     //       panic message: `"Guess value must be greater than or equal to 1, got 200."`,
    //     //  expected substring: `"Guess value must be less than or equal to 100"`
    //     //
    //     // failures:
    //     //     tests::greater_than_100__no_expected_panic
    //
    //     // panic 输出的信息并没有包含预期的字符串
    //     // 同时打印出了实际的 panic 额信息和预期的信息
    //     Guess::new_4(200);
    // }





    // #[test]
    // fn greeting_contains_name() {
    //     let result = greeting("Carol");
    //     assert!(result.contains("Carol"));
    // }

    // #[test]
    // fn greeting_contains_name_fail() {
    //     let result = greeting_2("Carol");
    //     // assert! 断言失败的时候，仅仅告诉我们代码的某一行发生了断言失败，没有更多的信息
    //     // test tests::greeting_contains_name_fail ... FAILED
    //     //
    //     // failures:
    //     //
    //     // ---- tests::greeting_contains_name_fail stdout ----
    //     // thread 'tests::greeting_contains_name_fail' panicked at 'assertion failed: result.contains(\"Carol\")', src\lib.rs:49:9
    //     // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    //     // assert!(result.contains("Carol"));
    //
    //     //  assert! 还可以自定义的错误提示信息，该信息由一个包含占位符的格式化字符串与具体的要输出到字符串中的值组成：
    //     // assert!(result.contains("Carol"), "Greeting did not contain name, value was `{}`", result);
    //     // 我们添加了自定义的错误提示信息，当断言失败的时候，输出如下：
    //    //     failures:
    //     //
    //     // ---- tests::greeting_contains_name_fail stdout ----
    //     // thread 'tests::greeting_contains_name_fail' panicked at 'Greeting did not contain name, value was `Hello`', src\lib.rs:59:9
    //     // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    //
    //     // 我们自定义的信息被打印出来，还包括了实际的值 result，这有助于我们观察程序真正发生的行为，并迅速定位与预期产生差异的地方
    // }







    // 使用 assert_eq! 和 assert_ne! 宏
    // assert_eq! 和assert_ne!。这两个宏分别用于比较并断言两个参数相等或不相等
    // 在断言失败时，它们还可以自动打印出两个参数的值，从而方便我们观察测试失败的原因
    // 相反，使用assert! 宏则只能得知==判断表达式失败的事实，而无法知晓被用于比较的值

    // #[test]
    // fn it_adds_two() {
    //     assert_eq!(4, add_two(2));
    // }

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
    
    // #[test]
    // fn not_equal() {
    //     assert_ne!(2, add_two(2))
    // }
    //
    // #[test]
    // fn larger_can_hold_smaller() {
    //     let larger = Rectangle { length: 8, width: 7 };
    //     let smaller = Rectangle { length: 5, width: 1 };
    //
    //     // can_hold 方法返回 布尔值，因此可以作为 assert! 这个宏的参数
    //     // can_hold 为 true，表示当前的长方形可以容纳作为参数的长方形
    //     // 如果通过测试，表示我们上面的结论是正确的
    //     assert!(larger.can_hold(&smaller));
    // }

    // #[test]
    // fn smaller_cannot_hold_larger() {
    //     let larger = Rectangle { length: 8, width: 7 };
    //     let smaller = Rectangle { length: 5, width: 1 };
    //
    //     // 这个测试用例是用来测试 smaller 这个长方形不能装下larger 这个长方形的
    //     // 所以 can_hold 方法返回 false，而为了 assert! 这个宏，我们需要对结果取反：!
    //     assert!(!smaller.can_hold(&larger));
    // }

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
    // #[test]
    // fn exploration() {
    //     let result = add(2, 2);
    //     // assert_eq! 是一个断言的宏，用于判断 某个结果是不是与指定的值相等
    //     assert_eq!(result, 4);
    // }
    

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


