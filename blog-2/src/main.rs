
// 基于前面的博客的实例，我们对其做一些改造，将状态编码为不同的类型，而不是完全封装状态与转移过程以使外部对其一无所知
// 结果，Rust 的类型检查系统将会通过编译时错误来阻止用户使用无效的状态
// 比如在需要使用已发布文章的场合误用处于草稿状态的文章


use blog_2::Post;

fn main() {
    // println!("Hello, world!");

    // 因为 request_review 和 approve 方法会返回新的实例，而不是修改调用方法的结构体本身
    // 所以我们需要使用变量来绑定来保存返回的新实例
    let mut post = Post::new();

    // 此时 post 是 DraftPost 实例，它没有提供访问 content 的方法，所以像下面这样访问 content，在编译过程中就会报错
    // post.content();
    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
