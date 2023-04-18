pub fn setup() {
    // 一些测试需要的初始化的代码
}


// 将测试文件拆分为多个不同的模块，将每个集成测试的文件编译成独立的包有助于隔离作用域，并使集成测试环境更加贴近于用户的使用场景。

// 使用模块化的方式管理集成测试文件

// 如果我们直接在 tests 目录下，新建一个 common.rs 文件，并写入如下内容：
// pub fn setup() {
//    // 一些测试需要的初始化的代码
// }

// 即便我们没有在这个文件中包含任何测试函数，也没有在任何地方调用过 setup 函数
// 当我们执行 cargo test 命令的时候，cargo 依旧会把 common 当做测试文件进行执行，输出如下：

//  Running tests/common.rs (target/debug/deps/common-e03a066aeb7c40b6)
// running 0 tests
// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// 让 common 出现在测试输出中，并显示毫无意义的running 0 tests 消息并不是我们需要的
// 我们只是想要在多个集成测试文件之间共享一些代码而已

// 为了避免 common 出现在测试结果中，我们可以创建tests/common/mod.rs
// 而不再创建 tests/common.rs 。这是另一种可以被Rust理解的命名规范（另外一种创建模块的方式）
// 通过采用这种文件命名方式，Rust 就不会再将 common 模块视作一个集成测试文件了

//  当我们将 setup 函数移动至 tests/common/mod.rs 中并删除 tests/common.rs 文件后
// 测试输出中就再也不会出现与 common 相关的区域了
// tests 子目录中的文件不会被视作单独的包进行编译，更不会在测试输出中拥有自己的区域
// 当tests/common/mod.rs 创建完毕后，我们就可以将其视作一个普通的模块并应用到不同的集成测试文件中了

// tests 目录下的 rs 文件，会被当做集成测试文件，而子目录中的文件不会被视为集成测试文件
// 因此可以使用 子目录/mod.rs 的方式，建立模块，从而写一些公共的函数供其他测试文件使用
// 模块的使用方式同在 src 中建立的模块相同