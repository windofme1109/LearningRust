# Rust 中的模块系统

## 1. 参考资料

1. [Rust 包与模块系统 --- Packge Crate 与 Module 解惑](https://www.jianshu.com/p/9f5776c8e6bb)

2. [Rust模块系统的清晰解释](https://www.jdon.com/62794.html)

3. [Rust模块系统详解](https://zhuanlan.zhihu.com/p/487716085)

4. [【翻译】关于Rust模块系统的清晰解释](https://zhuanlan.zhihu.com/p/164556350)

5. [Rust 的包管理机制](https://zyy.rs/post/rust-package-management)

6. [对比学习 Go 和 Rust：模块化编程](https://chingli.com/coding/go-rust/10-modular/)

7. [了解下Rust 模块使用方式](https://juejin.cn/post/7070481262749679653)

## 2. mod 关键字

## 3. super 与 self

## 4. 可见性

## 5. 模块组织方式

### 1. 将所有的模块定义在 `src/lib.rs` 中

### 2. 单文件方式声明
1. 将模块定义在 `src` 下，文件名即为 module 名，然后在 `src/lib.rs` 中声明

### 3. 目录 + `mod.rs` 方式声明
1. 将模块定义在 `src` 下的不同文件目录内，对应的文件目录即为 module 名称，每个文件目录下必须有一个 mod.rs

### 4. 目录 + 目录同名 rs 文件方式声明

## 6. 引入外部模块的方式

1. 在上述的模块组织方式的说明中，方式 3 和 方式 4，都需要新建一个目录，那么如果想要引用目录外部的模块，该怎么办呢？

2. 项目结构：
```bash
 src
 ├── service_log.rs
 ├── my_content
     ├── custom_content.rs
     ├── mod.rs
 ├── my_print
     ├── custom_print.rs
     ├── user_print.rs
 ├── my_user
     ├── custom_user.rs
     ├── mod.rs
     ├── private_user.rs

```
3. `my_content` 下的各个文件的内容：
```rs
// custom_content.rs
pub fn my_content() {
    println!("hello my content");
}

// mod.rs
pub mod custom_content;
```
3. `my_print` 下的各个文件的内容：
```rs
// custom_print.rs
pub fn custom_print() {
    println!("Hello world");
}

// user_print.rs

```
4. `service_log.rs` 内容：
```rs
pub mod log {
    pub fn my_log() {
        
    }
}
```

5. 如果想在 `my_user/custom_user.rs` 下，引用与 `my_user` 同级的其他模块，有两种方式：
   - 绝对路径
   - path 属性

### 1. 使用 crate::... 方式的绝对路径

1. 在 `my_user/custom_user.rs` 直接指定某个模块的绝对路径，即可使用外部模块：
```rs
use crate::my_print::custom_print::custom_print;

pub fn custom_user() {
    custom_print();
}

```
2. 在某个模块内，使用 `pub mod xxxx` 声明某个外部模块，那么 rust 就会去在同级目录下，去寻找 `xxxx.rs` 或者是 `xxxx/mod.rs`
3. 举个例子，在 `my_user/custom_user.rs` 声明一个模块 my_log： 
```
// my_user/custom_user.rs
pub mod my_log;
```
rust 并不会从根路径下开始查找，而是在 `custom_user.rs` 同一级别的目录下寻找：`my_log.rs` 或者是 `my_log/mod.rs`。

4. 希望的目录结构如下：
```bash
 src
 ├── my_user
     ├── custom_user.rs
     ├── mod.rs
     ├── my_log.rs
```
或者是：
```
 src
 ├── my_user
     ├── custom_user.rs
     ├── mod.rs
     ├── my_log
         ├── mod.rs
```
5. 显然不能通过 `pub mod xxxx` 的方式声明外部模块，而是要使用绝对路径的方式引入。

6. 此时的 crate 表示 lib（module_demo）节点。

### 2. 使用外部属性 path 指定外部的模块路径

1. 使用 `path` 属性，结合 `pub mod xxxx`，即可声明外部模块。
```rs
// my_user/custom_user.rs

#[path ="../service_log.rs"]
pub mod service_log;

#[path="../my_content/mod.rs"]
pub mod custom_content;

pub fn my_user() {

    service_log::log::my_log();
    custom_content::my_content();
    // private_user::private_user()
}


```

2. 使用 `path` 属性指定外部模块路径有以下几种方式：
   -  直接指定 rs 文件，即 rs 文件作为模块。
   - 指定某个目录下的 mod.rs 文件。

3. `path` 一般使用相对路径，其相对的起点是当前的文件所在的目录，本例中，是 `custom_user.rs` 所在的目录。


4. 关于 `path` 属性的详解：[path attribute](https://doc.rust-lang.org/reference/items/modules.html#the-path-attribute)

## 7. bin 目录下引用外部模块方式

1. 在 bin 目录下，引用外部模块的方式也有两种：
   - 绝对路径
   - path 属性

2. 整体的目录结构如下所示：
```
 src
 ├── bin
     ├── service.rs
 ├── foo2
     ├── foo_mod1.rs
     ├── foo_mod2.rs
     ├── mod.rs
 ├── my_content
     ├── custom_content.rs
     ├── mod.rs
```

### 1. 绝对路径

1. 使用绝对路径的方式引用 `foo2` 下定义的模块：
```rs
use module_demo::foo2::foo_mod1;

fn main() {

    foo_mod1::hello_foo_mod1();

    println!("hello service");
}

```

2. 注意，绝对路径的起点是根节点，即 crate 名，这里是 `module_demo`。


### 2. path 属性

1. 使用 `path` 属性，结合 `pub mod xxxx`，即可声明外部模块。
```rs

#[path ="../my_content/mod.rs"]
mod my_content;


fn main() {

    my_content::custom_content::my_content();
    println!("hello service");
}
```

2. 使用 `path` 属性指定外部模块路径有以下几种方式：
   -  直接指定 rs 文件，即 rs 文件作为模块。
   - 指定某个目录下的 `mod.rs` 文件。

3. `path` 一般使用相对路径，其相对的起点是当前的文件所在的目录，本例中，是 `service.rs` 所在的目录。