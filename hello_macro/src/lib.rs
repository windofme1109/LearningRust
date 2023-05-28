// 自定义 derive 宏
// 并在其中定义一个拥有关联函数 hello_macro 的 HelloMacro trait
// 为了避免用户在他们的每一个类型上逐一实现 HelloMacro trait，我们会提供一个能够自动实现 trait 的过程宏
// 这使用户可以在他们的类型上标注 #[derive(HelloMacro)]，进而得到 hello_macro 函数的默认实现
// 即将文本 Hello Macro! My name is TypeName! 中的 TypeName 替换为当前类型的名称后打印出来

// 定义一个给每个标注了 #[derive(HelloMacro)] 的结构体需要实现的 HelloMacro trait
// 这个 trait 需要和 HelloMacro 宏一样，导入到需要使用的地方
pub trait HelloMacro {
    fn hello_macro();
}

// 定义过程宏
// 过程宏需要被单独放置到它们自己的包内，Rust 开发团队也许会在未来去掉这一限制
// 就目前而言，组织主包和宏包的惯例是，对于一个名为 foo 的包，我们会生成一个用于放置自定义派生过程宏的包 foo_derive
// 所以我们需要在 hello_macro 包中创建 hello_macro_derive 包