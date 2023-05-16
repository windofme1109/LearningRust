// 对外只暴露一个 Post，用户能感知到的，只有 Post 实例
pub struct Post {
    // state 表示当前的文章的状态
    // 为了消耗旧的状态，request_review 方法需要获取状态值的所有权
    // 这也正是 Post 的 state 字段引入 Option 的原因：Rust不允许结构体中出现未被填充的值
    // 我们可以通过 Option<T> 的 take 方法来取出 state 字段的 Some 值，并在原来的位置留下一个 None
    // 这样做使我们能够将 state 的值从 Post 中移出来，而不单单只是借用它
    state: Option<Box<dyn State>>,
    // content 表示文章的内容
    content: String
}

impl Post {
    pub fn new() -> Post {
        // 关联函数 new 在创建 Post 实例时把它的 state 字段设置为了持有 Box 的 Some值，而该 Box 则指向了 Draft 结构体的一个实例
        // 这保证了任何创建出来的 Post 实例都会从草稿状态开始，因为 Post 的 state 字段是私有的，所以用户无法采用其他状态来创建Post
        // 同时将 content 字段设置为了新的空字符串
        Post { 
            state: Some(Box::new(Draft {})), 
            content: String::new()
        }
    }

    // 将传入的字符串切片添加到 content 中
    // 我们之所以将这个功能作为方法来实现而不是通过 pub 关键字来直接暴露 content 字段
    // 是因为我们需要控制用户访问 content 字段中的数据时的具体行为
    // 由于我们需要改变 Post 实例，所以 需要接收 self 的可变引用作为参数
    pub fn add_text(&mut self, text: &str) {
        // 调用字符串的 push_str 方法将字符串追加到 content 中
        self.content.push_str(text);
    }



    // 定义一个 request_review 的公共方法
    // 它会接收 self 的可变引用并调用当前 state 的 request_review 方法
    pub fn request_review(&mut self) {

        // take 方法取出 state 的 Some 变体，并进行匹配
        if let Some(s) = self.state.take() {
            // 匹配成功，执行Some 变体 包裹的 trait 对象的 request_review 方法
            // 并将 Some 变体赋值给 state，以改变 state

            // 状态模式的优势：无论 state 的值是什么，Post 的 request_review 方法都不需要发生改变
            // 每个状态都会负责维护自己的运行规则
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    // 获取文章内容
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }

    // 因为我们希望使所有的规则在 State 相关结构体的内部实现，所以我们会调用 state 值的 content 方法
    // 并将 Post 实例本身（也就是 self）作为参数传入，最后将这个方法返回的值作为结果
    // 这段代码调用了 Option 的 as_ref 方法，因为我们需要的只是 Option 中值的引用而不是它的所有权
    // 由于 state 的类型是 Option<Box<dyn State>> ， 所以我们会在调用 as_ref 时得到 Option<&Box<dyn State>>
    // 如果这段代码中没有调用 as_ref，那么就会导致编译时错误，因为我们不能将 state 从函数参数的借用 &self 中移出


    // 我们接着调用了 unwrap 方法。由于 Post 的具体实现保证了方法调用结束时的 state 总会是一个有效的 Some 值
    // 所以我们可以确信调用 unwrap 不会发生 panic
    // 随后，我们又调用了 &Box<dyn State>的content方法
    // 由于解引用转换会依次作用于 & 与 Box，所以我们最终调用的 content 方法来自实现了State trait的具体类型
    // 这意味着我们需要在 State trait 的定义中添加 content 方法，并在这个方法的实现中基于当前状态来决定究竟返回哪些内容

}


// 定义了所有的文章状态共享的行为
trait State {

    // 我们为 State trait 添加了一个request_review方法，
    // 所有实现了这个 trait 的类型都必须实现这个 request_review 方法
    // 值得注意的是，request_review 的参数类型是 self: Box<Self>，而不是 self、&self 或 &mut self
    // 这个语法意味着该方法只能被包裹着当前类型的Box实例调用，它会在调用过程中获取 Box<Self> 的所有权并使旧的状态失效
    // 从而将 Post 的状态值转换为一个新的状态
    fn request_review(self: Box<self>) -> Box<dyn State>;

    // 添加一个 approve 的方法，函数签名和 request_review 类似，作用也类似
    fn approve(self: Box<self>) -> Box<dyn State>;

    // 我们为content方法添加了默认的trait实现，它会返回一个空的
    // 字符串切片❶。这使得我们可以不必在Draft和PendingReview结构体
    // 中重复实现content。Published结构体会覆盖content方法并返回
    // post.content的值❷。
    // 注意，我们需要在这个方法上添加相关的生命周期标注，正如在
    // 第10章讨论过的那样。这个方法的实现需要接收post的引用作为参
    // 数，并返回post中某一部分的引用作为结果，因此，该方法中返回值
    // 的生命周期应该与post参数的生命周期相关
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}


// 文章的草案状态
struct Draft {

}

// 为 Draft 状态实现 State trait
impl State for Draft {
    // 这里的 request_review 方法需要在 Box 中包含一个新的PendingReview结构体实例，这一状态意味着文章正在等待审批
    // 实现了状态的转移
    fn request_review(self: Box<self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    // 为 Draft 实例调用 approve 方法会简单地返回 self 而不会产生任何作用
    // 因为草稿状态下调用 approve 方法没有任何意义（审批通过）
    //
    fn approve(self: Box<self>) -> Box<dyn State> {
        self
    }
}


struct PendingReview {
    
}

impl State for PendingReview {
    // PendingReview 结构体同样实现了 request_review 方法，但它没有执行任何状态转移过程
    // 仅仅是返回了自己。 对于一篇已经处在 PendingReview 状态下的文章，发起审批请求并不会改变该文章的当前状态
    fn request_review(self: Box<self>) -> Box<dyn State> {
        self
    }

    // PendingReview 实例会在调用 approve 时返回一个包裹在 Box 内的 Published 结构体的新实例
    fn approve(self: Box<self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

// 定义 Published 结构体
struct Published {}


impl State for Published {
    // Published 结构体同样实现了 State trait，它的 request_review 和 approve 方法都只会返回它们本身
    // 因为处于 Published 状态下的文章不应当被这些操作改变状态
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
