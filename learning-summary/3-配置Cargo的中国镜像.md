# 配置 Cargo 的国内镜像地址

## 1. 参考资料

1. [Rust crates.io 稀疏索引镜像使用帮助](https://mirrors.tuna.tsinghua.edu.cn/help/crates.io-index/)

2. [Rust crates.io 索引镜像使用帮助](https://mirrors.tuna.tsinghua.edu.cn/help/crates.io-index.git/)

3. [配置 Cargo 国内镜像源](https://rust-guide.niqin.com/en-us/4-cargo/4.1-source-replacement.html)

4. [Rust安装并配置国内加速镜像源](https://juejin.cn/post/7133482060307496997)

5. [cargo设置国内源](https://www.cnblogs.com/qumogu/p/14167597.html)

## 2. 全局配置（推荐）

1. Cargo 的默认源是：[crates](https://crates.io/)，其服务器地址在北美，国内访问非常慢。 所以为了加快访问速度，需要将 Cargo 的源设置为国内镜像。

2. 国内的 Cargo 镜像包括中国科学技术大学源、上海交通大学源、清华大学源，以及 rustcc 社区源，这里选取清华大学镜像进行配置。

### 1. 创建 config 文件

1. 找到 `$HOME/.cargo/`，windows 系统的路径是 `C:\Users\你的计算机的用户名\.cargo` 。MacOs 和 Linux 的路径是 `~/.cargo`。

2. 在 `.cargo` 目录下创建名为 `config` 文件，Linux 和 MacOS 可以使用 touch 命令创建。Windows 不能直接创建没有后缀名称的文件，因此需要使用文本编辑器创建。

### 2. 写入配置内容

1. 在 config 文件内写入下述配置内容：
```
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
# 指定镜像
replace-with = '镜像源名' # 如：tuna、sjtu、ustc，或者 rustcc

# 注：以下源配置一个即可，无需全部

# 中国科学技术大学
[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
# >>> 或者 <<<
registry = "git://mirrors.ustc.edu.cn/crates.io-index"

# 上海交通大学
[source.sjtu]
registry = "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index/"

# 清华大学
[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

# rustcc社区
[source.rustcc]
registry = "https://code.aliyun.com/rustcc/crates.io-index.git"
```
2. 其中协议推荐使用 git，但对于 https 和 git 协议，一般各镜像源都支持，并且是可以互换的。如果你所处的环境中不允许使用 git 协议，或者配置 git 协议后不能正常获取和编译 crate，可以换 https 协议再试试。

3. 以清华镜像为例，具体的配置如下：
```
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
# 指定镜像
replace-with = 'tuna'

# 清华大学
[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
```
4. **Cargo 1.68 版本开始支持稀疏索引**：不再需要完整克隆 crates.io-index 仓库，可以加快获取包的速度。如果您的 Cargo 版本大于等于 1.68，可以直接使用而不需要开启 nightly。配置方法如下：
```
[source.crates-io]
replace-with = 'tuna'

[source.tuna]
registry = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/"
```
5. `sparse+` 表示在使用稀疏索引，链接末尾的 / 不能缺少。

### 3. 项目配置（不推荐）

1. 在项目工程结构中，与 `Cargo.toml` 同级目录的 `.cargo` 文件夹下创建 `config` 文件，`config` 文件配置方法和内容与第一种相同。