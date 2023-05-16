// 对外只暴露一个 Post，用户能感知到的，只有 Post 实例
pub struct Post {
    // state 表示当前的文章的状态
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
        self.content.push_str(text);
    }
}


// 定义了所有的文章状态共享的行为
trait State {

}


// 文章的草案状态
struct Draft {

}

// 为 Draft 状态实现 State trait
impl State for Draft {
    
}