# Cargo 中的工作空间 - workspace

## 1. 参考资料

1. [Cargo Book - Workspaces](https://doc.rust-lang.org/stable/cargo/reference/workspaces.html#the-workspace-section)

## 1. 工作空间

1. 工作空间（workspace）是由共用同一个 Cargo.lock 和输出目录的一系列包所组成的。

2. 工作空间的作用：在一个空间下或者说是一个包下组织管理多个代码包，这些代码包被称为工作空间成员（workspace member）。

3. 工作空间的特点：
   - 通用命令对工作空间的所有成员起作用，如：`cargo check --workspace`
   - Common commands can run across all workspace members, like `cargo check --workspace`.
   - 所有的代码包共享一个在根路径下的通用的 Cargo.lock 文件。
   - All packages share a common Cargo.lock file which resides in the workspace root.
   - 所有的代码包共享一个通用的输出目录，这个目录默认是根路径下的 target。
   - All packages share a common output directory, which defaults to a directory named target in the workspace root.
   - 共享包的元数据（metadata），如 `workspace.package`。
   - Sharing package metadata, like with workspace.package.
   - Cargo.toml 中的 `[patch]`、`[replace]` and `[profile.*]` 部分仅仅能被根 manifest 识别，并且会忽略代码包中的 manifests。
   - The [patch], [replace] and [profile.*] sections in Cargo.toml are only recognized in the root manifest, and ignored in member crates’ manifests.

## 2. 创建工作空间

 1. 组织工作空间的方法有许多种，让我们先从最常见的着手。这个工作空间最终包含一个二进制包和两个代码包（库 crate），二进制包依赖于另外两个代码包来实现自己的主要功能。其中一个代码包会提供 add_one 函
数，而另一个代码包则会提供 add_two 函数。这3个包将会共处于同一个工作空间中。

### 1. 创建工作空间

1. 执行下面的命令：
```
   mkdir add
   cd add
```

2. 创建一个工作空间，名称为 add。

3. 在 add 目录下（根路径下），新建一个 Cargo.toml 文件，并写入如下内容：
```
[workspace]
members = [
    "adder",
]

```
4. Cargo.toml 中的 [workspace] 部分用来定义一个工作空间，可以配置下面这些字段：
   - resolver：设置要使用的依赖项解析器
   - members：包含在当前工作空间的代码包
   - exclude：不包含在当前工作空间的代码包
   - default-members：当没有指定的包被选中的时候用来操作的代码包
   - package：代码包中继承的关键字
   - dependencies：代码包依赖中继承的关键字
   - metadata：给外部工具的其他设置

5. `members` 中，指定了工作空间中包含的代码包。其值为数组，可以指定多个代码包。

### 2. 创建 adder 二进制包

1. 在 add 目录下，创建 adder 二进制包：
```
cargo new adder
```
2. 这样项目中就有了一个 adder 二进制包，与我们在 根目录下的 Cargo.toml 中的 `[workspace]` 中 member 字段指定的一样。

3. 在 add 目录下执行 `cargo build` 命令，则会在 add 目录下生成 target 目录，用来存放所有成员的编译产出物，相对应地，adder 包也就没有了自己独立的 target 目录。即使我们进入 adder 目录中运行`cargo build`，编译产出物依然会输出到 `add/target` 而不是 `add/adder/target` 中。Cargo 之所以会将不同的 target 目录集中到一处是因为工作空间中的包往往是互相依赖的。如果每个包都有自己的target 目录，那么它们就不得不在执行各自的构建过程中反复编译工作空间下的其余包。而通过共享一个 target 目录，不同的包就可以避免这些不必要的重复编译过程。

4. 现在 add 目录下整体的文件结构如下所示：
```
├── Cargo.lock
├── Cargo.toml
├── adder
│ ├── Cargo.toml
│ └── src
│ └── main.rs
└── target
```
### 3. 创建第二个代码包 - add_one

1. 打开根目录下的 `Cargo.toml` 文件，并向 members 列表中添加add_one 路径：
```toml
[workspace]
members = [
    "adder",
    "add-one",
]
```
2. 接着生成一个名为 add_one 的新代码包：`cargo new add-one --lib`

3. 此时，add 目录下应该有如下所示的目录和文件：
```
├── Cargo.lock
├── Cargo.toml
├── add-one
│ ├── Cargo.toml
│ └── src
│ └── lib.rs
├── adder
│ ├── Cargo.toml
│ └── src
│ └── main.rs
└── target
```
4. 在 `add-one/src/lib.rs` 文件中添加一个 add_one 函数：
```rust
pub fn add_one(x: i32) -> i32 {
    x+ 1
}
```

### 4. 引入其他代码包作为依赖

1. 创建好库 crate 以后，我们可以让二进制包 adder 依赖于库 crate 
`add_one`。

2. 我们需要在 `adder` 目录下的 `Cargo.toml` 中添加 add_one的路径作为依赖：
```toml
[dependencies]
add_one = { path = "../add_one" }
```
3. 由于Cargo不会主动去假设工作空间中的包会彼此依赖，所以我们
必须要显式地指明包与包之间的依赖关系。

4. 在 adder 包中使用来自 add-one 包的 add_one 函数。打开 `adder/src/main.rs` 文件，并在文件顶部使用 use 语句将新的 `add_one` 包引入作用域。随后修改 main 函数来调用 `add_one `函数，示例如下：
```rust
// rust 会自动的将库 crate 名称中的连字符（-）转换为下划线（_）
// 因此，即使我们将库 crate 命名为 add-one，引入是也必须是 add_one
use add_one;

fn main() {
    let num = 5;
    println!("Hello, world! {} plus one is {}!", num, add_one::add_one(num));

}
```
5. 注意：rust 会自动的将库 crate（代码包）名称中的连字符（-）转换为下划线（_）。因此，即使我们将库 crate 命名为 `add-one`，引入是也必须是 `add_one`。

6. 在 add 根目录下运行 cargo build 来构建整个工作空间：
```
cargo build
```
7. 为了在 add 根目录下运行二进制包 adder，我们需要在调用 `cargo run` 时通过 `-p` 参数来指定需要运行的包名：
```
cargo run -p adder
```
8. 输出为：
```
Compiling adder v0.1.0 (/Users/bytedance/myWork/myFrontEnd/LearningRust/cargo-and-crate.io/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/adder`
Hello, world! 5 plus one is 6!
```
9. 上面的命令运行了 `adder/src/main.rs` 中的代码，而这段代码则依赖了 `add-one` 包。

### 5. 引入外部依赖

1. **注意**：整个工作空间只在根目录下有一个 `Cargo.lock` 文
件，而不是在每个包的目录下都有一个 `Cargo.lock` 文件。这一规则确保了所有的内部包都会使用完全相同的依赖版本。

2. 假设我们将 rand 包同时添加到了 `adder/Cargo.toml` 与`add-one/Cargo.toml` 文件中，那么 Cargo 会将这两个依赖解析为同一版本的 rand 包，并将此信息记录在唯一的 `Cargo.lock` 文件中。确保工作空间中所有的包使用相同的依赖。

3. 使用相同的依赖意味着这些包将会是彼此兼容的。让我们在 `add_one/Cargo.toml` 文件的 `[dependencies]` 区域中加入rand 包，以便可以在 `add_one` 包中使用 rand 包内的功能：
```toml
[dependencies]
rand = "0.3.14"
```
4. 接着在 `add-one/src/lib.rs` 文件中添加 `use rand;`，如下所示：
```rust

use rand;

pub fn add_one(x: i32) -> i32 {
     x + 1
}
```
5. 并在 add 目录下运行 `cargo build` 来构建整个工作空间。此时，Cargo就会引入并编译 rand 包。

6. **注意**：根目录下的 `Cargo.lock `文件包含了 add_one 依赖于 rand 的记录。但需要注意的是，虽然当前的工作空间已经引用了rand，但工作空间内其余的包依然不能直接使用它，除非我们将 rand 添加到这些包对应的 `Cargo.toml` 中去。如果我们没有在 `Cargo.toml` 中的 `[dependencies]` 中指定 rand 依赖，但是在二进制包或者代码包中引用 rand，就会报错。

### 6. 为工作空间增加测试

1. 让我们来为 add_one 包中的 add_one 函数添加一个测试：
```rust
   pub fn add_one(x: i32) -> i32 {
    x + 1
} 

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
    assert_eq!(3, add_one(2));
    }
}
```
2. 在 add 根目录下执行 `cargo test` 命令，输出如下：
```
running 1 test
test test::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/add_one-06e4a15eb88380a5)
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
   
   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```
3. 第一段输出表明 add_one 包中的 it_works 测试通过了。第二段输出表明指令没有在 adder 包中发现可用的测试，第三段输出则表明指令没有在 add_one 包中发现可用的文档测试。

4. 在这样的结构中调用 `cargo test` 会一次性执行工作空间中所有包的测试。

5. 我们同样可以在工作空间根目录下，使用参数 `-p` 及指定的包名称
来运行某一个特定包的测试：
```
cargo test -p add_one
```
6. 输出如下：
```
running 1 test
test test::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/add_one-06e4a15eb88380a5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```
7. 当哦们想要将工作空间中的各个包发布到 crates.io 上时，你、必须要将它们分别发布。`cargo publish` 命令并没有提供类似于 `--all` 或 `-p` 之类的标记，你必须手动切换到每个包的目录，并对每个包分别执行 `cargo publish` 来完成发布任务。
