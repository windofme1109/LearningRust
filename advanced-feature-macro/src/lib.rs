// 基于属性创建代码的过程宏

// 第二种形式的宏更像函数(某种形式的过程)一些，所以它们被称为过程宏
// 过程宏会接收并操作输入的 Rust 代码，并生成另外一些 Rust 代码作为结果，这与声明宏根据模式匹配来替换代码的行为有所不同

// 虽然过程宏存在 3 种不同的类型(自定义派生宏、属性宏及函数宏)，但它们都具有非常类似的工作机制
// 当创建过程宏时，宏的定义必须单独放在它们自己的包中，并使用特殊的包类型。这完全是因为技术上的原因，我们希望未来能够消除这种限制
// use proc_macro;
// #[some_attribute]
// pub fn some_name(input: TokenStream) -> TokenStream {

// }

// 上面的代码定义了过程宏的函数
// 其接收一个 TokenStream 作为输入，并产生一个 TokenStream 作为输出
// TokenStream 类型在 proc_macro 包(Rust 自带)中定义，表示一段标记序列
// 这也是过程宏的核心所在: 需要被宏处理的源代码组成了输入的 TokenStream
// 而宏生成的代码则组成了输出的 TokenStream，函数附带的属性决定了我们究竟创建的是哪一种过程宏
// 同一个包中可以拥有多种不同类型的过程宏


// 1. 创建自定义 derive 宏

// 2. 属性宏

// 属性宏与自定义派生宏类似，它们允许你创建新的属性，而不是为 derive 属性生成代码
// 属性宏在某种程度上也更加灵活：derive 只能被用于结构体和枚举，而属性则可以同时被用于其他条目比如函数等

// 下面是一个属性宏的例子，即假设我们拥有一个名为 route 的属性
// 那么就可以在编写 Web 应用框架时为函数添加标记：
// #[route(GET, "/")]
// fn index() {}

// 这个 #[route] 属性是由框架本身作为一个过程宏来定义的，这个宏定义的函数签名如下所示：
// #[proc_macro_attribute]
// pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}

// 上面的代码中出现了两个类型为 TokenStream 的参数。前者是属性本身的内容，也就是本例中 的Get, "/"部分
// 而后者则是这个属性所附着的条目，也就是本例中的fn index() {} 及剩下的函数体
// 除此之外，属性宏与自定义派生宏的工作方式几乎一样：它们都需要创建一个 proc-macro 类型的包并提供生成相应代码的函数

// 其核心也是在 route 函数中操作 Rust 源码


// 3. 函数宏

// 函数宏可以定义出类似于函数调用的宏，但它们远比普通函数更为灵活
// 例如，与 macro_rules! 宏类似，函数宏也能接收未知数量的参数
// 但是，macro_rules! 宏只能使用类似于 match 的语法来进行定义，而函数宏则可以接收一个 TokenStream 作为参数
// 并与另外两种过程宏一样在定义中使用 Rust 代码来操作 TokenStream
//
// 例如，我们可能会这样调用一个名为 sql! 的函数宏：
// let sql = sql!(SELECT * FROM posts WHERE id=1);

// 这个宏会解析圆括号内的 SQL 语句并检验它在语法上的正确性，这一处理过程所做的比 macro_rules! 宏可以完成的任务要复杂得多
// 此处的 sql! 可以被定义为如下所示的样子：
// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream {}
// 这里的定义与自定义派生宏的签名十分类似：我们接收括号内的标记序列作为参数，并返回一段执行相应功能的生成代码

// 其核心也是在 sql 函数中操作 Rust 源码

