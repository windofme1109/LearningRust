// 集成测试 （integration test）

// 引入刚刚建立的 common 模块
// common 模块中是一些通用的代码
mod common;

use adder;


// 二进制包的集成测试
// 如果我们的项目是一个只有 src/main.rs 文件而没有 src/lib.rs 文件的二进制包
// 那么我们就无法在 tests 目录中创建集成测试，也无法使用 use 语句将 src/main.rs 中定义的函数导入作用域
// 只有代码包（library crate）才可以将函数暴露给其他包来调用，而二进制包只被用于独立执行

// 这就是Rust的二进制项目经常会把逻辑编写在 src/lib.rs 文件中，而只在 src/main.rs 文件中进行简单调用的原因
// 这种组织结构使得集成测试可以将我们的项目视作一个代码包，并能够使用 use 访问包中的核心功能
// 只要我们能够保证核心功能一切正常，src/main.rs 中的少量胶水代码就能够工作，无须进行测试

// 只有 main.rs 的项目（二进制 crate）无法使用集成测试
// 必须是库 crate 才能使用集成测试
// 也就是说，项目中必须有 lib.rs 才能使用集成测试
// 因此推荐我们将逻辑写在 lib.rs 中， 并且能够使用 use 访问包中的核心功能，这样能在集成测试文件中也能访问。而集成测试文件无法访问 main.rs 中的内容
// main.rs 只提供入口和简单调用


#[test]
fn it_adds_two() {
    // 引入 common 模块中定义的 setup 初始化函数
    common::setup();

    assert_eq!(4, adder::add_two(2));
}

// 与单元测试不同，集成测试需要在代码顶部添加语句use adder
// 这是因为 tests 目录下的每一个文件都是一个独立的包，所以我们需要将目标库导入每一个测试包中
// 我们不需要为 tests/integration_test.rs 中的任何代码标注#[cfg(test)]
// Cargo 对 tests 目录进行了特殊处理，它只会在执行 cargo test 命令时编译这个目录下的文件

// 集成测试中，每个文件都是独立的 crate，因此需要手动导入要测试的内容
// 

// 执行 cargo test，输出如下：
// running 3 tests
// test tests::expensive_test ... ignored
// test tests::internal ... ok
// test tests::it_works ... ok

// test result: ok. 2 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

//      Running tests/integration_test.rs (target/debug/deps/integration_test-7c6a06c640998d6d)

// running 1 test
// test it_adds_two ... ok

// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

//    Doc-tests adder

// running 0 tests

// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// 上面的输出中出现了单元测试、集成测试和文档测试这3部分。
// 1. 单元测试部分与我们之前见到的一样：每行输出一个单元测试结果，并在后面给出单元测试的摘要
// 
// 2. 集成测试部分从输出行 Running tests/integration_test.rs (target/debug/deps/integration_test-7c6a06c640998d6d) 开始
// 它会为当前集成测试中的每一个测试函数使用单独一行输出结果，并在 Doc-tests addr 部分开始前给出集成测试的摘要
// 
// 我们添加的单元测试函数越多，在输出中的单元测试部分产生的结果行就越多
// 与之类似，集成测试文件中添加的测试函数越多，对应的集成测试文件部分产生的结果行就越多
// 每一个集成测试文件都会在输出中有自己独立的区域，所以我们在 tests 目录下添加的文件越多，出现的集成测试区域就越多

// 运行指定的集成测试文件

// 在执行 cargo test 命令时使用 --test 并指定集成测试的文件名，可以单独运行某个特定集成测试文件下的所有测试函数：
// cargo test --test integration_test

// 上面这条命令只运行了 tests/integration_test.rs 文件中的测试，输出如下：
// running 1 test
// test it_adds_two ... ok

// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// 只执行了 tests/integration_test.rs 文件中的测试用例