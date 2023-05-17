

// Post 和 DraftPost 结构体都有一个用来存储文本的私有字段 content
// 由于我们将状态直接编码为了结构体类型，所以这两个结构体不再拥有之前的 state 字段
// 新的 Post 结构体将会代表一篇已发布的文章，它的 content 方法被用来返回内部 content 字段的值

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {

    // 关联函数 new 返回一个 DraftPost 实例，而不再是 Post 实例
    // 由于 content 字段是私有的，且没有任何直接返回 Post 的函数，所以我们暂时无法创建出 Post 实例
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new()
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {

    // 定义一个 add_text 方法，直接向 Draft 中的 content 添加内容
    // 但是请注意，DraftPost 根本就没有定义 content 方法
    // 所以程序能够保证所有文章都从草稿状态开始，并且处于草稿状态的文章无法对外展示自己的内容了
    // 任何绕过这些限制的尝试都会导致编译时错误
    // 也就是说，如果像之前一样，直接调用 content 方法访问草稿内容，就会编译报错
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // request_review 方法创建 PendingReviewPost 实例
    // request_review 接收的参数是 self，而不是 &self，这表示调用 request_review 方法时
    // DraftPost 实例的所有权会转移到函数内，并返回一个新的 PendingReviewPost 实例
    // 实现了实例的转换，也实现了 content 的转移
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content
        }
    }
}


pub struct PendingReviewPost {
    content: String,
}


impl PendingReviewPost {

    // approve 方法创建 Post 实例
    // approve 接收的参数是 self，而不是 &self，这表示调用 approve 方法时
    // PendingReviewPost 实例的所有权会转移到函数内，并返回一个新的 Post 实例
    // 实现了实例的转换，也实现了 content 的转移
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

// DraftPost 中的 request_review 方法和 PendingReviewPost 中的 approve 方法
// 我们不可能在调用 request_review 方法后遗漏任何 DraftPost 实例，调用 approve 方法与此同理，因为调用完相关方法以后，原来的实例就被销毁了

// 尝试读取 PendingReviewPost 的内容同样会导致编译错误，因为它没有定义 content 方法
// 含有 content 方法的 Post 实例只能够通过 PendingReviewPost 的approve 方法来获得
// 而用户只能通过调用 DraftPost 的 request_review 方法来获得 PendingReviewPost 实例