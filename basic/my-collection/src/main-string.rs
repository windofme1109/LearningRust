fn main() {
    // Rust 中，字符串统一使用 utf-8 编码
    // 字符串本身就是基于字节的集合，并通过功能性的方法将字节解析为文本
    // ust在语言核心部分只有一种字符串类型，那就是字符串切片str
    // 它通常以借用的形式 (&str)出现。
    // 字符串切片是一些指向存储 在别处的 UTF-8 编码字符串的引用。
    // 例如，字符串字面量的数据被存储在程序的二进制文件中，而它们本身也是字符串切片的一种
    
    
    // String 是标准库的一部分
    // Rust的标准库中同时包含了其他一系列的字符串类型
    // 比如 OsString、OsStr、CString 及 CStr
    // 某些第三方库甚至还提供了更多用于存储字符串数据的选择
    // 注意到这些名字全都以 String 或 Str 结尾了吗?
    // 这用来表明类型提供的是所有者版本还是借用者版本：String 结尾的是所有者版本，Str 结尾的是借用者版本

    // 使用 String::new 新建一个字符串
    let s = String::new();

    // 使用 to_string 方法将字符串字面量转换为字符串类型
    let data = "initial content";
    // s 这个 String 类型会拥有 nitial content 作为初值
    let s = data.to_string();

    // 使用函数String::from来基于字符串字面量生成 String
    let s2 = String::from("initial content");

    // 任何合法的 utf-8 字符都可以被编码进字符串中
    // let hello = String::from("
    // ");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("
    ");
    // let hello = String::from("
    // ");
    let hello = String::from("こんにちは"); 
    let hello = String::from("안녕하세요"); 
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // String 的大小可以变化，其中的内容也可以修改

    // 使用 push_str 或 push 向字符串中添加内容
    let mut s = String::from("foo");
    // s 中的字符串会被更新为foobar
    // 由于我们并不需要取得参数的所有权
    // 所以这里的 push_str 方法只需要接收一个字符串切片作为参数
    s.push_str("bar");

    let mut  s3 = String::from("foo");
    let s4 = "bar";
    s3.push_str(s4);
    println!("s4 is {}", s4);

    // push 方法接收单个字符作为参数，并将它添加到 String 中，在 Rust 中，使用单引号定义一个字符
    let mut s4 = String::from("lo");
    s4.push('l');


    // 拼接字符串
    // Rust 中，使用 + 或者 format! 宏来拼接字符串
    let s5 = String::from("Hello, ");
    let s6 = String::from("World");
    // 加法操作的第一个字符串是原始的字符串，而第二个及其后面的，都是字符串的引用
    let s7 = s5 + &s6;

    // s7 is Hello, World
    println!("s7 is {}", s7);
    // error[E0382]: borrow of moved value: `s5`
    // println!("s5 is {}", s5);

// s7 变成了 Hello, World
// 值得注意的是，我们在加法操作中仅对 s6 采用了引用，而 s5 在加法操作之后则不再有效。
// 产生这一现象的原因与使用 + 运算符时所调用的方法签名有关
// 当我们使用加法操作实现字符串拼接时，实际上调用的是 add 方法
// 其函数签名是：
// fn add(self, s: &str) -> String {}
// 当然，这与标准库中实际的签名有些许差别:
// 在标准库中，add函数使用了泛型来进行定义。此处展示的 add 函数将泛型替换为了具体的类型
// 这是我们使用 String 值调用 add 时使用的签名

// 在执行加法操作时，&s6 表示这是对字符串 s6 的引用，也就是将字符串引用与字符串相加
// 观察 add 函数的定义，第二个参数 s 的类型正是 &str，因此只能是将 &str 与 String 类型进行拼接
// 因为 s6 是 String 类型，&s6 是 &String 类型，
// 编译器可以自动将 &String 类型的参数强制转换为 &str 类型。当我们调用 add 函数时
// Rust使用了一种被称作解引用强制转换的技术，将&s2转换为了 &s2[..]
// 因为 add 函数的参数 s 只是引用，并不会获得 s 的所有权，所以在 add 函数执行完以后，s6 依旧是一个有效的值

// add 函数签名中的 self 并没有 & 标记，所以 add 函数会取得 self 的所有权
// 这也意味着 s5 会作为 add 的第一个参数传入 add 函数内，而且这是一个所有权的转移，s5 的所有权会被转移到 self 上
// 当 add 函数执行完以后，s5 就会失效。
// 对于 let s7 = s5 + &s6;
// 看起来像是复制两个字符串并创建一个新的字符串，但实际上这条语句会取得 s5 的所有权，
// 再将 s6 中的内容复制到其中，最后再将 s5 的所有权作为结果返回
// 换句话说，它看起来好像进行了很多复制，但实际上并没有，这种实现要比单纯的复制更加高效
// 即所有权转移和引用

    // 使用加法操作实现多个字符串拼接
    let s8 = String::from("tic");
    let s9 = String::from("tac");
    let s10 = String::from("toe");
    let s11 = s8 + "-" + &s9 + "-" + &s10;

println!("s11 is {}", s11);

// 对于拼接多个字符串，使用加法操作还是有点繁琐，而且会转移第一个字符串的所有权
// 因此可以使用 format! 这个宏来拼接字符串

let s8 = String::from("tic");
let s9 = String::from("tac");
let s10 = String::from("toe");
// format! 会将结果包含在一个 String 中返回
// 使用 format! 拼接字符串，形式上要更加易读，并且不会夺取任何参数的所有权
let s12 = format!("{}-{}-{}", s8, s9, s10);
// s12 is tic-tac-toe
println!("s12 is {}", s12);

// 字符串索引
// Rust 不支持直接使用索引访问字符串，例如下面通过索引访问字符串 s13 的第一个元素，就会报错：
// let s13 = String::from("hello");
// error[E0277]: the type `String` cannot be indexed by `{integer}`
// let h = s13[0];

// String 实际上是一个基于 Vec<u8> 的封装类型，其索引对应的每个位置都是一个 u8 类型的值，也就是一个字节大小
// 这仅仅是底层存储，但是要记住，Rust 中的字符是 UTF-8 编码的，也就是说，字符串的每个字符使用 UTF-8 进行编码
// 那就带来一个问题，UTF-8 是变长编码，就是字符可能占据一个字节，也可能占据两个甚至更多的字节
// ascii 格式的字符就是一个字节，而汉字占据两个或者三个字节
// 举例如下：
let len1 = String::from("Здравствуйте").len();  // 西里尔字母
let len2 = String::from("Hola").len();
let len3 = String::from("你好").len();
// len1 24, len2 4, len3 6
println!("len1 {}, len2 {}, len3 {}", len1, len2, len3);

// let len1 = String::from("Здравствуйте").len();
// Rust 返回的结果是 24，这就是使用 UTF-8 编码来存储"Здравствуйте"所需要的字节数
// 因为这个字符串中的每个 Unicode 标量值都需要占据 2 字节
// let len3 = String::from("你好").len();
// 使用 UTF-8 编码来存储”你好“需要 6 个字节
// 而对字符串的索引，实际上是对 Vec 中字节的访问，而一个字节并不能存储一个完整的 UTF-8 字符
// 对字符串中字节的索引并不总是能对应到一个有效的Unicode标量值

// let hello = "Здравствуйте";
// let answer = &hello[0];
// 这段代码中的 answer 值会是多少呢?
// 它应该是首字母 З 吗?
// 当使 用 UTF-8 编码时，З 依次使用了 208、151 两字节空间，所以这里的 answer应该是208吧
// 但208本身却又不是一个合法的字符。请求字符串中首字母的用户可不会希望获得一个 208 的返回值，
// 可这又偏偏是Rust在索引0处取到的唯一字节数据。
// 用户想要的结果通常不会是一个字节值，即便这个字符串只由拉丁字母组成
// 如果我们将&"hello"[0] 视作合法的代码，那么它会返回一个字节值104，而不是h
// 为了避免 返回意想不到的值，以及出现在运行时才会暴露的错误，Rust会直接拒绝编译这段代码，在开发阶段提前杜绝可能的误解


// 字节：存储字符或者是字符串的存储单位，一个 UTF-8 编码的字符占据可以占据多个字节
// Unicode标量值：一个有效的 Unicode 编码的内容，即 Rust 中的 char 类型
// 字形簇：由若干 Unicode 标量值组成的内容，可以理解为是一个有意义的内容，
//        比如说有些语言的注音符号，单独拿出来，只能算是合法的 char 类型，但是没有任何意义，必须和其他字母组合起来，才有意义，这样的组合就是字形簇

// 
// Rust不允许我们通过索引来获得 String 中的字符还有最后一个原因
// 那就是索引操作的复杂度往往会被预期为常数时间( (1))
// 但 在String中，我们无法保障这种做法的性能
// 因为Rust必须要遍历从头至索引位置的整个内容来确定究竟有多少合法的字符存在
// 


    // 可以使用切片访问字符串
    // 既然使用索引访问单个字节不被运行，那么可以使用切片访问连续的字节
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    // s is Зд
    println!("s is {}", s);
    // 使用切片访问字符串，需要注意的是切片的范围
    // 这个范围内的字节一定能组成一个 Unicode 标量
    // 因此要小心谨慎地使用范围语法创建字符串切片，因为错误的指 令会导致程序崩溃

    // 遍历字符串

    // 使用 chars 方法进行遍历
    // chars 方法返回一个迭代器，迭代器的元素是 char 形式的字符串切片
    for c in "Здравст".chars() {
        // char is З
        // char is д
        // char is р
        // char is а
        // char is в
        // char is с
        // char is т
        println!("char is {}", c);
    }

    // 使用 bytes 方法会依次返回每个元素字节（十进制形式）
    for b in "Здравст".bytes() {
        // byte is 208
        // byte is 151
        // byte is 208
        // byte is 180
        // byte is 209
        // byte is 128
        // byte is 208
        // byte is 176
        // byte is 208
        // byte is 178
        // byte is 209
        // byte is 129
        // byte is 209
        // byte is 130
        println!("byte is {}", b);
    }

    // 从字符串中获取字形簇相对复杂一些，所以标准库中也没有提供这个功能
    // 如果有这方面的需求，那么可以在 crates.io 上获取相关 的开源库

}