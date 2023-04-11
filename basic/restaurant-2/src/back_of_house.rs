pub mod serving;
// 在非 lib.rs 中使用下面这种形式定义的模块
// rust 会去以当前文件名为名称的文件夹下寻找同名的 rs 文件
// 即：back_of_house/hosting.rs
// 或者去以当前文件名为名称的文件夹下寻找同名的文件夹下的 mod.rs 文件
// back_of_house/hosting/mod.rs
pub mod hosting;

// back_of_house/cooking/mod.rs
pub mod cooking;