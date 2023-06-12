// 在 trait 中使用 async
//
// 在目前版本中，我们还无法在 trait 中定义 async fn 函数
// 不过我们也不用担心，目前已经有计划在未来移除这个限制了
// https://blog.rust-lang.org/inside-rust/2023/05/03/stabilizing-async-fn-in-trait.html
// 这篇博客说，大约在 1.74 版本可以支持在 trait 中定义 async 函数

// 在 trait 中定义 async 函数，直接编译会报错：
// error[E0706]: functions in traits cannot be declared `async`
//  --> src\main.rs:9:5
//   |
// 9 |     async fn test();
//   |     -----^^^^^^^^^^^
//   |     |
//   |     `async` because of this
//   |
//   = note: `async` trait functions are not currently supported
//   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
//   = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information

//
// trait Test {
//     async fn test();
// }

// 关于 trait 中不能定义 async 函数，可以参考这篇文章：https://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard/



// 编译器给出了提示：使用 async-trait 解决这个问题： https://crates.io/crates/async-trait
// async-trait 提供了一个抽象的宏来保证 trait 中的 async 函数正常工作



// 用法如下，示例来自 async-trait 的官方说明：
use async_trait::async_trait;

#[async_trait]
trait Advertisement {
    async fn run(&self);
}

struct Modal;

#[async_trait]
impl Advertisement for Modal {
    async fn run(&self) {
        self.render_fullscreen().await;
        for _ in 0..4u16 {
            remind_user_to_join_mailing_list().await;
        }
        self.hide_for_now().await;
    }
}

struct AutoplayingVideo {
    media_url: String,
}

#[async_trait]
impl Advertisement for AutoplayingVideo {
    async fn run(&self) {
        let stream = connect(&self.media_url).await;
        stream.play().await;

        // 用视频说服用户加入我们的邮件列表
        Modal.run().await;
    }
}

fn main() {

}