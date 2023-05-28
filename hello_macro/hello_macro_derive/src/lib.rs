// 定义 derive 宏，其核心是读取并解析 Rust 源码，对源码进行改动或者是从源码中拿到相关信息，并基于相关信息生成新的源码
// 因此，需要我们对编译原理有一定的了解


// 我们声明的 hello_macro_derive 包是一个含有过程宏的包
// 因此还需要使用 syn 和 quote 包中的功能
// 所以我 们应该将它们声明为依赖
// 在 Cargo.toml 中添加如下依赖：
// [dependencies]
// syn = "0.14.4"
// quote = "0.6.3"

// 除了添加上面的依赖
// 还需要再 Cargo.toml 中配置 lib：
// [lib]
// proc-macro = true
// 详见下面的 issue
// https://github.com/rust-lang/book/issues/2027


// 一个用来辅助开发宏的库
// 由标准分发提供，提供了编译器接口可以在代码中读取和操作 Rust 代码
extern crate proc_macro;

// 由 proc_macro 提供的一个类型表示一个抽象的 token 流，或者说是一个 token 树序列
// derive 宏接收的类型
use crate::proc_macro::TokenStream;

// quote包则能够将syn包产生的数据结构重新转换为 Rust 代码
use quote::quote;
// syn包被用来将 Rust 代码从字符串转换为可供我们进一步操作的数据结构体
use syn;

// 当包的用户在某个类型上标注 #[derive(HelloMacro)] 时 ，hello_macro_derive 函数就会被自动调用
// 之所以会发生这样的操作，是因为我们在 hello_macro_derive 函数上标注了 proc_macro_derive
// 并在该属性中指定了可以匹配到 trait 的名称 HelloMacro，这是大多数过程宏都需要遵循的编写惯例


// hello_macro_derive 的返回值也是 TokenStream。
// 返回的TokenStream会被添加到使用这个宏的用户代码中，并使用户在编译自己的包时获得我们提供的额外功能

// proc_macro_derive 用来定义宏的名称
// 当我们使用这个宏的时候，就会自动调用 hello_macro_derive 函数
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {

    // 将Rust代码转换为我们能够进行处理的语法树

    // syn::parse 将 TokenStream 解析为语法树节点

    // 在使用了 syn::parse 函数后调用了 unwrap ， 函数 hello_macro_derive 会在出现解析错误时直接触发 panic
    // 在失败时立即中止程序对于编写过程宏来说是必要的
    // 因为 proc_macro_derive 函数必须遵循过程宏的 API 规范来返回一个 TokenStream 而不是 Result
    // 我们在这里选择了使用 unwrap 来简化示例
    // 但在产品级的代码中，你应该使用 panic! 或 expect 来添加更多用于指明错误原因的信息

    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    // syn::parse 函数返回一个 DeviceInput 结构体作为结果，这个结构体代表了解析后的 Rust 代码
    // 例如字符串 struct Pancakes; 被解析为 DeviceInput 结构体，形式如下：
    // DeriveInput {
    // // --略
    // --
    //      ident: Ident {
    //          ident: "Pancakes",
    //          span: #0 bytes(95..103)
    //      },
    //      data: Struct(
    //          DataStruct {
    //              struct_token: Struct,
    //              fields: Unit,
    //              semi_token: Some(
    //                  Semi
    //              )
    //          }
    //      )
    // }


    // 这个结构体中的字段表明刚刚解析的 Rust 代码是一个单位结构体
    // 它的 ident（identifier，也就是标识符的意思）是 Pancakes
    // 根据官方文档：Ident 结构体中的 ident 字段就是被解析的 struct 名称或者 enum 名称
    //
    // 构造对应的trait实现
    impl_hello_macro(&ast)

}

// 我们将负责解析 TokenStream 的代码提取到了单独的函数 hello_macro_derive 中
// 而 impl_hello_macro 函数则只负责转换语法树
// 这一实践方式会使得编写过程宏更加方便
// 这段代码中的外部函数（也就是本例中的hello_macro_derive）会出现在你能看到的大部分拥有过程宏的包中
// 你仅仅需要根据特定目标来定制内部函数（也就是本例中的impl_hello_macro）的具体实现

// hello_macro_derive 是一个入口
// 真正的代码转换（添加 trait）则是由 impl_hello_macro 实现的

// 实现代码转换
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    // 首先取得了一个 Ident 结构体实例，它包含了被标注类型的名称 ast.ident
    // 前面说过，ident 字段实际上是被标注的结构体或者枚举的名称
    // 所以 name 变量实际上是被标注的结构体或者枚举的名称
    let name = &ast.ident;


    // quote! 宏允许我们定义那些希望返回的 Rust 代码
    // 由于 quote! 宏的执行结果是一种编译器无法直接理解的类型，所以我们还需要将执行结果转换为 TokenStream 类型
    // 我们可以通过调用 into 方法来实现这样的转换，该方法可以将这段中间代码的返回值类型转换为符合要求的 TokenStream 类型
    // quote! 宏还提供了一些非常酷的模板机制：它会将我们输入的 #name 替换为变量 name 中的值
    // 你甚至可以在这个宏的代码块中编写一些类似于常规宏的重复操作
    let gen = quote! {
        // #name 是模板语法，quote! 会将 #name 替换为 name 变量的值
        impl HelloMacro for #name {
            fn hello_macro() {
                // stringify! 宏是内置在 Rust 中的，它接收一个Rust表达式，比如 1 + 2
                // 并在编译时将这个表达式转换成字符串字面量，比如"1 + 2"
                // 这种行为与format! 或 println! 的行为可不相同，后者会计算出表达式的值并将其返回为String
                // 代码中输入的 #name 有可能是一个表达式，而因为我们希望直接打印出这个值的字面量，所以这里使用了 stringify!
                // 使用 stringify! 还可以省去内存分配的开销，因为它在编译时就已经将 #name 转换为了字符串字面量
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };

    // 我们希望编写的过程宏能够为用户标注的类型生成一份 HelloMacro trait 的实现
    // 而这个类型的名称可以通过使用 #name 得到
    // 该 trait 的实现只有一个 hello_macro 函数
    // 它的函数体内会包含我们想要提供的功能：打印出 Hello, Macro! My name is 及被标注类型的名称

    gen.into()
}


