//! # Art
//!
//! 一个用来建模艺术概念的代码库
//!

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;


pub mod kinds {

    /// RYB颜色模型的三原色
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }
    /// RYB模型的调和色
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}
pub mod utils {
    use crate::kinds::*;
    /// 将两种等量的原色混合生成调和色
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --略
        //     --
        SecondaryColor
    }
}

// 上面的模块代码，对于库的开发者二氧，引用是非常方便的，因为开发者是非常了解每个模块的位置
// 但是对于调用库的用户而言，却不是非常方便，因为调用者不知道模块的组织层次

// 而且 PrimaryColor 类型、SecondaryColor 类型及 mix 函数虽然都有文档注释，但是都没有被显示在首页上
// 我们必须通过点击 kinds 和 utils才能进入相应的页面看到它们
// 如果用户想要在其他的包中依赖这个代码库，那么他们就需要使
// 用 use 语句来将 art 中的条目引入作用域

// 使用 art 包中的模块
// use art::kinds::PrimaryColor;
// use art::kinds::SecondaryColor;
//
// use art::utils::mix;

// 对于用户而言， 须搞清楚 PrimaryColor 位于 kinds 模块，而 mix 则位于 utils 模块
// 但此处的模块结构其实只是为了方便 art 包的开发者进行维护，对用户却没有什么太大的用处
// 这些用于将条目组织到 kinds 模块和 utils 模块的内部结构并不能对用户理解 art 包的使用方式提供任何有用的信息
// 相反地，art 包的模块结构还会使用户产生困惑，因为他们不得不搞清楚功能的实现路径
// 另外，由于用户需要在使用 use 语句时指定完整的模块名称，所以这一结构本身也让代码变得更加冗长了

// 因此，我们可以使用 pub use 语句将需要公开的条目重新导出到顶层结构中，这样用户就能直接导入相关的模块
// 而且生成的文档中，也会显示 pub use 语句，这样导入模块的路径就不一定和模块的实际路径相同，可以直接从包里导入

// pub use self::kinds::PrimaryColor;
// pub use self::kinds::SecondaryColor;
// pub use self::utils::mix;

// 使用 PrimaryColor 枚举和 mix 方法

// use art::PrimaryColor;
// use art::mix;

// 当存在较多嵌套模块时，使用 pub use 将类型重新导出到顶层模块可以显著地改善用户体验



