//! # My Crate
//!
//! my_crate是一系列工具的集合
//! 这些工具被用来简化特定的计算操作

//

// 还有一种文档注释形式 ：//!，它可以为包裹当前注释的外层条目（而不是紧随注释之后的条目）添加文档
// 这种文档注释通常被用 在包的根文件（也就是惯例上的 src/lib ）或模块的根文件上，分别为整个包或整个模块提供文档
// 例如，假设我们需要为含有 add_one 函数的 my_crate 包添加描述性文档，那么我们就可以在src/lib 文件的起始处增加以 //! 开头的文档注释

// 在条目内部的文档注释对于描述包或模块特别有用，通过它们来描述外部条目的整体意图可以帮助用户理解包的组织结构

//  三个斜线开头（///）的是文档注释
// 文档注释中使用 Markdown 语法来格式化内容。文档注释被放置在它所说明的条目之前

// 下面的例子中，我们首先描述了 add_one 函数的用途，接着开始 了一段名为 Examples 的区域并提供了一段演示 add_one 函数使用方式的代码
// 我们可以通过运行 cargo doc 命令来基于这段文档注释生成HTML文档
// 这条命令会调用 Rust 内置的 rustdoc 工具在 target/doc 路径下生成HTML文档。
// 为了方便，你也可以调用 cargo doc --open 来生成并自动在浏览器中打开当前的包的文档（以及所有依赖包的文档）
// 打开浏览器后，导航到 add_one 函数，你应该能够看到文档注释被渲染出来的效果

// 除了 Examples 区域以外，文档还可以包含下面这几个区域：

// Panics：指出函数可能引发panic的场景。不想触发 panic 的调用者应当确保自己的代码不会在这些场景下调用该函数

// Errors：当函数返回Result作为结果时，这个区域会指出可能出现的错误，以及造成这些错误的具体原因，它可以帮助调用者在编写代码时为不同的错误采取不同的措施

// Safety：当函数使用了 unsafe 关键字时，这个区域会指出当前函数不安全的原因，以及调用者应当确保的使用前提

/// 将传入的数字 + 1
/// # Examples
/// ```
/// use my_crate::add_one;
/// let x = 5;
/// let my_answer = add_one(x);
/// assert_eq!(my_answer, 6);
/// ```
///
///
pub fn add_one(x: i32) -> i32 {
    x + 1
}


// 执行 cargo test 命令，会在执行时将文档注释中的代码示例作为测试去运行
// 对 add_one 所在的文档执行 cargo test，会输出如下的文档测试内容：
//   Doc-tests my_crate
//
// running 1 test
// test src\libs - add_one (line 20) ... ok
//
// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.21s
