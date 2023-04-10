fn main() {
    // 使用 cargo new 命令创建一个二进制的 package
    // Cargo 会默认将 src/main.rs 作为一个二进制 crate 的根节点，而无需在 Cargo.toml 中指定
    // 同时这个二进制 crate 与 package 的名称相同，即为 my-project
    // 还可以使用 cargo new --lib 命令创建一个库
    // 入口文件是 src/lib.rs，Cargo 也会自动将其视作与 package 同名的库 crate 的根节点
    // Cargo会在构建库和二进制程序时将这些单元包的根节点文件作为参数传递给 rust

    // 最初生成的 package 中只包含源文件 src/main.rs ，这也意味着它只包含一个名为 my-project 的二进制 crate
    // 而假设包中同时存在 src/main.rs 及 src/lib.rs
    // 那么其中就会分别存在一个二进制 crate 与一个库 crate，它们拥有与包相同的名称
    // 我们可以在路径 src/bin 下添加源文件来创建出更多的二进制 crate，这个路径下的每个源文件都会被视作单独的二进制 crate

    // crate 可以将相关的功能分组，并放到同一作用域下，这样便可以使这些功能轻松地在多个项目中共享
    // 将单元包的功能保留在它们自己的作用域中有助于指明某个特定功能来源于哪个单元包，并避免可能的命名冲突。
    println!("Hello, world!");
}
