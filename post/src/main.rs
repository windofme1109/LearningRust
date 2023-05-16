// 状态模式(state pattern)是一种面向对象的设计模式
// 它的关键特点是，一个值拥有的内部状态由数个状态对象(state object) 表达而成,而值的行为则随着内部状态的改变而改变
// 这种设计模式会通过状态对象来共享功能

// 相对应地，Rust 使用了结构体与 trait 而不是对象与继承来实现这一特性。
// 每个状态对象都会负责自己的行为并掌控自己转换为其他状态的时机
// 而持有状态对象的值则对状态的不同行为和状态转换的时机一无所知
// 使用状态模式意味着在业务需求发生变化时我们不需要修改持有状态对象的值，或者使用这个值的代码
// 我们只需要更新状态对象的代码或增加一些新的状态对象，就可以改变程序的运转规则


// 举一个博客的例子来实现状态模式，这个例子会采用增量式的开发过程来实现一个用于发布博客的工作流程
// 这个博客最终的工作流程如下:
// 1. 在新建博客文章时生成一份空白的草稿文档。
// 2. 在草稿撰写完毕后，请求对这篇草稿状态的文章进行审批。
// 3. 在文章通过审批后正式对外发布。
// 4. 仅返回并打印成功发布后的文章，而不能意外地发布没有通过审批的文章。

// 除了上面描述的流程，任何其他对文章的修改行为都应当是无效的
// 例如，假设某人试图跳过审批过程来直接发布草稿状态的文章， 那么我们的程序应当阻止这一行为并保持文章的草稿状态


fn main() {
    println!("Hello, world!");
}
