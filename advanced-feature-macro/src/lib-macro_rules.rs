// 使用 marco_rules! 宏定义一个简版的 vec!


// 注意，macro_rules! 可能再将来被废弃

// #[macro_export] 表示宏会在它所处的包被引入作用域后可用
// 缺少了这个标注的宏则不能被引入作用域
#[macro_export]
// 定义一个宏的名称
macro_rules! vec {
    // 宏的实现代码块
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
             )*
            temp_vec 
        }
    };
 }

//  vec! 代码块中的结构与 match 表达式的结构相似
// 这段实现中存在一个模式为 ($( $x:expr ), *) 的分支，模式后紧跟着的是 => 及对应的代码块
// 这些关联代码会在模式匹配成功时触发
// 由于这是这个宏中仅有的模式，所以整个宏只存在一种有效的匹配方法
// 任何其他模式都会导致编译时错误
// 某些更加复杂的宏会包含多个分支

// 上面这段宏的工作过程（简版）
// 1. 使用了一对圆括号来把整个模式包裹起来，接着是一个 $ 符号，以及另外一对包裹着匹配模式的圆括号，这些被匹配并捕获的值最终会被用于生成替换代码
// $()中的 $x:expr 可以匹配任意的 Rust 表达式，并将其命名为 $x

// 2. $() 之后的逗号意味着一个可能的字面逗号分隔符会出现在捕获代码的后面，而逗号后的 * 则意味着这个模式能够匹配零个或多个 * 之前的东西
// 当我们使用指令vec![1, 2, 3]; 调用这个宏时，$x 模式会分别匹配 3 个表达式: 1、2 及 3

// 3. 让我们把目光转移到该分支对应的代码中: 它会为模式中匹配到的每一个 $() 生成 $()* 中对应的temp_vec.push() 代码
// 这一展开过程会重复零次还是多次，取决于匹配成功的表达式数量，而 $x 则会被每个匹配到的表达式替代
// 使用vec![1, 2, 3];调用宏会生成如下所示的代码来替换调用语句:
// let mut temp_vec = Vec::new();
// temp_vec.push(1);
// temp_vec.push(2);
// temp_vec.push(3);
// temp_vec

// 我们定义的这个宏可以接收任意数量、任意类型的参数，并创建出一个包含指定元素的动态数组

// 从上面的示例中，可以看出，要想使用好 macro_rules! 宏，首先要学好能匹配代码的匹配表达式，这样才能根据需求，找到需要进行替换或者是需要的内容
// 然后在匹配成功以后，生成新的代码